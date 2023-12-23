use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;


pub fn decrypt_with_aes(key: [u8; 16], mut buf: Vec<u8>) -> String {
    let pt = Aes128EcbDec::new(&key.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .unwrap();

    String::from_utf8(pt.to_vec()).unwrap()
}


pub fn encrypt_with_aes(key: [u8; 16], plaintext: &str) -> Vec<u8> {
    let plaintext = plaintext.as_bytes();
    let pt_len = plaintext.len();
    let mut buf: [u8; 1000] = [0u8; 1000];
    buf[..pt_len].copy_from_slice(&plaintext);
    let ct = Aes128EcbEnc::new(&key.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();

    ct.to_vec()
}


pub fn pad_password(key: Vec<u8>) -> [u8; 16] {
    let mut new_vec = key.to_vec();
    
    while new_vec.len() < 16 {
        new_vec.push(31);
    };

    let padded_key: [u8; 16] = new_vec.try_into().unwrap();
    padded_key
}

