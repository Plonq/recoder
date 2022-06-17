pub fn encode(text: String, encoding: &Encoding) -> String {
    match encoding {
        Encoding::Base64 => base64::encode(text.into_bytes()),
        Encoding::Uri => urlencoding::encode(text.as_str()).to_string(),
        Encoding::Hex => hex::encode(text),
        Encoding::Html => html_escape::encode_text(text.as_str()).to_string(),
    }
}

pub fn decode(text: String, encoding: &Encoding) -> Result<String, String> {
    match encoding {
        Encoding::Base64 => match base64::decode(text) {
            Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
            Err(error) => Err(error.to_string()),
        },
        Encoding::Uri => match urlencoding::decode(text.as_str()) {
            Ok(decoded) => Ok(decoded.to_string()),
            Err(error) => Err(error.to_string()),
        },
        Encoding::Hex => match hex::decode(text) {
            Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
            Err(error) => Err(error.to_string()),
        },
        Encoding::Html => Ok(html_escape::decode_html_entities(text.as_str()).to_string()),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Encoding {
    Base64,
    Uri,
    Hex,
    Html,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Base64
    }
}
