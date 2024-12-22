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
    const BASE64_ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes = text.as_bytes();
    let mut encoded = String::new();
    let mut padding = 0;

    // Iterate over input bytes in chunks of 3 (24 bits).
    for chunk in bytes.chunks(3) {
        let mut buffer = 0u32;
        for (i, &byte) in chunk.iter().enumerate() {
            buffer |= (byte as u32) << (16 - i * 8);
        }

        // Calculate padding based on chunk length.
        padding = 3 - chunk.len();

        // Encode into 4 Base64 characters.
        for i in 0..(4 - padding) {
            let index = ((buffer >> (18 - i * 6)) & 0x3F) as usize;
            encoded.push(BASE64_ALPHABET[index] as char);
        }
    }

    // Add padding characters '='.
    for _ in 0..padding {
        encoded.push('=');
    }

    encoded
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
pub fn decode_base64(encoded: &str) -> Result<String, String> {
    const BASE64_ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut decoded = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in encoded.as_bytes() {
        if byte == b'=' {
            break; // Stop processing on padding characters.
        }

        let index = BASE64_ALPHABET.iter().position(|&b| b == byte);
        if let Some(index) = index {
            buffer = (buffer << 6) | (index as u32);
            bits_collected += 6;

            if bits_collected >= 8 {
                bits_collected -= 8;
                decoded.push((buffer >> bits_collected) as u8 & 0xFF);
            }
        } else {
            return Err(format!("Invalid Base64 character: {}", byte as char));
        }
    }

    // Convert decoded bytes to a UTF-8 string.
    match String::from_utf8(decoded) {
        Ok(s) => Ok(s),
        Err(_) => Err("Decoded bytes are not valid UTF-8".to_string()),
    }
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
    text.bytes()
        .map(|b| format!("{:02x}", b))
        .collect()
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
pub fn decode_hex(encoded: &str) -> Result<String, String> {
    if encoded.len() % 2 != 0 {
        return Err("Invalid hex string length".to_string());
    }

    let bytes: Result<Vec<u8>, String> = (0..encoded.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&encoded[i..i + 2], 16)
                .map_err(|_| format!("Invalid hex character at position {}", i))
        })
        .collect();

    match bytes {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err("Decoded bytes are not valid UTF-8".to_string()),
        },
        Err(e) => Err(e),
    }
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

