//! EPUB 解析
//!
//! EPUB 本质是一个 ZIP 包，结构如下（简化）：
//! - `mimetype` 文件：内容固定 `application/epub+zip`
//! - `META-INF/container.xml`：声明主 OPF 文件路径
//! - `<package>.opf`：元数据 + manifest（资源列表）+ spine（阅读顺序）
//! - 各章节 XHTML（按 spine 顺序）
//!
//! 本模块使用 `zip` + `quick-xml` 实现：
//! 1. 解析 container.xml → 拿到 OPF 路径
//! 2. 解析 OPF → 书名 / 作者 / 简介 / 章节顺序 / 封面 id
//! 3. 按 spine 顺序解析每个 XHTML → 章节标题与纯文本内容
//! 4. 提取封面到本地缓存目录（`std::env::temp_dir()/stealthreader/covers/`）

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use zip::ZipArchive;

use super::{ParsedBook, ParsedChapter};

// ---------------------------------------------------------------------------
// 公共入口
// ---------------------------------------------------------------------------

pub fn parse(file_path: &str) -> Result<ParsedBook, String> {
    let file = File::open(file_path).map_err(|e| format!("打开 EPUB 失败: {e}"))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("读取 ZIP 失败: {e}"))?;

    // 一次性构建「规范化名字 → 实际条目名」索引，后续按 O(1) 查找，
    // 避免每个章节都遍历整个 ZIP（否则大 EPUB 会退化成 O(n²) 卡死）。
    let name_index = build_name_index(&mut archive);

    // 1) 解析 container.xml → 拿到 OPF 在 zip 中的相对路径
    let container_xml = read_zip_entry(&mut archive, &name_index, "META-INF/container.xml")?;
    let opf_path = parse_container(&container_xml)?;

    // 2) 解析 OPF
    let opf_xml = read_zip_entry(&mut archive, &name_index, &opf_path)?;
    let opf_info = parse_opf(&opf_xml)?;

    // 2.5) 解析 EPUB2 目录 toc.ncx，得到「规范化 href → 权威章节标题」映射
    // （很多 EPUB 的 XHTML `<title>` 是空或统一书名，toc.ncx 才是真正的章节标题）
    let toc_map = match &opf_info.ncx_href {
        Some(ncx_href) => match read_zip_entry(&mut archive, &name_index, ncx_href) {
            Ok(ncx_xml) => parse_toc(&ncx_xml),
            Err(_) => HashMap::new(),
        },
        None => HashMap::new(),
    };

    // 3) 书名兜底：若 OPF 没给，用文件名 stem
    let fallback_title = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名")
        .to_string();
    let title = {
        let t = opf_info.title.trim();
        if t.is_empty() { fallback_title } else { t.to_string() }
    };

    // 4) 按 spine 顺序解析章节
    let chapters = extract_chapters(&mut archive, &name_index, &toc_map, &opf_info)?;

    if chapters.is_empty() {
        return Err("未能从 EPUB 中提取到任何章节内容".to_string());
    }

    let total_chars: i32 = chapters.iter().map(|c| c.total_chars).sum();
    let total_chapters = chapters.len() as i32;

    // 5) 抽取封面（失败不应影响导入）
    let cover_path = extract_cover(&mut archive, &name_index, &opf_info).ok().flatten();

    Ok(ParsedBook {
        title,
        author: opf_info.author,
        introduction: opf_info.description,
        total_chapters,
        total_chars,
        chapters,
        cover_path,
    })
}

// ---------------------------------------------------------------------------
// 数据结构
// ---------------------------------------------------------------------------

#[derive(Default)]
struct OpfInfo {
    title: String,
    author: String,
    description: String,
    /// EPUB 2 风格的 `<meta name="cover" content="id">`
    cover_id: Option<String>,
    /// manifest：id → 资源
    items: HashMap<String, ManifestItem>,
    /// spine：顺序的 idref 列表
    spine: Vec<String>,
    /// EPUB2 目录文件 toc.ncx 的 href（相对 OPF 目录），存在则用于提取权威章节标题
    ncx_href: Option<String>,
}

