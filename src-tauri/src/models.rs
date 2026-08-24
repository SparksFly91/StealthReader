use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

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
            code: 0,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    // 成功响应，数据为空
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse {
            success: true,
            code: 0,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Books {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover: String,
    pub file_path: String,
    pub total_chapters: i32,
    pub total_chars: i32,
    pub introduction: String,
    pub create_time: chrono::NaiveDateTime,
    pub last_read_chapter_id: i64,
    pub last_read_position: i32,
    pub last_read_time: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Chapters {
    pub id: i64,
    pub book_id: i64,
    pub number: i32,
    pub title: String,
    pub content: String,
    pub total_chars: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct ReadingProgress {
    pub id: i64,
    pub book_id: i64,
    pub chapter_id: i64,
    pub chapter_number: i32,
    pub position: i32,
    pub is_finished: bool,
    pub read_count: i32,
    pub first_read_time: chrono::NaiveDateTime,
    pub last_read_time: chrono::NaiveDateTime,
}