mod totp_client;
mod dbfuncs;
mod crypto_funcs;

use std::env;
use std::process;
use std::io::{stdin, stdout, Write};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: my_cli_app <command>");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "new" => {
            println!("Reading QR from clipboard.");
            let data = totp_client::get_qr_from_clipboard();
            println!("Data received: {:?}", data);
            let mut password = String::new();
            print!("Enter a password (same with others): ");
            let _ = stdout().flush();
            stdin().read_line(&mut password).expect("Did not enter a correct string.");
            
            if password.len() > 16 {println!("Max password length is 16.");return;}
            if let Some((service, code)) = data {
                let padded_password: [u8; 16] = crypto_funcs::pad_password(password.as_bytes().to_vec());
                let encrypted_code = crypto_funcs::encrypt_with_aes(padded_password, &code);
                let _ = dbfuncs::add_new_secret(service, encrypted_code);
            }

        },
        "show" => {
            let secrets = dbfuncs::read_secrets();

            let mut password = String::new();
            print!("Enter a password: ");
            let _ = stdout().flush();
            stdin().read_line(&mut password).expect("Did not enter a valid string.");

            let padded_password: [u8; 16] = crypto_funcs::pad_password(password.as_bytes().to_vec());

            for svc in secrets {
                
                let decrpt = crypto_funcs::decrypt_with_aes(padded_password, svc.secret);
                let code = totp_client::generate_otp(decrpt);
                println!("Code for {} is {}", svc.service, code)
            }
        },
        _ => println!("Unknown command!: {}", command),
    }
}
