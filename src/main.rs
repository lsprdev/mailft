use std::io;
use std::io::Write;
//use std::process;

use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use rpassword;

struct UserData {
    user_email: String,
    user_pwd: String,
    user_email_server: String,
    mailing_to: String,
}

fn get_data() -> Vec<String> {

    // ============================== DECLARING VARS ==============================
    let infos: Vec<String>;
    let ( mut your_email, mut your_email_server, mut mail_to ) = {
            ( String::new(), String::new(), String::new())
        };
    let your_pwd;
    let mut sub = String::new();
    let ( mut out, into ) = ( io::stdout(), io::stdin() );
    // ============================== GETING INPUT ==============================
    print!("Your email: ");
    out.flush().unwrap();
    into.read_line(&mut your_email).unwrap();

    your_pwd = rpassword::prompt_password("Your password: ").unwrap(); 
    // Reads passwords in terminal

    print!("Your email server: ");
    out.flush().unwrap();
    into.read_line(&mut your_email_server).unwrap();

    print!("Email that you're going to mail: ");
    out.flush().unwrap();
    into.read_line(&mut mail_to).unwrap();

    println!("====================================");

    print!("Subject of the Email: ");
    out.flush().unwrap();
    into.read_line(&mut sub).unwrap();

    infos = vec![your_email, your_pwd, your_email_server, mail_to, sub];

    infos // returning Vec<String> as especified (fn get_data() -> Vec<String>)

}

fn main() {

    let usr_info = get_data();
    
    let user = UserData {
        user_email: String::from(usr_info[0].trim()),
        user_pwd: String::from(usr_info[1].trim()),
        user_email_server: String::from(usr_info[2].trim()),
        mailing_to: String::from(usr_info[3].trim()),
    };

    // ============================== LETTRE ==============================
    let your_email = String::from(user.user_email); // Your email
    let your_passwd = String::from(user.user_pwd); // Your password
    let email_server = String::from(user.user_email_server); // The server of your email
    let mail_to = String::from(user.mailing_to); // Email that you're mailing to
    let mail_from = your_email.clone(); 
    
    let mailft = EmailBuilder::new()
        .to(mail_to)
        .from(mail_from)
        .subject(usr_info[4].trim()) // Email Subject
        .html("<h1>Hello, Mailft!</h1>") // Email body
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