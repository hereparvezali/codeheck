use crate::{error::AppError, utils::config::SmtpConfig};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
    Message,
};

pub struct Mailer;

impl Mailer {
    pub async fn send_verification_email(
        config: &SmtpConfig,
        to_email: &str,
        username: &str,
        verification_token: &str,
    ) -> Result<(), AppError> {
        let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_email)
            .parse()
            .map_err(|e| AppError::internal(format!("Invalid SMTP from address: {}", e)))?;

        let to_mailbox: Mailbox = format!("{} <{}>", username, to_email)
            .parse()
            .map_err(|e| AppError::internal(format!("Invalid recipient email address: {}", e)))?;

        let verification_url = format!(
            "{}/verify-email?token={}",
            config.frontend_url.trim_end_matches('/'),
            verification_token
        );

        let email_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify your CodeHeck account</title>
</head>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; background-color: #09090b; color: #f4f4f5; padding: 40px 20px; margin: 0;">
    <div style="max-width: 560px; margin: 0 auto; background-color: #18181b; border: 1px solid #27272a; border-radius: 16px; padding: 32px;">
        <h1 style="color: #ffffff; font-size: 24px; font-weight: 700; margin-top: 0;">Welcome to CodeHeck, {username}!</h1>
        <p style="color: #a1a1aa; font-size: 15px; line-height: 1.6;">
            Thank you for signing up. Please verify your email address to activate your account and start solving coding challenges.
        </p>
        <div style="margin: 32px 0; text-align: center;">
            <a href="{verification_url}" style="background-color: #ffffff; color: #09090b; font-weight: 600; font-size: 14px; text-decoration: none; padding: 12px 28px; border-radius: 8px; display: inline-block;">
                Verify Email Address
            </a>
        </div>
        <p style="color: #71717a; font-size: 13px; line-height: 1.5;">
            Or copy and paste this link in your browser:<br>
            <a href="{verification_url}" style="color: #a1a1aa; word-break: break-all;">{verification_url}</a>
        </p>
        <hr style="border: none; border-top: 1px solid #27272a; margin: 24px 0;">
        <p style="color: #71717a; font-size: 12px; margin-bottom: 0;">
            If you did not create an account on CodeHeck, you can safely ignore this email.
        </p>
    </div>
</body>
</html>"#,
            username = username,
            verification_url = verification_url,
        );

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject("Verify your CodeHeck email address")
            .header(ContentType::TEXT_HTML)
            .body(email_body)
            .map_err(|e| AppError::internal(format!("Failed to build email message: {}", e)))?;

        let mut builder = if config.port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|e| AppError::internal(format!("Failed to create SMTP relay: {}", e)))?
                .port(config.port)
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
                .map_err(|e| AppError::internal(format!("Failed to create SMTP relay: {}", e)))?
                .port(config.port)
        };

        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            builder = builder.credentials(Credentials::new(username.clone(), password.clone()));
        }

        let mailer = builder.build();

        mailer
            .send(email)
            .await
            .map_err(|e| AppError::internal(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_message_formatting() {
        let config = SmtpConfig {
            host: "smtp.example.com".to_string(),
            port: 587,
            username: Some("testuser".to_string()),
            password: Some("testpass".to_string()),
            from_email: "noreply@codeheck.com".to_string(),
            from_name: "CodeHeck".to_string(),
            frontend_url: "https://codeheck.com".to_string(),
        };

        let token = "a1b2c3d4e5f678901234567890abcdef";
        let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_email)
            .parse()
            .unwrap();
        let to_mailbox: Mailbox = format!("{} <{}>", "testuser", "user@example.com")
            .parse()
            .unwrap();

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject("Verify your CodeHeck email address")
            .header(ContentType::TEXT_HTML)
            .body(format!("Token: {}", token));

        assert!(email.is_ok());
    }

    #[test]
    fn test_verification_token_length() {
        let token = uuid::Uuid::new_v4().simple().to_string();
        assert_eq!(token.len(), 32);
    }

    #[tokio::test]
    #[ignore]
    async fn test_send_actual_email() {
        let config = SmtpConfig {
            host: "smtp.gmail.com".to_string(),
            port: 587,
            username: Some("hereparvezali@gmail.com".to_string()),
            password: Some("gxcxaexfqquwaroa".to_string()),
            from_email: "hereparvezali@gmail.com".to_string(),
            from_name: "CodeHeck".to_string(),
            frontend_url: "http://localhost:5173".to_string(),
        };

        let result = Mailer::send_verification_email(
            &config,
            "parvezsamiraali@gmail.com",
            "parvezsamiraali",
            "12345678901234567890123456789012",
        )
        .await;

        println!("Send result: {:?}", result);
        assert!(result.is_ok());
    }
}
