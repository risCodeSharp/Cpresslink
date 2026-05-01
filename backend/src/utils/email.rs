use lettre::{
    Message, SmtpTransport, Transport, message::{MultiPart}, transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    }
};

use crate::utils::env_registry::{EnvRegistry, EnvKey};

use crate::error::AppError;

pub async fn send(
    to: &str,
    subject: String,
    plan_text: String,
    html_body: String,
) -> Result<(), AppError> {
    let smtp_user = EnvRegistry::get_env(EnvKey::SmtpUser);
    let smtp_key = EnvRegistry::get_env(EnvKey::SmtpKey);
    let sender_email = EnvRegistry::get_env(EnvKey::SenderEmail);
    let smtp_tls_domain = EnvRegistry::get_env(EnvKey::SmtpTlsDomain);

    // Parse sender
    let from_addr = format!("CPresslink Website <{}>", sender_email)
        .parse()
        .map_err(|e| AppError::InvalidSendEmailMessage(format!("Invalid sender email: {e}")))?;

    // Parse recipient
    let to_addr = to
        .parse()
        .map_err(|e| AppError::InvalidSendEmailMessage(format!("Invalid recipient email: {e}")))?;

    // Build message
    let email = Message::builder()
        .from(from_addr)
        .to(to_addr)
        .subject(subject)
        .multipart(MultiPart::alternative_plain_html(plan_text, html_body))
        .map_err(|e| AppError::InvalidSendEmailMessage(e.to_string()))?;

    // TLS config
    let tls = TlsParameters::builder(smtp_tls_domain.clone())
        .build()
        .map_err(|e| AppError::InvalidSendEmailMessage(format!("TLS build failed: {e}")))?;

    // SMTP transport
    let mailer = SmtpTransport::relay(&smtp_tls_domain)
        .map_err(|e| AppError::InvalidSendEmailMessage(format!("SMTP relay failed: {e}")))?
        .credentials(Credentials::new(smtp_user, smtp_key))
        .port(587)
        .tls(Tls::Required(tls))
        .build();

    // Send email
    mailer
        .send(&email)
        .map_err(|e| AppError::FailedToSendMail(e.to_string()))?;

    Ok(())
}