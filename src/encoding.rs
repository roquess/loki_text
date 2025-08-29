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

/// Encodes a string using URL encoding (percent encoding).
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to encode.
///
/// # Returns
///
/// * A `String` containing the URL encoded text.
///
/// # Examples
///
/// ```
/// let text = "hello world!";
/// let result = loki_text::encoding::encode_url(text);
/// assert_eq!(result, "hello%20world%21");
/// ```
pub fn encode_url(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
}

/// Decodes a URL encoded string.
///
/// # Arguments
///
/// * `encoded` - A string slice that holds the URL encoded text to decode.
///
/// # Returns
///
/// * A `Result<String, String>` containing the decoded text or an error.
///
/// # Examples
///
/// ```
/// let encoded = "hello%20world%21";
/// let result = loki_text::encoding::decode_url(encoded);
/// assert_eq!(result, Ok("hello world!".to_string()));
/// ```
pub fn decode_url(encoded: &str) -> Result<String, String> {
    let mut decoded = Vec::new();
    let mut chars = encoded.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '%' => {
                let hex1 = chars.next().ok_or("Invalid URL encoding: incomplete percent sequence")?;
                let hex2 = chars.next().ok_or("Invalid URL encoding: incomplete percent sequence")?;
                let hex_str = format!("{}{}", hex1, hex2);
                let byte = u8::from_str_radix(&hex_str, 16)
                    .map_err(|_| format!("Invalid hex sequence: {}", hex_str))?;
                decoded.push(byte);
            }
            '+' => decoded.push(b' '),
            _ => decoded.push(ch as u8),
        }
    }

    String::from_utf8(decoded).map_err(|_| "Decoded bytes are not valid UTF-8".to_string())
}

/// Encodes a string using HTML entity encoding.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to encode.
///
/// # Returns
///
/// * A `String` containing the HTML entity encoded text.
///
/// # Examples
///
/// ```
/// let text = "<script>alert('hello');</script>";
/// let result = loki_text::encoding::encode_html_entities(text);
/// assert_eq!(result, "&lt;script&gt;alert(&#x27;hello&#x27;);&lt;/script&gt;");
/// ```
pub fn encode_html_entities(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Decodes HTML entity encoded string.
///
/// # Arguments
///
/// * `encoded` - A string slice that holds the HTML entity encoded text to decode.
///
/// # Returns
///
/// * A `String` containing the decoded text.
///
/// # Examples
///
/// ```
/// let encoded = "&lt;script&gt;alert(&#x27;hello&#x27;);&lt;/script&gt;";
/// let result = loki_text::encoding::decode_html_entities(encoded);
/// assert_eq!(result, "<script>alert('hello');</script>");
/// ```
pub fn decode_html_entities(encoded: &str) -> String {
    encoded
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
}

/// Encodes a string using ROT13 cipher.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to encode.
///
/// # Returns
///
/// * A `String` containing the ROT13 encoded text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::encoding::encode_rot13(text);
/// assert_eq!(result, "uryyb jbeyq");
/// ```
pub fn encode_rot13(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// Decodes a ROT13 encoded string (ROT13 is its own inverse).
///
/// # Arguments
///
/// * `encoded` - A string slice that holds the ROT13 encoded text to decode.
///
/// # Returns
///
/// * A `String` containing the decoded text.
///
/// # Examples
///
/// ```
/// let encoded = "uryyb jbeyq";
/// let result = loki_text::encoding::decode_rot13(encoded);
/// assert_eq!(result, "hello world");
/// ```
pub fn decode_rot13(encoded: &str) -> String {
    // ROT13 is its own inverse
    encode_rot13(encoded)
}

/// Converts text to binary representation.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the binary representation.
///
/// # Examples
///
/// ```
/// let text = "Hi";
/// let result = loki_text::encoding::to_binary(text);
/// assert_eq!(result, "0100100001101001");
/// ```
pub fn to_binary(text: &str) -> String {
    text.bytes()
        .map(|b| format!("{:08b}", b))
        .collect()
}

/// Converts binary string back to text.
///
/// # Arguments
///
/// * `binary` - A string slice that holds the binary data.
///
/// # Returns
///
/// * A `Result<String, String>` containing the decoded text or an error.
///
/// # Examples
///
/// ```
/// let binary = "0100100001101001";
/// let result = loki_text::encoding::from_binary(binary);
/// assert_eq!(result, Ok("Hi".to_string()));
/// ```
pub fn from_binary(binary: &str) -> Result<String, String> {
    if binary.len() % 8 != 0 {
        return Err("Binary string length must be a multiple of 8".to_string());
    }
    
    let bytes: Result<Vec<u8>, String> = (0..binary.len())
        .step_by(8)
        .map(|i| {
            u8::from_str_radix(&binary[i..i + 8], 2)
                .map_err(|_| format!("Invalid binary sequence at position {}", i))
        })
        .collect();
    
    match bytes {
        Ok(bytes) => String::from_utf8(bytes)
            .map_err(|_| "Decoded bytes are not valid UTF-8".to_string()),
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

    #[test]
    fn test_encode_url() {
        let text = "hello world!";
        assert_eq!(encode_url(text), "hello%20world%21");
        
        let text = "test@example.com";
        assert_eq!(encode_url(text), "test%40example.com");
    }

    #[test]
    fn test_decode_url() {
        let encoded = "hello%20world%21";
        assert_eq!(decode_url(encoded), Ok("hello world!".to_string()));
        
        let encoded = "test%40example.com";
        assert_eq!(decode_url(encoded), Ok("test@example.com".to_string()));
    }

    #[test]
    fn test_encode_html_entities() {
        let text = "<script>alert('hello');</script>";
        assert_eq!(encode_html_entities(text), "&lt;script&gt;alert(&#x27;hello&#x27;);&lt;/script&gt;");
        
        let text = "Tom & Jerry";
        assert_eq!(encode_html_entities(text), "Tom &amp; Jerry");
    }

    #[test]
    fn test_decode_html_entities() {
        let encoded = "&lt;script&gt;alert(&#x27;hello&#x27;);&lt;/script&gt;";
        assert_eq!(decode_html_entities(encoded), "<script>alert('hello');</script>");
        
        let encoded = "Tom &amp; Jerry";
        assert_eq!(decode_html_entities(encoded), "Tom & Jerry");
    }

    #[test]
    fn test_encode_rot13() {
        let text = "hello world";
        assert_eq!(encode_rot13(text), "uryyb jbeyq");
        
        let text = "ABC xyz";
        assert_eq!(encode_rot13(text), "NOP klm");
    }

    #[test]
    fn test_decode_rot13() {
        let encoded = "uryyb jbeyq";
        assert_eq!(decode_rot13(encoded), "hello world");
        
        let encoded = "NOP klm";
        assert_eq!(decode_rot13(encoded), "ABC xyz");
    }

    #[test]
    fn test_to_binary() {
        let text = "Hi";
        assert_eq!(to_binary(text), "0100100001101001");
        
        let text = "A";
        assert_eq!(to_binary(text), "01000001");
    }

    #[test]
    fn test_from_binary() {
        let binary = "0100100001101001";
        assert_eq!(from_binary(binary), Ok("Hi".to_string()));
        
        let binary = "01000001";
        assert_eq!(from_binary(binary), Ok("A".to_string()));
    }
}

