use serde::Deserialize;
use tauri::Manager;
use tauri_helper::auto_collect_command;

use crate::models::ApiResponse;
use crate::services::feedback::{load_or_init_config, send_feedback, FeedbackImage};

/// 反馈内容大小上限（字符数）
const MAX_HTML_LEN: usize = 100_000;
/// 最多内联图片张数
const MAX_IMAGE_COUNT: usize = 5;
/// 单张图片大小上限（字节）
const MAX_SINGLE_IMAGE_BYTES: usize = 2 * 1024 * 1024;
/// 图片总大小上限（字节）
const MAX_TOTAL_IMAGE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Deserialize)]
pub struct FeedbackSendReq {
    pub email: String,
    pub html: String,
    pub text: String,
    pub images: Vec<FeedbackImage>,
}

/**
 * 发送意见反馈邮件
 * @param req 反馈请求（邮箱、富文本 HTML、纯文本、内联图片）
 * @returns 发送结果
 */
#[tauri::command]
#[auto_collect_command]
pub async fn feedback_send(
    app: tauri::AppHandle,
    req: FeedbackSendReq,
) -> Result<ApiResponse<()>, String> {
    let email = req.email.trim();
    if email.is_empty() || !is_valid_email(email) {
        return Ok(ApiResponse::error(1, "请填写有效的邮箱地址"));
    }
    if req.text.trim().is_empty() {
        return Ok(ApiResponse::error(1, "反馈内容不能为空"));
    }
    if req.html.len() > MAX_HTML_LEN {
        return Ok(ApiResponse::error(1, "反馈内容过长"));
    }
    if req.images.len() > MAX_IMAGE_COUNT {
        return Ok(ApiResponse::error(1, "图片数量过多，最多 5 张"));
    }
    let mut total_bytes = 0usize;
    for img in &req.images {
        // base64 解码后约为原始字节数的 3/4，此处粗略估算用于快速拦截
        let approx_len = img.base64.len() / 4 * 3;
        total_bytes += approx_len;
        if approx_len > MAX_SINGLE_IMAGE_BYTES {
            return Ok(ApiResponse::error(1, "单张图片不能超过 2MB"));
        }
    }
    if total_bytes > MAX_TOTAL_IMAGE_BYTES {
        return Ok(ApiResponse::error(1, "图片总大小不能超过 5MB"));
    }

    // 配置存放于应用配置目录（不随代码提交），首次运行自动生成模板
    let config_path = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("获取配置目录失败：{e}"))?
        .join("feedback.config.json");
    let config = match load_or_init_config(&config_path) {
        Ok(c) => c,
        Err(e) => return Ok(ApiResponse::error(1, &e)),
    };
    if !config.is_configured() {
        return Ok(ApiResponse::error(
            1,
            &format!(
                "反馈邮箱未配置，请编辑 {} 填写 163 邮箱与授权码后重试",
                config_path.display()
            ),
        ));
    }

    let subject = format!("[StealthReader 意见反馈] {email}");
    match send_feedback(&config, email, &subject, &req.html, &req.text, &req.images).await {
        Ok(()) => Ok(ApiResponse::<()>::success_empty()),
        Err(e) => Ok(ApiResponse::error(1, &e)),
    }
}

/// 简易邮箱格式校验
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    !local.is_empty() && domain.contains('.') && !email.chars().any(|c| c.is_whitespace())
}
