use lettre::{Message, transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use lettre::transport::smtp::{response::Response, Error};
use crate::mail::Mail;

pub struct Mailer{
    server: String,
    username: String,
    password: String,
}

impl Mailer{
    pub fn new(server: &str, username: &str, password: &str)->Self{
        Self{
            server: server.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    pub fn send(&self, mail: Mail) -> Result<Response, Error>{
        self.post(&mail.from, &mail.to, &mail.subject, &mail.body)
    }

    pub fn post(&self, from: &str, to: &str, subject: &str, body: &str) -> Result<Response, Error>{
        let email = Message::builder()
            .from(from.parse().unwrap())
            .reply_to(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(body.to_string())
            .unwrap();
        let credentials = Credentials::new(self.username.to_string(), self.password.to_string());
        let mailer = SmtpTransport::relay(&self.server)
            .unwrap()
            .credentials(credentials)
            .build();
        mailer.send(&email)
    }
}
