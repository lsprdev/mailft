extern crate lettre;
use std::env;
use lettre::smtp::authentication::Credentials;
use lettre::{
    SmtpClient, Transport
};
use lettre_email::EmailBuilder;

fn main() {
    let your_email = String::from("mailft@mail.com"); // Your email
    let your_passwd = String::from("linusisalegend"); // Your password
    let email_server = String::from("smtp.mailft.com"); // The server of your email
    let mail_to = String::from("mailft2@mail.com"); // Email that you're mailing to
    let mail_from = your_email.clone(); 
    
    let mailft = EmailBuilder::new()
        .to(mail_to)
        .from(mail_from)
        .subject("Hello From Rust!") // Email Subject
        .html("<h1>Hello, World!</h1>") // Email body
        .build() 
        .unwrap();

        let mut mailft_auth = SmtpClient::new_simple(&email_server)
            .unwrap()
            .credentials(Credentials::new(your_email.into(), your_passwd.into()))
            .transport();
        
        let result = mailft_auth.send(mailft.into()); // Gets all the info passed above and send the email

        if result.is_ok() {
            println!("Email sent, thanks for using mailft!");
        } else {
            println!("Could not send email :( {:?}", result);
        }

        assert!(result.is_ok());
}