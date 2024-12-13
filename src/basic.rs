/// Splits the text into substrings based on a delimiter.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to split.
/// * `delimiter` - A string slice that holds the delimiter to split by.
///
/// # Returns
///
/// * A `Vec<String>` containing the substrings.
///
/// # Examples
///
/// ```
/// let text = "one,two,three";
/// let delimiter = ",";
/// let result = loki_text::basic::split_text(text, delimiter);
/// assert_eq!(result, vec!["one", "two", "three"]);
/// ```
pub fn split_text(text: &str, delimiter: &str) -> Vec<String> {
    text.split(delimiter).map(|s| s.to_string()).collect()
}

/// Joins a list of substrings into a single string with a delimiter.
///
/// # Arguments
///
/// * `parts` - A vector of string slices that holds the substrings to join.
/// * `delimiter` - A string slice that holds the delimiter to join by.
///
/// # Returns
///
/// * A `String` containing the joined substrings.
///
/// # Examples
///
/// ```
/// let parts = vec!["one", "two", "three"];
/// let delimiter = ",";
/// let result = loki_text::basic::join_text(parts, delimiter);
/// assert_eq!(result, "one,two,three");
/// ```
pub fn join_text(parts: Vec<&str>, delimiter: &str) -> String {
    parts.join(delimiter)
}

/// Converts a string to uppercase.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the uppercase text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::basic::to_uppercase(text);
/// assert_eq!(result, "HELLO WORLD");
/// ```
pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// Converts a string to lowercase.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the lowercase text.
///
/// # Examples
///
/// ```
/// let text = "HELLO WORLD";
/// let result = loki_text::basic::to_lowercase(text);
/// assert_eq!(result, "hello world");
/// ```
pub fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

/// Trims whitespace from the beginning and end of a string.
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
/// let result = loki_text::basic::trim_whitespace(text);
/// assert_eq!(result, "hello world");
/// ```
pub fn trim_whitespace(text: &str) -> String {
    text.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_text() {
        let text = "one,two,three";
        let delimiter = ",";
        assert_eq!(split_text(text, delimiter), vec!["one", "two", "three"]);
    }

    #[test]
    fn test_join_text() {
        let parts = vec!["one", "two", "three"];
        let delimiter = ",";
        assert_eq!(join_text(parts, delimiter), "one,two,three");
    }

    #[test]
    fn test_to_uppercase() {
        let text = "hello world";
        assert_eq!(to_uppercase(text), "HELLO WORLD");
    }

    #[test]
    fn test_to_lowercase() {
        let text = "HELLO WORLD";
        assert_eq!(to_lowercase(text), "hello world");
    }

    #[test]
    fn test_trim_whitespace() {
        let text = "  hello world  ";
        assert_eq!(trim_whitespace(text), "hello world");
    }
}

