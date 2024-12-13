use base64::{engine::general_purpose, Engine as _};
use hex::{encode as hex_encode, decode as hex_decode};

/// Encodes a string into Base64.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to encode.
///
/// # Returns
///
/// * A `String` containing the Base64 encoded text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::encoding::encode_base64(text);
/// assert_eq!(result, "aGVsbG8gd29ybGQ=");
/// ```
pub fn encode_base64(text: &str) -> String {
    general_purpose::STANDARD.encode(text)
}

/// Decodes a Base64 string into a regular string.
///
/// # Arguments
///
/// * `encoded` - A string slice that holds the Base64 encoded text to decode.
///
/// # Returns
///
/// * A `Result<String, base64::DecodeError>` containing the decoded text or an error.
///
/// # Examples
///
/// ```
/// let encoded = "aGVsbG8gd29ybGQ=";
/// let result = loki_text::encoding::decode_base64(encoded);
/// assert_eq!(result, Ok("hello world".to_string()));
/// ```
pub fn decode_base64(encoded: &str) -> Result<String, base64::DecodeError> {
    let bytes = general_purpose::STANDARD.decode(encoded)?;
    Ok(String::from_utf8(bytes).unwrap())
}

/// Encodes a string into Hex.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to encode.
///
/// # Returns
///
/// * A `String` containing the Hex encoded text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::encoding::encode_hex(text);
/// assert_eq!(result, "68656c6c6f20776f726c64");
/// ```
pub fn encode_hex(text: &str) -> String {
    hex_encode(text)
}

/// Decodes a Hex string into a regular string.
///
/// # Arguments
///
/// * `encoded` - A string slice that holds the Hex encoded text to decode.
///
/// # Returns
///
/// * A `Result<String, hex::FromHexError>` containing the decoded text or an error.
///
/// # Examples
///
/// ```
/// let encoded = "68656c6c6f20776f726c64";
/// let result = loki_text::encoding::decode_hex(encoded);
/// assert_eq!(result, Ok("hello world".to_string()));
/// ```
pub fn decode_hex(encoded: &str) -> Result<String, hex::FromHexError> {
    let bytes = hex_decode(encoded)?;
    Ok(String::from_utf8(bytes).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base64() {
        let text = "hello world";
        let result = encode_base64(text);
        assert_eq!(result, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_decode_base64() {
        let encoded = "aGVsbG8gd29ybGQ=";
        let result = decode_base64(encoded);
        assert_eq!(result, Ok("hello world".to_string()));
    }

    #[test]
    fn test_encode_hex() {
        let text = "hello world";
        let result = encode_hex(text);
        assert_eq!(result, "68656c6c6f20776f726c64");
    }

    #[test]
    fn test_decode_hex() {
        let encoded = "68656c6c6f20776f726c64";
        let result = decode_hex(encoded);
        assert_eq!(result, Ok("hello world".to_string()));
    }
}

