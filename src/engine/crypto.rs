use hmac::{Hmac, Mac};
use sha1::{Digest, Sha1};
use sha2::Sha256;

pub fn md5_hash(msg: &str) -> String {
    hex::encode(md5::compute(msg).to_vec())
}

pub fn sha1_hash(msg: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(msg.as_bytes());
    hex::encode(hasher.finalize().to_vec())
}

pub fn sha256_hash(msg: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(msg.as_bytes());
    hex::encode(hasher.finalize().to_vec())
}

pub fn hmac_digest_hex(key: &str, msg: &str) -> String {
    hex::encode(hmac_digest(key, msg))
}

pub fn hmac_digest_b64(key: &str, msg: &str) -> String {
    base64::encode(hmac_digest(key, msg))
}

fn hmac_digest(key: &str, msg: &str) -> Vec<u8> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(&msg.as_bytes());

    let code_bytes = mac.finalize().into_bytes();

    code_bytes.to_vec()
}
