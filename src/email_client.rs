use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};

use crate::{contact::Contact, settings::EmailSettings};

pub struct EmailClient {
    email_settings: EmailSettings,
    smtp_client: SmtpTransport,
}

impl EmailClient {
    pub fn new<'a>(email_settings: EmailSettings, smtp_client: SmtpTransport) -> EmailClient {
        EmailClient {
            email_settings,
            smtp_client: smtp_client,
        }
    }

    pub fn send_email(
        &self,
        recipient: &Contact,
        subject: &str,
        text_content: &str,
    ) -> Result<(), String> {
        let email = Message::builder()
            .from(self.email_settings.from.to_string().parse().unwrap())
            .reply_to(self.email_settings.reply_to.to_string().parse().unwrap())
            .to(recipient.to_string().parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(text_content.to_string())
            .unwrap();

        let _formatted_message = String::from_utf8(email.formatted());
        println!("{}", String::from_utf8(email.formatted()).unwrap());

        match self.smtp_client.send(&email) {
            Ok(_) => Result::Ok(()),
            Err(e) => {
                println!("{}", format!("{e:?}"));
                return Result::Err(format!("{e:?}").to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{SafeEmail, Username},
        faker::lorem::en::{Paragraph, Sentence},
        Fake,
    };
    use lettre::{transport::smtp::client::Tls, SmtpTransport};
    use socket_server_mocker::{
        server_mocker::ServerMocker, server_mocker_instruction::ServerMockerInstruction,
        tcp_server_mocker::TcpServerMocker,
    };

    use crate::{
        contact::Contact, email::Email, email_client::EmailClient, settings::EmailSettings,
    };

    #[test]
    fn test_smtp_mock() {
        // Arrange

        // Server
        let smtp_server_port = 2121;
        let smtp_server_mock = TcpServerMocker::new(smtp_server_port).unwrap();
        configure_smtp_server(&smtp_server_mock);

        // Email Client
        let smtp_client = SmtpTransport::relay("localhost")
            .unwrap()
            .tls(Tls::None)
            .port(smtp_server_port)
            .timeout(Some(std::time::Duration::from_secs(10)))
            .build();

        let from = Contact {
            name: Username().fake::<String>(),
            email: Email::parse(SafeEmail().fake::<String>()).unwrap(),
        };
        let reply_to = Contact {
            name: Username().fake::<String>(),
            email: Email::parse(SafeEmail().fake::<String>()).unwrap(),
        };
        let email_settings = EmailSettings {
            from: from.clone(),
            reply_to: reply_to.clone(),
        };
        let email_client = EmailClient::new(email_settings, smtp_client);

        // Email
        let recipient = Contact {
            name: Username().fake::<String>(),
            email: Email::parse(SafeEmail().fake::<String>()).unwrap(),
        };
        let subject: String = Sentence(1..2).fake();
        let text_content: String = Paragraph(1..2).fake();

        // Act
        let send_email_result = email_client.send_email(&recipient, &subject, &text_content);

        // Assert

        println!("\nMessages received by server:");
        // while let Some(message) = smtp_server_mock.pop_received_message() {
        //     println!("{}", String::from_utf8(message).unwrap());
        // }

        //todo!("https://github.com/thomasarmel/socket-server-mocker/issues/6")
        assert!(send_email_result.is_ok());

        // Check that the server received the expected SMTP message
        assert_eq!(
            "EHLO ".as_bytes().to_vec(),
            smtp_server_mock.pop_received_message().unwrap()[..5]
        );
        assert_eq!(
            format!("MAIL FROM:<{}>\r\n", from.clone().email.as_ref())
                .as_bytes()
                .to_vec(),
            smtp_server_mock.pop_received_message().unwrap()
        );
        assert_eq!(
            format!("RCPT TO:<{}>\r\n", recipient.clone().email.as_ref())
                .as_bytes()
                .to_vec(),
            smtp_server_mock.pop_received_message().unwrap()
        );
        assert_eq!(
            "DATA\r\n".as_bytes().to_vec(),
            smtp_server_mock.pop_received_message().unwrap()
        );

        let mail_payload_str =
            String::from_utf8(smtp_server_mock.pop_received_message().unwrap()).unwrap();
        let mut mail_payload_lines = mail_payload_str.lines();

        // Check that the server received the expected mail payload
        assert_eq!(
            format!(
                "From: {} <{}>",
                from.clone().name,
                from.clone().email.as_ref()
            ),
            mail_payload_lines.next().unwrap()
        );
        assert_eq!(
            format!(
                "Reply-To: {} <{}>",
                reply_to.clone().name,
                reply_to.clone().email.as_ref()
            ),
            mail_payload_lines.next().unwrap()
        );
        assert_eq!(
            format!("To: {} <{}>", recipient.name, recipient.email.as_ref()),
            mail_payload_lines.next().unwrap()
        );
        assert_eq!(
            format!("Subject: {subject}"),
            mail_payload_lines.next().unwrap()
        );
        assert_eq!(
            "Content-Type: text/plain; charset=utf-8",
            mail_payload_lines.next().unwrap()
        );
        assert!(Option::is_some(&mail_payload_lines.next()));

        // Email date
        assert!(Option::is_some(&mail_payload_lines.next()));

        // Content
        assert_eq!("", mail_payload_lines.next().unwrap()); // empty line before the content
        assert_eq!(text_content, mail_payload_lines.next().unwrap());

        //Last message line with only a dot "." is not returned by lines() method
        assert_eq!(None, mail_payload_lines.next());

        // Check that no error has been raised by the mocked server
        assert!(smtp_server_mock.pop_server_error().is_none());
    }

    fn configure_smtp_server(smtp_server_mock: &TcpServerMocker) {
        smtp_server_mock.add_mock_instructions(&[
            ServerMockerInstruction::SendMessage("220 smtp.localhost.mock ESMTP Mocker\r\n".as_bytes().to_vec()),
            ServerMockerInstruction::ReceiveMessage,
            ServerMockerInstruction::SendMessage("250-smtp.localhost.mock\r\n250-PIPELINING\r\n250-SIZE 20971520\r\n250-ETRN\r\n250-STARTTLS\r\n250-ENHANCEDSTATUSCODES\r\n250 8BITMIME\r\n".as_bytes().to_vec()),
            ServerMockerInstruction::ReceiveMessage,
            ServerMockerInstruction::SendMessage("250 2.1.0 Ok\r\n".as_bytes().to_vec()),
            ServerMockerInstruction::ReceiveMessage,
            ServerMockerInstruction::SendMessage("250 2.1.5 Ok\r\n".as_bytes().to_vec()),
            ServerMockerInstruction::ReceiveMessage,
            ServerMockerInstruction::SendMessage("354 End data with <CR><LF>.<CR><LF>\r\n".as_bytes().to_vec()),
            ServerMockerInstruction::ReceiveMessage,
            ServerMockerInstruction::SendMessage("250 2.0.0 Ok: queued as 1C1A1B1C1D1E1F1G1H1I1J1K1L1M1N1O1P1Q1R1S1T1U1V1W1X1Y1Z\r\n".as_bytes().to_vec()),
            ServerMockerInstruction::StopExchange,
        ]).unwrap();
    }
}
