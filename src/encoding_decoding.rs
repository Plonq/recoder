pub fn encode(text: String) -> String {
    base64::encode(text.into_bytes())
}

pub fn decode(text: String) -> Result<String, ()> {
    if let Ok(decoded) = base64::decode(text) {
        Ok(String::from_utf8(decoded).unwrap())
    } else {
        Err(())
    }
}
