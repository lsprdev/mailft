use std::io;
use std::io::Write;
use std::process;
use rpassword;

pub fn run() -> Vec<String> {

    let mut res = String::new();
    //========================================
    let mut out = io::stdout();
    let into = io::stdin();
    //========================================
    let mut your_email = String::new();
    let mut your_pwd = String::new();
    let mut your_email_server = String::new();
    let mut mail_to = String::new();
    //========================================
    let info: Vec<String>;
    //========================================
    print!("Do you already have a config?(y/n): ");
    out.flush().unwrap();
    into.read_line(&mut res).unwrap();

    if res.trim() == "n" {
        // make config
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

    } else if res.trim() == "y" {
    // search for existing config
        println!("Deu 'y' aqui!");

    } else {
        process::exit(1);
    }

    info = vec![your_email, your_pwd, your_email_server, mail_to]; // Get if info and put into this vec
    return info // returns the vector so that I can use into anoter file
}
