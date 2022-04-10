extern crate lettre;
use lettre::smtp::authentication::Credentials;
use lettre::{
    SmtpClient, Transport
};
use lettre_email::EmailBuilder;

//use std::env; - let args: Vec<String> = env::args().collect();

//mod config_search; - config_search::run();
mod config;

fn main() {

    //config_search::run(); 
    let info = config::run(); // Calling the function that return the vector "info"

    let your_email = String::from(info[0].trim()); // Your email
    let your_passwd = String::from(info[1].trim()); // Your password
    let email_server = String::from(info[2].trim()); // The server of your email
    let mail_to = String::from(info[3].trim()); // Email that you're mailing to
    let mail_from = your_email.clone(); 
    
    let mailft = EmailBuilder::new()
        .to(mail_to)
        .from(mail_from)
        .subject("Hello with mailft") // Email Subject
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