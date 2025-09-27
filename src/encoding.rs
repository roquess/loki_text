use std::collections::HashMap;

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

/// Encodes text using Base32 encoding.
///
/// # Arguments
///
/// * `text` - A string slice to encode.
///
/// # Returns
///
/// * A `String` containing the Base32 encoded text.
///
/// # Examples
///
/// ```
/// let text = "hello";
/// let result = loki_text::encoding::encode_base32(text);
/// assert_eq!(result, "NBSWY3DP");
/// ```
pub fn encode_base32(text: &str) -> String {
    const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let bytes = text.as_bytes();
    let mut encoded = String::new();
    
    for chunk in bytes.chunks(5) {
        let mut buffer = 0u64;
        for (i, &byte) in chunk.iter().enumerate() {
            buffer |= (byte as u64) << (32 - i * 8);
        }
        
        let output_chars = match chunk.len() {
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 8,
            _ => unreachable!(),
        };
        
        for i in 0..output_chars {
            let index = ((buffer >> (35 - i * 5)) & 0x1F) as usize;
            encoded.push(BASE32_ALPHABET[index] as char);
        }
    }
    
    encoded
}

/// Decodes Base32 encoded text.
///
/// # Arguments
///
/// * `encoded` - A string slice containing Base32 encoded text.
///
/// # Returns
///
/// * A `Result<String, String>` containing the decoded text or an error.
pub fn decode_base32(encoded: &str) -> Result<String, String> {
    const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut decoded = Vec::new();
    let mut buffer = 0u64;
    let mut bits_collected = 0;
    
    for &byte in encoded.as_bytes() {
        if byte == b'=' { break; }
        
        let index = BASE32_ALPHABET.iter().position(|&b| b == byte)
            .ok_or_else(|| format!("Invalid Base32 character: {}", byte as char))?;
        
        buffer = (buffer << 5) | (index as u64);
        bits_collected += 5;
        
        if bits_collected >= 8 {
            bits_collected -= 8;
            decoded.push((buffer >> bits_collected) as u8 & 0xFF);
        }
    }
    
    String::from_utf8(decoded).map_err(|_| "Decoded bytes are not valid UTF-8".to_string())
}

/// Encodes text using Base58 (Bitcoin-style) encoding.
///
/// # Arguments
///
/// * `text` - A string slice to encode.
///
/// # Returns
///
/// * A `String` containing the Base58 encoded text.
pub fn encode_base58(text: &str) -> String {
    const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let bytes = text.as_bytes();
    
    if bytes.is_empty() {
        return String::new();
    }
    
    // Count leading zeros
    let leading_zeros = bytes.iter().take_while(|&&b| b == 0).count();
    
    // Convert to big integer representation
    let mut num = bytes.iter().fold(0u128, |acc, &b| acc * 256 + b as u128);
    
    // Convert to base58
    let mut encoded = String::new();
    while num > 0 {
        let remainder = (num % 58) as usize;
        encoded.insert(0, BASE58_ALPHABET[remainder] as char);
        num /= 58;
    }
    
    // Add leading '1's for leading zeros
    for _ in 0..leading_zeros {
        encoded.insert(0, '1');
    }
    
    if encoded.is_empty() {
        "1".to_string()
    } else {
        encoded
    }
}

/// Simple XOR cipher encoding/decoding.
///
/// # Arguments
///
/// * `text` - A string slice to encode/decode.
/// * `key` - A string slice containing the key.
///
/// # Returns
///
/// * A `String` containing the XOR encoded/decoded text (as hex).
///
/// # Examples
///
/// ```
/// let text = "hello";
/// let key = "key";
/// let encoded = loki_text::encoding::xor_cipher(text, key);
/// let decoded = loki_text::encoding::xor_cipher_from_hex(&encoded, key);
/// assert_eq!(decoded.unwrap(), "hello");
/// ```
pub fn xor_cipher(text: &str, key: &str) -> String {
    if key.is_empty() {
        return encode_hex(text);
    }
    
    let key_bytes = key.as_bytes();
    text.bytes()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Decodes XOR cipher from hex string.
pub fn xor_cipher_from_hex(hex_text: &str, key: &str) -> Result<String, String> {
    if key.is_empty() {
        return decode_hex(hex_text);
    }
    
    let bytes = (0..hex_text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_text[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|_| "Invalid hex string".to_string())?;
    
    let key_bytes = key.as_bytes();
    let decoded: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    
    String::from_utf8(decoded).map_err(|_| "Decoded bytes are not valid UTF-8".to_string())
}

/// Encodes text using a simple Caesar cipher.
///
/// # Arguments
///
/// * `text` - A string slice to encode.
/// * `shift` - The number of positions to shift (0-25).
///
/// # Returns
///
/// * A `String` containing the Caesar cipher encoded text.
pub fn caesar_cipher(text: &str, shift: u8) -> String {
    let shift = shift % 26;
    text.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + shift) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + shift) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// Decodes Caesar cipher.
pub fn caesar_decipher(text: &str, shift: u8) -> String {
    caesar_cipher(text, 26 - (shift % 26))
}

/// Encodes text using Atbash cipher (A=Z, B=Y, etc.).
///
/// # Arguments
///
/// * `text` - A string slice to encode.
///
/// # Returns
///
/// * A `String` containing the Atbash encoded text.
pub fn atbash_cipher(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
            'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
            _ => c,
        })
        .collect()
}

