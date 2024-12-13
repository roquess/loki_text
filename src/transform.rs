use regex::Regex;

/// Reverses a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to reverse.
///
/// # Returns
///
/// * A `String` containing the reversed text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::reverse_string(text);
/// assert_eq!(result, "dlrow olleh");
/// ```
pub fn reverse_string(text: &str) -> String {
    text.chars().rev().collect()
}

/// Checks if a string is a palindrome.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to check.
///
/// # Returns
///
/// * A `bool` indicating whether the text is a palindrome.
///
/// # Examples
///
/// ```
/// let text = "racecar";
/// let result = loki_text::transform::is_palindrome(text);
/// assert_eq!(result, true);
/// ```
pub fn is_palindrome(text: &str) -> bool {
    let cleaned: String = text.chars().filter(|c| c.is_alphanumeric()).collect();
    cleaned.eq_ignore_ascii_case(&cleaned.chars().rev().collect::<String>())
}

/// Removes punctuation from a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to clean.
///
/// # Returns
///
/// * A `String` containing the text without punctuation.
///
/// # Examples
///
/// ```
/// let text = "Hello, world!";
/// let result = loki_text::transform::remove_punctuation(text);
/// assert_eq!(result, "Hello world");
/// ```
pub fn remove_punctuation(text: &str) -> String {
    text.chars().filter(|c| !c.is_ascii_punctuation()).collect()
}

/// Extracts all numbers from a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to extract numbers from.
///
/// # Returns
///
/// * A `Vec<String>` containing the extracted numbers.
///
/// # Examples
///
/// ```
/// let text = "There are 123 apples and 456 oranges.";
/// let result = loki_text::transform::extract_numbers(text);
/// assert_eq!(result, vec!["123", "456"]);
/// ```
pub fn extract_numbers(text: &str) -> Vec<String> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(text).map(|mat| mat.as_str().to_string()).collect()
}

/// Capitalizes the first letter of each word in a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to capitalize.
///
/// # Returns
///
/// * A `String` containing the capitalized text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::capitalize_words(text);
/// assert_eq!(result, "Hello World");
/// ```
pub fn capitalize_words(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let text = "hello world";
        assert_eq!(reverse_string(text), "dlrow olleh");
    }

    #[test]
    fn test_is_palindrome() {
        let text = "racecar";
        assert_eq!(is_palindrome(text), true);
        let text = "hello";
        assert_eq!(is_palindrome(text), false);
    }

    #[test]
    fn test_remove_punctuation() {
        let text = "Hello, world!";
        assert_eq!(remove_punctuation(text), "Hello world");
    }

    #[test]
    fn test_extract_numbers() {
        let text = "There are 123 apples and 456 oranges.";
        assert_eq!(extract_numbers(text), vec!["123", "456"]);
    }

    #[test]
    fn test_capitalize_words() {
        let text = "hello world";
        assert_eq!(capitalize_words(text), "Hello World");
    }
}

