//! 小说文件解析
//!
//! 支持的格式：
//! - `.txt`：纯文本，按正则切章节
//! - `.epub`：ZIP+XML 容器，按 OPF spine 顺序解析章节 XHTML，并抽取元数据/封面

pub mod epub;
pub mod txt;

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
    /// 封面本地绝对路径（已写入应用可访问的本地缓存目录），无封面则为 `None`
    pub cover_path: Option<String>,
}

/// 解析入口，根据扩展名分发到具体解析器
pub fn parse_book(file_path: &str) -> Result<ParsedBook, String> {
    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "epub" => epub::parse(file_path),
        // 其它（含 .txt 与无扩展名）都按纯文本处理，保留原 txt 解析行为
        _ => txt::parse(file_path),
    }
}
