pub struct EmailClient {
    pub email: String,
    pub subject: String,
    pub creds: EmailClientCreds,
}

pub struct EmailClientCreds {
    pub username: String,
    pub password: String,
    pub smtp_relay: String,
}
