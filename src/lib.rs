use regex::Regex;

/// Finds the first occurrence of a pattern in the text and returns the captured group.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the regex pattern to search for.
///
/// # Returns
///
/// * An `Option<String>` containing the captured group if the pattern is found, otherwise `None`.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = r"quick\s(\w+)";
/// assert_eq!(loki_text::find_pattern(text, pattern), Some("brown".to_string()));
/// ```
pub fn find_pattern(text: &str, pattern: &str) -> Option<String> {
    let re = Regex::new(pattern).ok()?;
    re.captures(text).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

/// Replaces all occurrences of a pattern in the text with a replacement string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the regex pattern to search for.
/// * `replacement` - A string slice that holds the replacement text.
///
/// # Returns
///
/// * A `String` with all occurrences of the pattern replaced by the replacement text.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = r"brown";
/// let replacement = "red";
/// assert_eq!(loki_text::replace_pattern(text, pattern, replacement), "The quick red fox jumps over the lazy dog");
/// ```
pub fn replace_pattern(text: &str, pattern: &str, replacement: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    re.replace_all(text, replacement).to_string()
}

/// Counts the number of occurrences of a pattern in the text.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the regex pattern to search for.
///
/// # Returns
///
/// * A `usize` representing the number of occurrences of the pattern in the text.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = r"the";
/// assert_eq!(loki_text::count_pattern(text, pattern), 2);
/// ```
pub fn count_pattern(text: &str, pattern: &str) -> usize {
    let re = Regex::new(&format!("(?i){}", pattern)).unwrap();
    re.find_iter(text).count()
}

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
/// assert_eq!(loki_text::split_text(text, delimiter), vec!["one", "two", "three"]);
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
/// assert_eq!(loki_text::join_text(parts, delimiter), "one,two,three");
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
/// assert_eq!(loki_text::to_uppercase(text), "HELLO WORLD");
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
/// assert_eq!(loki_text::to_lowercase(text), "hello world");
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
/// assert_eq!(loki_text::trim_whitespace(text), "hello world");
/// ```
pub fn trim_whitespace(text: &str) -> String {
    text.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pattern() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = r"quick\s(\w+)";
        assert_eq!(find_pattern(text, pattern), Some("brown".to_string()));
    }

    #[test]
    fn test_replace_pattern() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = r"brown";
        let replacement = "red";
        assert_eq!(replace_pattern(text, pattern, replacement), "The quick red fox jumps over the lazy dog");
    }

    #[test]
    fn test_count_pattern() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = r"the";
        assert_eq!(count_pattern(text, pattern), 2);
    }
    
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

