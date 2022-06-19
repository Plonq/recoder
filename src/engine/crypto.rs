use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn hmac_signature_hex(key: &str, msg: &str) -> String {
    hex::encode(hmac_signature(key, msg))
}

pub fn hmac_signature_b64(key: &str, msg: &str) -> String {
    base64::encode(hmac_signature(key, msg))
}

fn hmac_signature(key: &str, msg: &str) -> Vec<u8> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(&msg.as_bytes());

    let code_bytes = mac.finalize().into_bytes();

    code_bytes.to_vec()
}
