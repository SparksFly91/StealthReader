use std::fs;
use std::path::Path;
use std::time::Duration;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use lettre::{
    message::{
        header::{ContentDisposition, ContentId, ContentType},
        Mailbox, MultiPart, SinglePart,
    },
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Deserialize;

/// 反馈配置模板（首次运行自动复制到应用配置目录，真实授权码不随代码提交）
const EXAMPLE_CONFIG: &str = include_str!("../../feedback.config.example.json");

#[derive(Debug, Deserialize)]
pub struct FeedbackConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
    pub to_address: String,
}

impl FeedbackConfig {
    /// 是否已填入真实配置（而非模板占位符）
    pub fn is_configured(&self) -> bool {
        let has_placeholder = self.smtp_username.contains("your_email")
            || self.smtp_password.contains("your_16_digit")
            || self.from_address.contains("your_email")
            || self.to_address.contains("your_email");
        !(has_placeholder
            || self.smtp_username.trim().is_empty()
            || self.smtp_password.trim().is_empty()
            || self.from_address.trim().is_empty()
            || self.to_address.trim().is_empty())
    }
}

/// 内联图片：cid 对应 HTML 中的 `cid:xxx` 引用
#[derive(Debug, Deserialize)]
pub struct FeedbackImage {
    pub cid: String,
    pub mime: String,
    pub base64: String,
}

/// 加载反馈配置；文件不存在时用模板自动生成占位配置
pub fn load_or_init_config(config_path: &Path) -> Result<FeedbackConfig, String> {
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败：{e}"))?;
        }
        fs::write(config_path, EXAMPLE_CONFIG).map_err(|e| format!("生成反馈配置模板失败：{e}"))?;
    }
    let content = fs::read_to_string(config_path).map_err(|e| format!("读取反馈配置失败：{e}"))?;
    serde_json::from_str(&content).map_err(|e| format!("反馈配置解析失败：{e}"))
}

/// 通过 SMTP 发送反馈邮件（text + html 双 body，图片以 cid 内联）
pub async fn send_feedback(
    config: &FeedbackConfig,
    email: &str,
    subject: &str,
    html: &str,
    text: &str,
    images: &[FeedbackImage],
) -> Result<(), String> {
    let from: Mailbox = config
        .from_address
        .parse()
        .map_err(|e| format!("发件地址无效：{e}"))?;
    let to: Mailbox = config
        .to_address
        .parse()
        .map_err(|e| format!("收件地址无效：{e}"))?;
    // Reply-To 设为反馈人邮箱，收到后点「回复」即可直接回信
    let reply_to: Mailbox = email
        .parse()
        .map_err(|e| format!("反馈人邮箱无效：{e}"))?;

    let text_part = SinglePart::builder()
        .header(ContentType::TEXT_PLAIN)
        .body(text.to_string());
    let html_part = SinglePart::builder()
        .header(ContentType::TEXT_HTML)
        .body(html.to_string());
    let alternative = MultiPart::alternative()
        .singlepart(text_part)
        .singlepart(html_part);

    let mut related = MultiPart::related().multipart(alternative);
    for img in images {
        let bytes = STANDARD
            .decode(&img.base64)
            .map_err(|e| format!("图片数据解码失败：{e}"))?;
        let content_type = ContentType::parse(&img.mime)
            .unwrap_or_else(|_| ContentType::parse("application/octet-stream").unwrap());
        let part = SinglePart::builder()
            .header(content_type)
            .header(ContentDisposition::inline())
            .header(ContentId::from(img.cid.clone()))
            .body(bytes);
        related = related.singlepart(part);
    }

    let message = Message::builder()
        .from(from)
        .reply_to(reply_to)
        .to(to)
        .subject(subject.to_string())
        .multipart(related)
        .map_err(|e| format!("邮件构建失败：{e}"))?;

    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
    let tls_params = TlsParameters::new(config.smtp_host.clone())
        .map_err(|e| format!("TLS 参数配置失败：{e}"))?;
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
        .port(config.smtp_port)
        .tls(Tls::Wrapper(tls_params))
        .credentials(creds)
        .timeout(Some(Duration::from_secs(15)))
        .build();

    mailer
        .send(message)
        .await
        .map_err(|e| format!("邮件发送失败：{e}"))?;

    Ok(())
}
