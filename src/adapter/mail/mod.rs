use crate::{config::get_config, errors::ServiceError};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use uuid::Uuid;

pub struct Email<'a> {
    pub to: String,
    pub email_type: EmailType<'a>,
}

impl<'a> Email<'a> {
    pub fn new(to: String, email_type: EmailType<'a>) -> Self {
        Self { to, email_type }
    }
}

pub enum EmailType<'a> {
    VerificationCode(&'a String),
    ProjectInvitation(&'a Uuid),
}

impl EmailType<'_> {
    fn get_subject(&self) -> &'static str {
        match self {
            EmailType::VerificationCode(_) => "[AutCloud] Verification code has arrived",
            EmailType::ProjectInvitation(_) => "[AutCloud] You have been invited to join a project",
        }
    }

    fn get_body(&self) -> String {
        match self {
            EmailType::VerificationCode(code) => format!(
                "Your verification code: {}\n\nPlease enter this code to continue.\nThis code is valid for 5 minutes.",
                code
            ),
            EmailType::ProjectInvitation(project_id) => format!(
                "You have been invited to join the project\nTo accept this invitation, please visit the following URL: https://autcloud-fe.vercel.app/project/{}\n\nNote: For security reasons, you must be a registered member to join the project.",
                project_id
            ),
        }
    }
}

pub async fn send_email(email: Email<'_>) -> Result<(), ServiceError> {
    let config = get_config();
    let (username, password) = (
        config.gmail_username.clone(),
        config.gmail_app_password.clone(),
    );

    let email = Message::builder()
        .from(format!("AutCloud <{}>", username).parse()?)
        .to(format!("Receiver <{}>", email.to).parse()?)
        .subject(email.email_type.get_subject())
        .header(ContentType::TEXT_PLAIN)
        .body(email.email_type.get_body())?;

    // Open a remote connection to gmail
    let creds = Credentials::new(username, password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    if cfg!(test) {
        Ok(())
    } else {
        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(err) => Err(ServiceError::EmailError(Box::new(err))),
        }
    }
}

impl From<lettre::address::AddressError> for ServiceError {
    fn from(err: lettre::address::AddressError) -> Self {
        ServiceError::EmailError(Box::new(err))
    }
}

impl From<lettre::error::Error> for ServiceError {
    fn from(err: lettre::error::Error) -> Self {
        ServiceError::EmailError(Box::new(err))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn test_send_email() {
    //     let subject = "Test Email";
    //     let body = "This is a test email";
    //     send_email("jollidah@gmail.com", subject, body).await.unwrap();
    // }
}
