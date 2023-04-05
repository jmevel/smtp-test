use crate::contact::Contact;

pub struct EmailClientSettings<'a> {
    pub smtp_server: SmtpServerSettings<'a>,
    pub email: EmailSettings<'a>,
}

pub struct SmtpServerSettings<'a> {
    pub host: &'a str,
    pub port: u16,
    pub credentials: CredentialsSettings<'a>,
}

pub struct CredentialsSettings<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

pub struct EmailSettings<'a> {
    pub from: Contact<'a>,
    pub reply_to: Contact<'a>,
}
