use lettre::{
    Message, SmtpTransport, Transport, message::{MultiPart}, transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    }
};

use crate::utils::env_registry::{EnvRegistry, EnvKey};

use crate::error::AppError;

pub async fn send(to: &str, subject: String, plan_text: String, html_body: String) -> Result<(), AppError> {
    let smtp_user = EnvRegistry::get_env(EnvKey::SmtpUser);
    let smtp_key =  EnvRegistry::get_env(EnvKey::SmtpKey);
    let sender_email = EnvRegistry::get_env( EnvKey::SenderEmail);

    let email = Message::builder()
        .from(
            format!("CPresslink Website <{}>", sender_email)
                .parse()
                .unwrap(),
        )
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative_plain_html(plan_text, html_body)
        )    
        .map_err(|e| AppError::InvalidSendEmailMessage(e.to_string()))?;

    let creds = Credentials::new(smtp_user, smtp_key);
    let smtp_tls_domain = EnvRegistry::get_env(EnvKey::SmtpTlsDomain);
    let tls = TlsParameters::builder(smtp_tls_domain.clone())
        .build()
        .unwrap();

    let mailer = SmtpTransport::relay(&smtp_tls_domain)
        .unwrap()
        .credentials(creds)
        .port(587)
        .tls(Tls::Required(tls))
        .build();

    mailer
        .send(&email)
        .map_err(|e| AppError::FailedToSendMail(e.to_string()))?;

    Ok(())
}
