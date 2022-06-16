use crate::app::Encoding;


pub fn encode(text: String, encoding: &Encoding) -> String {
    match encoding {
        Encoding::Base64 => {
            base64::encode(text.into_bytes())
        }
    }
}

pub fn decode(text: String, encoding: &Encoding) -> Result<String, String> {
    match encoding {
        Encoding::Base64 => {
            match base64::decode(text) {
                Ok(decoded) => String::from_utf8(decoded).map_err(|e| e.to_string()),
                Err(error) => Err(error.to_string())
            }
        }
    }
}
