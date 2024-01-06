use std::fs::{self, File};
use std::io::{self, Write};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub service: String,
    pub secret: Vec<u8>,
}


pub fn read_secrets() -> Vec<Service> {
    let file_content: String = fs::read_to_string("./files/secrets.json").unwrap();
    let json: Vec<Service>= 
        serde_json::from_str(&file_content).expect("JSON was not well-formatted");

    json
}


pub fn add_new_secret(service_name: String, secret: Vec<u8>) -> io::Result<()> {
    // JSON dosyasını oku
    let file_content: String = fs::read_to_string("./files/secrets.json")?;
    let mut services: Vec<Service> = serde_json::from_str(&file_content)?;

    services.push(Service {
        service: service_name,
        secret: secret,
    });

    // Güncellenmiş veriyi JSON olarak dosyaya yaz
    let json: String = serde_json::to_string_pretty(&services)?;
    let mut file: File = File::create("./files/secrets.json")?;
    file.write_all(json.as_bytes())?;

    Ok(())
}