use std::path::Path;
use std::sync::LazyLock;

use chardetng::{EncodingDetector, Iso2022JpDetection, Utf8Detection};
use regex::Regex;

static CHAPTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^[ \t]*(第[0-9一二三四五六七八九十百千万零两]+[章节回卷集部篇][^\n]*|Chapter\s+\d+[^\n]*|楔子|序章|前言|引子|后记|尾声|番外[^\n]*)\s*$").unwrap()
});

#[derive(Debug, Clone)]
pub struct ParsedChapter {
    pub number: i32,
    pub title: String,
    pub content: String,
    pub total_chars: i32,
}

#[derive(Debug, Clone)]
pub struct ParsedBook {
    pub title: String,
    pub author: String,
    pub introduction: String,
    pub total_chapters: i32,
    pub total_chars: i32,
    pub chapters: Vec<ParsedChapter>,
}

/// 读取文件内容，自动识别编码并解码为 UTF-8 字符串
pub fn read_to_string(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("读取文件失败: {e}"))?;
    if bytes.is_empty() {
        return Ok(String::new());
    }
    let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
    detector.feed(&bytes, true);
    let encoding = detector.guess(None, Utf8Detection::Allow);
    let (text, _, _) = encoding.decode(&bytes);
    Ok(text.into_owned())
}

/// 解析小说文件，返回书籍信息与章节列表
pub fn parse_book(file_path: &str) -> Result<ParsedBook, String> {
    let text = read_to_string(file_path)?;
    if text.trim().is_empty() {
        return Err("文件内容为空".to_string());
    }

    let title = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("未命名")
        .to_string();

    let mut positions: Vec<(usize, usize)> = Vec::new();
    for m in CHAPTER_RE.find_iter(&text) {
        positions.push((m.start(), m.end()));
    }

    let mut chapters: Vec<ParsedChapter> = Vec::new();
    let mut introduction = String::new();

    if positions.is_empty() {
        let content = text.trim().to_string();
        let total_chars = content.chars().count() as i32;
        chapters.push(ParsedChapter {
            number: 1,
            title: "正文".to_string(),
            content,
            total_chars,
        });
    } else {
        introduction = text[..positions[0].0].trim().to_string();

        for (i, &(start, end)) in positions.iter().enumerate() {
            let chapter_title = text[start..end].trim().to_string();
            let content_start = end;
            let content_end = if i + 1 < positions.len() {
                positions[i + 1].0
            } else {
                text.len()
            };
            let content = text[content_start..content_end].trim().to_string();
            let total_chars = content.chars().count() as i32;
            chapters.push(ParsedChapter {
                number: i as i32 + 1,
                title: chapter_title,
                content,
                total_chars,
            });
        }
    }

    let total_chars = chapters.iter().map(|c| c.total_chars).sum();
    let total_chapters = chapters.len() as i32;

    Ok(ParsedBook {
        title,
        author: String::new(),
        introduction,
        total_chapters,
        total_chars,
        chapters,
    })
}
