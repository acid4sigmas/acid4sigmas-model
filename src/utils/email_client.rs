use crate::{
    models::email_client::{EmailClient, EmailClientCreds},
    secrets::{NO_REPLY_EMAIL, SMTP_PASSWORD, SMTP_RELAY, SMTP_USERNAME},
    to_string_,
};
use anyhow::Result;
use lettre::{
    message::SinglePart, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

impl EmailClient {
    pub fn new(email: &str, subject: &str) -> Result<Self> {
        let username = SMTP_USERNAME.get().unwrap();
        let password = SMTP_PASSWORD.get().unwrap();
        let smtp_relay = SMTP_RELAY.get().unwrap();

        Ok(EmailClient {
            subject: to_string_!(subject),
            email: to_string_!(email),
            creds: EmailClientCreds {
                username: username.clone(),
                password: password.clone(),
                smtp_relay: smtp_relay.clone(),
            },
        })
    }

    pub fn send(&self, body: &str) -> Result<()> {
        let email = Message::builder()
            .from(NO_REPLY_EMAIL.get().unwrap().parse().unwrap())
            .to(self.email.parse().unwrap())
            .subject(&self.subject)
            .singlepart(SinglePart::html(body.to_string()))?;

        let creds = Credentials::new(
            self.creds.username.to_string(),
            self.creds.password.to_string(),
        );

        let mailer = SmtpTransport::relay(&self.creds.smtp_relay)?
            .credentials(creds)
            .build();

        mailer.send(&email)?;

        Ok(())
    }
}
