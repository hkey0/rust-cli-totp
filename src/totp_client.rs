use bardecoder;
use image::{RgbaImage, ImageBuffer, EncodableLayout};
use arboard::Clipboard;


use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::decode;

use sha1::Sha1;
use hmac::{Hmac, Mac};

use data_encoding::BASE32;

use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};



pub fn get_qr_from_clipboard() -> Option<(String, String)> {
    let mut clipboard = Clipboard::new().unwrap();
    let image = match clipboard.get_image() {
        Ok(img) => img,
        Err(e) => {
            panic!("Error getting image: {}", e);
    
        }
    };

    let image: RgbaImage = ImageBuffer::from_raw(
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.bytes.into_owned(),
    ).unwrap();


    let decoder = bardecoder::default_decoder();
    let results = decoder.decode(&image);
    let last_result = results.get(0).unwrap();
    
    // print!("{:?}", last_result);
    parse_otpauth_url(String::from(last_result.as_ref().expect("CANNOT READ THE QR").to_string()).as_str())

}


fn secret_key_padding(secret_key: String) -> String {
    let missing = secret_key.len() % 8;
    return format!("{}{}", secret_key, std::iter::repeat("=").take(missing).collect::<String>());
}


pub fn generate_otp(secret_key: String) -> u128 {
    let secret_key = secret_key_padding(secret_key);   
    let from_unix: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() / 30;

    let time_bytes = from_unix.to_be_bytes();

    let secret_base32 = BASE32.decode(secret_key.as_bytes()).unwrap();
    let mut mac = Hmac::<Sha1>::new_from_slice(secret_base32.as_bytes())
        .expect("HMAC can take key of any size");
    

    mac.update(&time_bytes);
    let result = mac.finalize().into_bytes();

    // let hmac_digest = hex_encode(result);

    // println!("{}", hmac_digest);
    // println!("{:?}", u128::from_str_radix(&hmac_digest.trim(), 16).unwrap() & 0x0F);

    let num1: BigUint = BigUint::from_u8(result[result.len() - 1]).unwrap();
    let num2: BigUint = BigUint::from_u128(0x0F).unwrap();

    let offset: usize = (num1 & num2).to_usize().unwrap();
    let code: &[u8] = &result[offset..offset+4];

    let code_as_int: BigUint = BigUint::from_bytes_be(code);
    let bign: BigUint = BigUint::from_u128(0x7FFFFFFF).unwrap();

    let otp: BigUint = code_as_int & bign;
    let otp_code: u128 = BigUint::to_u128(&otp).unwrap();

    otp_code % 1000000

}


fn parse_otpauth_url(uri: &str) -> Option<(String, String)> {
    let url = decode(uri).expect("UTF-8");
    let parts: Vec<&str> = url.split('?').collect();
    
    if parts.len() != 2 {
        return None;
    }

    let path = parts[0];
    let query = parts[1];

    let path_parts: Vec<&str> = path.split('/').collect();
    if path_parts.len() < 4 {
        return None;
    }

    let service = path_parts[3].to_string();

    let query_parts: Vec<&str> = query.split('&').collect();
    let mut secret = String::new();

    for part in query_parts {
        if part.starts_with("secret=") {
            secret = part["secret=".len()..].to_string();
            break;
        }
    }

    if secret.is_empty() {
        None
    } else {
        Some((service, secret))
    }
}