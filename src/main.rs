use lettre::{
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    SmtpTransport,
};
use smtp_test::{
    contact::Contact,
    email::Email,
    email_client::EmailClient,
    settings::{CredentialsSettings, EmailSettings, SmtpServerSettings},
};

/// Sends a plain text email to yourself
fn main() {
    let contact = Contact {
        name: "Your name",
        email: Email::parse("youremail@hotmail.com").unwrap(),
    };
    let email_settings = EmailSettings {
        from: contact.clone(),
        reply_to: contact.clone(),
    };
    let credentials_settings = CredentialsSettings {
        username: "youremail@hotmail.com",
        password: "Your password",
    };
    let smtp_server_settings = SmtpServerSettings {
        host: "smtp.office365.com",
        port: 587,
        credentials: credentials_settings,
    };
    let smtp_client = get_smtp_client(smtp_server_settings);
    let email_client = EmailClient::new(email_settings, smtp_client);

    match email_client.send_email(&contact, "This is a test", "Here is a plain text body") {
        Ok(()) => println!("Success !!!"),
        Err(e) => println!("{}", format!("Failure: {e}")),
    }
}

fn get_smtp_client<'a>(settings: SmtpServerSettings<'a>) -> SmtpTransport {
    SmtpTransport::relay(&settings.host)
        .unwrap()
        .port(settings.port)
        .tls(Tls::Required(
            TlsParameters::new(settings.host.into()).unwrap(),
        ))
        .credentials(Credentials::new(
            settings.credentials.username.to_string(),
            settings.credentials.password.to_string(),
        ))
        .timeout(Some(std::time::Duration::from_secs(10)))
        .build()
}
