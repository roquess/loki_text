/// Checks if a string is empty or contains only whitespace.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to check.
///
/// # Returns
///
/// * A `bool` indicating whether the text is empty or contains only whitespace.
///
/// # Examples
///
/// ```
/// let text = "   ";
/// let result = loki_text::utils::is_empty_or_whitespace(text);
/// assert_eq!(result, true);
/// ```
pub fn is_empty_or_whitespace(text: &str) -> bool {
    text.trim().is_empty()
}

/// Removes leading and trailing whitespace from a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to trim.
///
/// # Returns
///
/// * A `String` containing the trimmed text.
///
/// # Examples
///
/// ```
/// let text = "  hello world  ";
/// let result = loki_text::utils::trim_whitespace(text);
/// assert_eq!(result, "hello world");
/// ```
pub fn trim_whitespace(text: &str) -> String {
    text.trim().to_string()
}

/// Converts a string to a vector of bytes.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `Vec<u8>` containing the bytes of the string.
///
/// # Examples
///
/// ```
/// let text = "hello";
/// let result = loki_text::utils::to_byte_vector(text);
/// assert_eq!(result, vec![104, 101, 108, 108, 111]);
/// ```
pub fn to_byte_vector(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

/// Converts a vector of bytes to a string.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to convert.
///
/// # Returns
///
/// * A `String` containing the converted text.
///
/// # Examples
///
/// ```
/// let bytes = vec![104, 101, 108, 108, 111];
/// let result = loki_text::utils::to_string(&bytes);
/// assert_eq!(result, "hello");
/// ```
pub fn to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_or_whitespace() {
        let text = "   ";
        assert_eq!(is_empty_or_whitespace(text), true);
        let text = "hello";
        assert_eq!(is_empty_or_whitespace(text), false);
    }

    #[test]
    fn test_trim_whitespace() {
        let text = "  hello world  ";
        assert_eq!(trim_whitespace(text), "hello world");
    }

    #[test]
    fn test_to_byte_vector() {
        let text = "hello";
        assert_eq!(to_byte_vector(text), vec![104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_to_string() {
        let bytes = vec![104, 101, 108, 108, 111];
        assert_eq!(to_string(&bytes), "hello");
    }
}

