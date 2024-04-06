use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::header::ContentType,Message, SmtpTransport, Transport};

pub fn send_email(from: &str, password: &str, to: &str, subject: &str, body: &str) {
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(from.to_owned(), password.to_owned());

    let mailer = SmtpTransport::starttls_relay("smtp-mail.outlook.com")
        .unwrap()
        .credentials(creds)
        .port(587)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully..."),
        Err(e) => eprintln!("Error sending email: {:?}", e),
    }
}
