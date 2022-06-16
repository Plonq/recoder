pub fn encode(text: String, encoding: &Encoding) -> String {
    match encoding {
        Encoding::Base64 => base64::encode(text.into_bytes()),
        Encoding::Uri => urlencoding::encode(text.as_str()).to_string(),
    }
}

pub fn decode(text: String, encoding: &Encoding) -> Result<String, String> {
    match encoding {
        Encoding::Base64 => match base64::decode(text) {
            Ok(decoded) => String::from_utf8(decoded).map_err(|e| e.to_string()),
            Err(error) => Err(error.to_string()),
        },
        Encoding::Uri => match urlencoding::decode(text.as_str()) {
            Ok(decoded) => Ok(decoded.to_string()),
            Err(error) => Err(error.to_string()),
        },
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Encoding {
    Base64,
    Uri,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Base64
    }
}