/// Converts text to Morse code.
///
/// # Arguments
///
/// * `text` - A string slice to convert.
///
/// # Returns
///
/// * A `String` containing the Morse code representation.
pub fn to_morse_code(text: &str) -> String {
    let morse_map: HashMap<char, &str> = [
        ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."), ('E', "."),
        ('F', "..-."), ('G', "--."), ('H', "...."), ('I', ".."), ('J', ".---"),
        ('K', "-.-"), ('L', ".-.."), ('M', "--"), ('N', "-."), ('O', "---"),
        ('P', ".--."), ('Q', "--.-"), ('R', ".-."), ('S', "..."), ('T', "-"),
        ('U', "..-"), ('V', "...-"), ('W', ".--"), ('X', "-..-"), ('Y', "-.--"),
        ('Z', "--.."), ('0', "-----"), ('1', ".----"), ('2', "..---"),
        ('3', "...--"), ('4', "....-"), ('5', "....."), ('6', "-...."),
        ('7', "--..."), ('8', "---.."), ('9', "----."), (' ', "/"),
    ].iter().cloned().collect();
    
    text.to_uppercase()
        .chars()
        .filter_map(|c| morse_map.get(&c).copied())
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Converts Morse code back to text.
///
/// # Arguments
///
/// * `morse` - A string slice containing Morse code.
///
/// # Returns
///
/// * A `Result<String, String>` containing the decoded text or an error.
pub fn from_morse_code(morse: &str) -> Result<String, String> {
    let morse_map: HashMap<&str, char> = [
        (".-", 'A'), ("-...", 'B'), ("-.-.", 'C'), ("-..", 'D'), (".", 'E'),
        ("..-.", 'F'), ("--.", 'G'), ("....", 'H'), ("..", 'I'), (".---", 'J'),
        ("-.-", 'K'), (".-..", 'L'), ("--", 'M'), ("-.", 'N'), ("---", 'O'),
        (".--.", 'P'), ("--.-", 'Q'), (".-.", 'R'), ("...", 'S'), ("-", 'T'),
        ("..-", 'U'), ("...-", 'V'), (".--", 'W'), ("-..-", 'X'), ("-.--", 'Y'),
        ("--..", 'Z'), ("-----", '0'), (".----", '1'), ("..---", '2'),
        ("...--", '3'), ("....-", '4'), (".....", '5'), ("-....", '6'),
        ("--...", '7'), ("---..", '8'), ("----.", '9'), ("/", ' '),
    ].iter().cloned().collect();
    
    morse.split_whitespace()
        .map(|code| {
            morse_map.get(code).copied()
                .ok_or_else(|| format!("Unknown Morse code: {}", code))
        })
        .collect()
}

/// Converts text to phonetic alphabet (NATO).
///
/// # Arguments
///
/// * `text` - A string slice to convert.
///
/// # Returns
///
/// * A `String` containing the phonetic alphabet representation.
pub fn to_phonetic_alphabet(text: &str) -> String {
    let phonetic_map: HashMap<char, &str> = [
        ('A', "Alpha"), ('B', "Bravo"), ('C', "Charlie"), ('D', "Delta"),
        ('E', "Echo"), ('F', "Foxtrot"), ('G', "Golf"), ('H', "Hotel"),
        ('I', "India"), ('J', "Juliet"), ('K', "Kilo"), ('L', "Lima"),
        ('M', "Mike"), ('N', "November"), ('O', "Oscar"), ('P', "Papa"),
        ('Q', "Quebec"), ('R', "Romeo"), ('S', "Sierra"), ('T', "Tango"),
        ('U', "Uniform"), ('V', "Victor"), ('W', "Whiskey"), ('X', "X-ray"),
        ('Y', "Yankee"), ('Z', "Zulu"), ('0', "Zero"), ('1', "One"),
        ('2', "Two"), ('3', "Three"), ('4', "Four"), ('5', "Five"),
        ('6', "Six"), ('7', "Seven"), ('8', "Eight"), ('9', "Nine"),
    ].iter().cloned().collect();
    
    text.to_uppercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                phonetic_map.get(&c).map(|s| s.to_string())
            } else if c == ' ' {
                Some("SPACE".to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Generates a hash-like string using a simple polynomial rolling hash.
///
/// # Arguments
///
/// * `text` - A string slice to hash.
///
/// # Returns
///
/// * A `String` containing the hash representation.
pub fn simple_hash(text: &str) -> String {
    const PRIME: u64 = 31;
    const MODULUS: u64 = 1_000_000_007;
    
    let hash = text.bytes().fold(0u64, |acc, b| {
        (acc.wrapping_mul(PRIME).wrapping_add(b as u64)) % MODULUS
    });
    
    format!("{:016x}", hash)
}

/// Encodes text using a simple substitution cipher with a given key.
///
/// # Arguments
///
/// * `text` - A string slice to encode.
/// * `key` - A 26-character substitution key.
///
/// # Returns
///
/// * A `Result<String, String>` containing the encoded text or an error.
pub fn substitution_cipher(text: &str, key: &str) -> Result<String, String> {
    if key.len() != 26 {
        return Err("Substitution key must be exactly 26 characters".to_string());
    }
    
    let key_upper = key.to_uppercase();
    let key_bytes = key_upper.as_bytes();
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    // Check if key contains all unique letters
    for &letter in alphabet {
        if !key_bytes.contains(&letter) {
            return Err("Substitution key must contain all 26 letters exactly once".to_string());
        }
    }
    
    let result = text.chars()
        .map(|c| match c {
            'a'..='z' => key_bytes[(c as u8 - b'a') as usize] as char,
            'A'..='Z' => key_bytes[(c as u8 - b'A') as usize] as char,
            _ => c,
        })
        .collect();
    
    Ok(result)
}

/// Converts text to various number bases.
///
/// # Arguments
///
/// * `text` - A string slice to convert.
/// * `base` - The target base (2-36).
///
/// # Returns
///
/// * A `Result<String, String>` containing the converted text or an error.
pub fn to_base(text: &str, base: u32) -> Result<String, String> {
    if base < 2 || base > 36 {
        return Err("Base must be between 2 and 36".to_string());
    }
    
    let result = text.bytes()
        .map(|b| {
            let mut num = b as u32;
            let mut digits = Vec::new();
            
            if num == 0 {
                digits.push('0');
            } else {
                while num > 0 {
                    let digit = num % base;
                    let char = if digit < 10 {
                        (b'0' + digit as u8) as char
                    } else {
                        (b'A' + (digit - 10) as u8) as char
                    };
                    digits.push(char);
                    num /= base;
                }
                digits.reverse();
            }
            
            digits.iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ");
    
    Ok(result)
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

    #[test]
    fn test_encode_base32() {
        assert_eq!(encode_base32("hello"), "NBSWY3DP");
        assert_eq!(encode_base32(""), "");
    }

    #[test]
    fn test_decode_base32() {
        assert_eq!(decode_base32("NBSWY3DP").unwrap(), "hello");
    }

    #[test]
    fn test_encode_base58() {
        assert_eq!(encode_base58("hello"), "Cn8eVZg");
        assert_eq!(encode_base58(""), "");
    }

    #[test]
    fn test_xor_cipher() {
        let encrypted = xor_cipher("hello", "key");
        let decrypted = xor_cipher_from_hex(&encrypted, "key").unwrap();
        assert_eq!(decrypted, "hello");
    }

    #[test]
    fn test_caesar_cipher() {
        assert_eq!(caesar_cipher("hello", 3), "khoor");
        assert_eq!(caesar_decipher("khoor", 3), "hello");
    }

    #[test]
    fn test_atbash_cipher() {
        assert_eq!(atbash_cipher("abc"), "zyx");
        assert_eq!(atbash_cipher(atbash_cipher("hello").as_str()), "hello");
    }

    #[test]
    fn test_morse_code() {
        let morse = to_morse_code("HELLO");
        assert_eq!(morse, ".... . .-.. .-.. ---");
        assert_eq!(from_morse_code(&morse).unwrap(), "HELLO");
    }

    #[test]
    fn test_phonetic_alphabet() {
        assert_eq!(to_phonetic_alphabet("ABC"), "Alpha Bravo Charlie");
    }

    #[test]
    fn test_simple_hash() {
        let hash1 = simple_hash("hello");
        let hash2 = simple_hash("hello");
        assert_eq!(hash1, hash2);
        
        let hash3 = simple_hash("world");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_substitution_cipher() {
        let key = "ZYXWVUTSRQPONMLKJIHGFEDCBA";
        let encrypted = substitution_cipher("HELLO", key).unwrap();
        assert_eq!(encrypted, "SVOOL");
    }

    #[test]
    fn test_to_base() {
        assert_eq!(to_base("A", 2).unwrap(), "1000001");
        assert_eq!(to_base("A", 16).unwrap(), "41");
    }
}

