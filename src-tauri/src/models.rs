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
