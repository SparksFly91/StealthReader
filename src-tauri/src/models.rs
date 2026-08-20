use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub total: i64,
    pub list: Vec<T>,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    success: bool,
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    // 成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: 200,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    // 成功响应，数据为空
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse {
            success: true,
            code: 200,
            msg: "操作成功".to_string(),
            data: None,
        }
    }
    // 错误响应
    pub fn error(code: i32, message: &str) -> Self {
        Self {
            success: false,
            code,
            msg: message.to_string(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BookInfo {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover: String,
    pub total_chapters: i32,
    pub total_chars: i32,
    pub introduction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: i64,
    pub number: i32,
    pub title: String,
    pub content: String,
    pub char_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i64>,
    pub title: String,
    pub author: String,
    pub cover: String,
    pub introduction: String,
    pub file_path: String,
    pub total_chapters: i32,
    pub total_chars: i32,
    pub chapters: Vec<Chapter>,
}