#[derive(Clone)]
struct ManifestItem {
    href: String,
    media_type: String,
    properties: String,
}

struct ChapterFile {
    title: Option<String>,
    content: String,
}

// ---------------------------------------------------------------------------
// container.xml → OPF 路径
// ---------------------------------------------------------------------------

fn parse_container(xml: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"rootfile" {
                    for attr in e.attributes().with_checks(false).flatten() {
                        if attr.key.as_ref() == b"full-path" {
                            let v = String::from_utf8_lossy(attr.value.as_ref()).into_owned();
                            if !v.is_empty() {
                                return Ok(v);
                            }
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    Err("container.xml 中找不到 rootfile/full-path".to_string())
}

// ---------------------------------------------------------------------------
// OPF 解析
// ---------------------------------------------------------------------------

fn parse_opf(xml: &str) -> Result<OpfInfo, String> {
    let mut info = OpfInfo::default();
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut in_metadata = false;
    let mut in_manifest = false;
    let mut in_spine = false;
    // 当前正在收集文本的元素名（dc:title / dc:creator / dc:description）
    let mut capturing: Option<Vec<u8>> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().as_ref().to_vec();
                match name.as_slice() {
                    b"metadata" => in_metadata = true,
                    b"manifest" => in_manifest = true,
                    b"spine" => in_spine = true,
                    b"dc:title" if in_metadata => capturing = Some(b"dc:title".to_vec()),
                    b"dc:creator" if in_metadata => capturing = Some(b"dc:creator".to_vec()),
                    b"dc:description" if in_metadata => capturing = Some(b"dc:description".to_vec()),
                    _ => {}
                }
            }
            Ok(Event::Empty(e)) => {
                let name = e.name().as_ref().to_vec();
                if in_manifest && name == b"item" {
                    let mut id = String::new();
                    let mut href = String::new();
                    let mut media_type = String::new();
                    let mut properties = String::new();
                    for attr in e.attributes().with_checks(false).flatten() {
                        match attr.key.as_ref() {
                            b"id" => id = String::from_utf8_lossy(&attr.value).into_owned(),
                            b"href" => href = String::from_utf8_lossy(&attr.value).into_owned(),
                            b"media-type" => media_type = String::from_utf8_lossy(&attr.value).into_owned(),
                            b"properties" => properties = String::from_utf8_lossy(&attr.value).into_owned(),
                            _ => {}
                        }
                    }
                    if !id.is_empty() && !href.is_empty() {
                        if media_type.eq_ignore_ascii_case("application/x-dtbncx+xml") {
                            info.ncx_href = Some(href.clone());
                        }
                        info.items.insert(
                            id,
                            ManifestItem { href, media_type, properties },
                        );
                    }
                } else if in_spine && name == b"itemref" {
                    for attr in e.attributes().with_checks(false).flatten() {
                        if attr.key.as_ref() == b"idref" {
                            let idref = String::from_utf8_lossy(&attr.value).into_owned();
                            if !idref.is_empty() {
                                info.spine.push(idref);
                            }
                            break;
                        }
                    }
                } else if in_metadata && name == b"meta" {
                    let mut name_attr = String::new();
                    let mut content_attr = String::new();
                    for attr in e.attributes().with_checks(false).flatten() {
                        match attr.key.as_ref() {
                            b"name" => name_attr = String::from_utf8_lossy(&attr.value).into_owned(),
                            b"content" => content_attr = String::from_utf8_lossy(&attr.value).into_owned(),
                            _ => {}
                        }
                    }
                    if name_attr.eq_ignore_ascii_case("cover") && !content_attr.is_empty() {
                        info.cover_id = Some(content_attr);
                    }
                }
            }
            Ok(Event::Text(t)) => {
                if let Some(tag) = &capturing {
                    let text = t.unescape().map_err(|e| e.to_string())?.into_owned();
                    let text = text.trim();
                    if !text.is_empty() {
                        match tag.as_slice() {
                            b"dc:title" => info.title = text.to_string(),
                            b"dc:creator" => info.author = text.to_string(),
                            b"dc:description" => info.description = text.to_string(),
                            _ => {}
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = e.name().as_ref().to_vec();
                if name.as_slice() == b"metadata" {
                    in_metadata = false;
                } else if name.as_slice() == b"manifest" {
                    in_manifest = false;
                } else if name.as_slice() == b"spine" {
                    in_spine = false;
                } else if Some(name.as_slice()) == capturing.as_deref() {
                    capturing = None;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(info)
}

// ---------------------------------------------------------------------------
// toc.ncx 解析（EPUB2 目录）
// ---------------------------------------------------------------------------

/// 解析 EPUB2 的 toc.ncx，返回「规范化 src → 章节标题」映射。
/// navMap 里每个 navPoint 的 navLabel/text 是标题，content/src 是相对 OPF 目录的文件路径。
fn parse_toc(xml: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut in_nav_label = false;
    let mut current_label = String::new();
    let mut current_src: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().as_ref().to_vec();
                match name.as_slice() {
                    b"navLabel" => in_nav_label = true,
                    b"content" => {
                        for attr in e.attributes().with_checks(false).flatten() {
                            if attr.key.as_ref() == b"src" {
                                current_src = Some(String::from_utf8_lossy(&attr.value).into_owned());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"content" {
                    for attr in e.attributes().with_checks(false).flatten() {
                        if attr.key.as_ref() == b"src" {
                            current_src = Some(String::from_utf8_lossy(&attr.value).into_owned());
                        }
                    }
                }
            }
            Ok(Event::Text(t)) => {
                if in_nav_label {
                    if let Ok(s) = t.unescape() {
                        current_label.push_str(&s);
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = e.name().as_ref().to_vec();
                match name.as_slice() {
                    b"navLabel" => in_nav_label = false,
                    b"navPoint" => {
                        if let Some(src) = current_src.take() {
                            let label = current_label.trim().to_string();
                            if !label.is_empty() {
                                let key = normalize_entry_name(&src);
                                map.insert(key, label);
                            }
                        }
                        current_label.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    map
}

// ---------------------------------------------------------------------------
// 章节抽取
// ---------------------------------------------------------------------------

fn extract_chapters<R: std::io::Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    index: &HashMap<String, String>,
    toc_map: &HashMap<String, String>,
    opf: &OpfInfo,
) -> Result<Vec<ParsedChapter>, String> {
    let mut chapters: Vec<ParsedChapter> = Vec::new();

    for idref in &opf.spine {
        let item = match opf.items.get(idref) {
            Some(i) => i,
            None => continue,
        };

        // 仅处理 XHTML/HTML
        let mt = item.media_type.to_ascii_lowercase();
        if !(mt.contains("xhtml") || mt == "text/html" || mt == "application/html") {
            continue;
        }

        // 过滤掉封面、目录、版权页等非正文项
        if is_skippable(item) {
            continue;
        }

        let xhtml = match read_zip_entry(archive, index, &item.href) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let cc = parse_xhtml(&xhtml);
        let content_trim = cc.content.trim();
        if content_trim.is_empty() {
            continue;
        }

        // 标题优先级：toc.ncx（权威）→ XHTML <title> → XHTML <h1>/<h2>/<h3> → 文件名
        let chapter_title = toc_map
            .get(&normalize_entry_name(&item.href))
            .cloned()
            .or_else(|| cc.title.clone())
            .or_else(|| extract_heading_title(&xhtml))
            .unwrap_or_else(|| file_stem_of(&item.href));
        let chapter_title = chapter_title.trim().to_string();
        if chapter_title.is_empty() {
            continue;
        }

        let total_chars = content_trim.chars().count() as i32;
        chapters.push(ParsedChapter {
            number: chapters.len() as i32 + 1,
            title: chapter_title,
            content: content_trim.to_string(),
            total_chars,
        });
    }

    Ok(chapters)
}

/// 是否应当作为非正文章节跳过（目录、封面、版权页等）
fn is_skippable(item: &ManifestItem) -> bool {
    let props_lower = item.properties.to_ascii_lowercase();
    for kw in &[
        "nav",
        "toc",
        "cover",
        "cover-image",
        "titlepage",
        "title-page",
        "colophon",
        "copyright-page",
        "copyright",
        "dedication",
        "acknowledgments",
        "glossary",
        "index",
        "bibliography",
    ] {
        if props_lower.split_whitespace().any(|p| p == *kw) {
            return true;
        }
    }
    let href_lower = item.href.to_ascii_lowercase();
    let fname = Path::new(&item.href)
        .file_name()
        .map(|s| s.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    for kw in &[
        "nav", "toc", "cover", "title-page", "titlepage",
        "colophon", "copyright", "dedication", "acknowledgments",
        "glossary", "index", "bibliography", "imprint",
    ] {
        if fname.contains(kw) {
            return true;
        }
        if href_lower.contains(kw) && !href_lower.contains("chapter")
            && !href_lower.contains("chap")
            && !href_lower.contains("section")
        {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// XHTML → (title?, content)
// ---------------------------------------------------------------------------

fn parse_xhtml(html: &str) -> ChapterFile {
    let mut reader = Reader::from_str(html);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut out = String::new();
    let mut title_text = String::new();
    let mut title_result: Option<String> = None;
    let mut in_title = false;
    // 是否要把当前节点的文本丢弃（head/style/script 等）
    let mut depth_in_skip = 0usize;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().as_ref().to_vec();
                match name.as_slice() {
                    b"title" => {
                        in_title = true;
                        title_text.clear();
                    }
                    b"head" | b"script" | b"style" | b"noscript" => {
                        depth_in_skip += 1;
                    }
                    b"p" | b"div" | b"li" | b"blockquote" | b"h1" | b"h2" | b"h3"
                    | b"h4" | b"h5" | b"h6" => {
                        ensure_paragraph_break(&mut out);
                    }
                    b"br" | b"hr" => {
                        out.push('\n');
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(e)) => {
                let name = e.name().as_ref().to_vec();
                if name.as_slice() == b"br" || name.as_slice() == b"hr" {
                    out.push('\n');
                }
            }
            Ok(Event::Text(t)) => {
                if depth_in_skip > 0 && !in_title {
                    // 跳过 head/style/script 内容（但 <title> 在 head 内，需保留其文本）
                } else {
                    let text = t.unescape().map(|c| c.into_owned()).unwrap_or_default();
                    let decoded = decode_named_entities(&text);
                    if in_title {
                        title_text.push_str(&decoded);
                    } else {
                        out.push_str(&decoded);
                    }
                }
            }
            Ok(Event::CData(t)) => {
                if depth_in_skip == 0 && !in_title {
                    let s = String::from_utf8_lossy(t.as_ref()).into_owned();
                    out.push_str(&s);
                }
            }
            Ok(Event::End(e)) => {
                let name = e.name().as_ref().to_vec();
                match name.as_slice() {
                    b"title" => {
                        let trimmed = title_text.trim();
                        if !trimmed.is_empty() && title_result.is_none() {
                            title_result = Some(trimmed.to_string());
                        }
                        in_title = false;
                    }
                    b"head" | b"script" | b"style" | b"noscript" => {
                        if depth_in_skip > 0 {
                            depth_in_skip -= 1;
                        }
                    }
                    b"p" | b"div" | b"li" | b"blockquote" | b"h1" | b"h2" | b"h3"
                    | b"h4" | b"h5" | b"h6" => {
                        out.push('\n');
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    ChapterFile {
        title: title_result,
        content: normalize_whitespace(&out),
    }
}

/// 从 XHTML 中兜底提取 `<h1>`/`<h2>`/`<h3>` 文本作为章节名
/// （很多 EPUB 没有 `<title>`、或标题用 `<h2>` 而非 `<h1>` 表示）
fn extract_heading_title(html: &str) -> Option<String> {
    let mut reader = Reader::from_str(html);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut in_heading = false;
    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if is_heading_tag(e.name().as_ref()) {
                    in_heading = true;
                    text.clear();
                }
            }
            Ok(Event::Text(t)) => {
                if in_heading {
                    let s = t.unescape().map(|c| c.into_owned()).unwrap_or_default();
                    text.push_str(&decode_named_entities(&s));
                }
            }
            Ok(Event::End(e)) => {
                if is_heading_tag(e.name().as_ref()) {
                    let t = text.trim();
                    if !t.is_empty() {
                        return Some(t.to_string());
                    }
                    in_heading = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    None
}

fn is_heading_tag(name: &[u8]) -> bool {
    matches!(name, b"h1" | b"h2" | b"h3" | b"h4" | b"h5" | b"h6")
}

/// 一些不是 XML 预定义实体的 HTML 命名实体（&nbsp; 等），按需替换为可见字符。
///
/// 注意：quick-xml 的 unescape 已处理数字实体（&#123;）与 XML 预定义实体（&amp; 等），
/// 这里只兜底替换常见的 HTML 命名实体。直接用 `str::replace` 逐个替换，
/// 天然按字符边界操作，避免手写字节遍历在中文字符中间切分导致 panic。
fn decode_named_entities(s: &str) -> String {
    const ENTITIES: &[(&str, &str)] = &[
        ("&nbsp;", " "),
        ("&mdash;", "——"),
        ("&ndash;", "–"),
        ("&hellip;", "…"),
        ("&laquo;", "«"),
        ("&raquo;", "»"),
        ("&lsquo;", "‘"),
        ("&rsquo;", "’"),
        ("&ldquo;", "“"),
        ("&rdquo;", "”"),
        ("&copy;", "©"),
        ("&reg;", "®"),
        ("&trade;", "™"),
    ];
    let mut out = s.to_string();
    for (name, repl) in ENTITIES {
        out = out.replace(name, repl);
    }
    out
}

/// 保证 string 末尾存在 `\n`（用于 block 元素前补一个换行，避免段落粘连）
fn ensure_paragraph_break(s: &mut String) {
    if s.is_empty() {
        return;
    }
    if !s.ends_with('\n') {
        s.push('\n');
    }
}

/// 把连续 `\n` 折叠为单个 `\n`，并去掉首尾空白
fn normalize_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_n = false;
    for c in s.chars() {
        if c == '\n' {
            if prev_n {
                continue;
            }
            prev_n = true;
        } else {
            prev_n = false;
        }
        result.push(c);
    }
    result.trim().to_string()
}

fn file_stem_of(href: &str) -> String {
    Path::new(href)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名章节")
        .to_string()
}

// ---------------------------------------------------------------------------
// ZIP 工具：跨平台读文本
//
// 路径前缀在 Windows 上常见 `OEBPS\foo.xhtml`，macOS 是 `OEBPS/foo.xhtml`；
// 且 OPF 里的 href 多为相对 OPF 目录的路径（如 `chapter1.xhtml`），
// 与 ZIP 内实际路径（`OEBPS/chapter1.xhtml`）不一致。
//
// 为避免每个章节都遍历整个 ZIP（会退化成 O(n²) 卡死大 EPUB），
// 这里一次性构建「规范化名字 → 实际条目名」索引，后续 O(1) 查找。
// ---------------------------------------------------------------------------

/// 规范化条目名：反斜杠转斜杠、去 leading slash、转小写
fn normalize_entry_name(name: &str) -> String {
    name.replace('\\', "/")
        .trim_start_matches('/')
        .to_ascii_lowercase()
}

/// 一次性遍历 ZIP 中央目录，构建「规范化名字 → 实际条目名」映射
fn build_name_index<R: std::io::Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for i in 0..archive.len() {
        if let Ok(f) = archive.by_index(i) {
            let actual = f.name().to_string();
            let normalized = normalize_entry_name(&actual);
            map.entry(normalized).or_insert(actual);
        }
    }
    map
}

/// 按目标名在索引中定位实际条目名（直接匹配 → 精确匹配 → 末段后缀匹配）
fn resolve_entry_from_index<R: std::io::Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    index: &HashMap<String, String>,
    target: &str,
) -> Option<String> {
    // 1) 直接按原名取（覆盖 OPF 里已展开的绝对/相对路径）
    if archive.by_name(target).is_ok() {
        return Some(target.to_string());
    }
    let target_n = normalize_entry_name(target);
    if target_n.is_empty() {
        return None;
    }
    // 2) 精确匹配（忽略 leading slash 与大小写）
    if let Some(actual) = index.get(&target_n) {
        return Some(actual.clone());
    }
    // 3) 末段后缀匹配（如 href="chapter1.xhtml" 命中 "OEBPS/chapter1.xhtml"）
    let suffix = format!("/{target_n}");
    for (normalized, actual) in index {
        if normalized.ends_with(&suffix) {
            return Some(actual.clone());
        }
    }
    None
}

/// 读取 ZIP 条目并解码为 String（BOM / UTF-16 / UTF-8）
fn read_zip_entry<R: std::io::Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    index: &HashMap<String, String>,
    target: &str,
) -> Result<String, String> {
    let actual = resolve_entry_from_index(archive, index, target)
        .ok_or_else(|| format!("EPUB 中找不到文件: {target}"))?;
    let mut f = archive
        .by_name(&actual)
        .map_err(|e| format!("读取 EPUB 条目失败: {e}"))?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    Ok(decode_xml_bytes(&bytes))
}

/// 把 XML/XHTML 字节按 BOM / UTF-16 / UTF-8 解码为 String
fn decode_xml_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    // BOM 检测
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8_lossy(&bytes[3..]).into_owned();
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        // UTF-16 LE
        let mut i = 2;
        let mut out = String::new();
        while i + 1 < bytes.len() {
            let cu = u16::from_le_bytes([bytes[i], bytes[i + 1]]);
            if let Some(c) = char::from_u32(cu as u32) {
                out.push(c);
            }
            i += 2;
        }
        return out;
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let mut i = 2;
        let mut out = String::new();
        while i + 1 < bytes.len() {
            let cu = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
            if let Some(c) = char::from_u32(cu as u32) {
                out.push(c);
            }
            i += 2;
        }
        return out;
    }
    String::from_utf8_lossy(bytes).into_owned()
}

#[allow(dead_code)]
fn _suppress_unused() {}

// ---------------------------------------------------------------------------
// 封面：抽取并写入本地缓存目录
// ---------------------------------------------------------------------------

fn extract_cover<R: std::io::Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    index: &HashMap<String, String>,
    opf: &OpfInfo,
) -> Result<Option<String>, String> {
    // 1) 优先找 properties="cover-image" / "cover" 的 item
    let item = opf
        .items
        .values()
        .find(|it| {
            let p = it.properties.to_ascii_lowercase();
            p.split_whitespace().any(|p| p == "cover-image")
                || p.split_whitespace().any(|p| p == "cover")
        })
        .or_else(|| opf.cover_id.as_ref().and_then(|id| opf.items.get(id)))
        .cloned();

    let item = match item {
        Some(i) => i,
        None => return Ok(None),
    };

    let media_type = item.media_type.to_ascii_lowercase();
    if !media_type.starts_with("image/") {
        return Ok(None);
    }

    // 后缀名兜底
    let ext = Path::new(&item.href)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_else(|| match media_type.as_str() {
            "image/jpeg" => "jpg".to_string(),
            "image/png" => "png".to_string(),
            "image/gif" => "gif".to_string(),
            "image/webp" => "webp".to_string(),
            "image/svg+xml" => "svg".to_string(),
            _ => "jpg".to_string(),
        });

    // 用 OPF href + 媒体类型做稳定 hash，避免反复覆写
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    item.href.hash(&mut h);
    media_type.hash(&mut h);
    let key = h.finish();

    let cover_dir = std::env::temp_dir()
        .join("stealthreader")
        .join("covers");
    std::fs::create_dir_all(&cover_dir).map_err(|e| format!("创建封面目录失败: {e}"))?;
    let target = cover_dir.join(format!("{:x}.{}", key, ext));

    let real_name = match resolve_entry_from_index(archive, index, &item.href) {
        Some(n) => n,
        None => return Ok(None),
    };

    let mut src = archive
        .by_name(&real_name)
        .map_err(|e| format!("读取封面失败: {e}"))?;
    let mut dst = File::create(&target).map_err(|e| format!("写入封面失败: {e}"))?;
    std::io::copy(&mut src, &mut dst).map_err(|e| format!("写入封面失败: {e}"))?;
    dst.flush().ok();

    Ok(Some(target.to_string_lossy().to_string()))
}
