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

/// Converts a string to CamelCase.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the CamelCase text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::to_camel_case(text);
/// assert_eq!(result, "helloWorld");
/// ```
pub fn to_camel_case(text: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in text.chars() {
        if c.is_whitespace() || c == '_' || c == '-' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c.to_ascii_lowercase());
            }
        }
    }

    result
}

/// Converts a string to snake_case.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the snake_case text.
///
/// # Examples
///
/// ```
/// let text = "Hello World";
/// let result = loki_text::transform::to_snake_case(text);
/// assert_eq!(result, "hello_world");
/// ```
pub fn to_snake_case(text: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in text.chars().enumerate() {
        if i > 0 && (c.is_uppercase() || c.is_whitespace() || c == '-') {
            if !result.ends_with('_') {
                result.push('_');
            }
        }
        
        if !c.is_whitespace() && c != '-' {
            result.push(c.to_ascii_lowercase());
        }
    }
    
    result
}

/// Converts a string to kebab-case.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the kebab-case text.
///
/// # Examples
///
/// ```
/// let text = "Hello World";
/// let result = loki_text::transform::to_kebab_case(text);
/// assert_eq!(result, "hello-world");
/// ```
pub fn to_kebab_case(text: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in text.chars().enumerate() {
        if i > 0 && (c.is_uppercase() || c.is_whitespace() || c == '_') {
            if !result.ends_with('-') {
                result.push('-');
            }
        }
        
        if !c.is_whitespace() && c != '_' {
            result.push(c.to_ascii_lowercase());
        }
    }
    
    result
}

/// Replaces spaces with underscores in a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
///
/// # Returns
///
/// * A `String` containing the text with spaces replaced by underscores.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::replace_spaces_with_underscores(text);
/// assert_eq!(result, "hello_world");
/// ```
pub fn replace_spaces_with_underscores(text: &str) -> String {
    text.replace(' ', "_")
}

/// Reverses the order of words in a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
///
/// # Returns
///
/// * A `String` containing the text with the order of words reversed.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::reverse_words(text);
/// assert_eq!(result, "world hello");
/// ```
pub fn reverse_words(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    words.into_iter().rev().collect::<Vec<&str>>().join(" ")
}

/// Removes special characters from a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
///
/// # Returns
///
/// * A `String` containing the text without special characters.
///
/// # Examples
///
/// ```
/// let text = "hello, world!";
/// let result = loki_text::transform::remove_special_characters(text);
/// assert_eq!(result, "hello world");
/// ```
pub fn remove_special_characters(text: &str) -> String {
    text.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect()
}

/// Converts a string to title case.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the title case text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::to_title_case(text);
/// assert_eq!(result, "Hello World");
/// ```
pub fn to_title_case(text: &str) -> String {
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

    #[test]
    fn test_to_camel_case() {
        let text = "hello world";
        let result = to_camel_case(text);
        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn test_to_snake_case() {
        let text = "Hello World";
        let result = to_snake_case(text);
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_to_kebab_case() {
        let text = "Hello World";
        let result = to_kebab_case(text);
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_replace_spaces_with_underscores() {
        let text = "hello world";
        let result = replace_spaces_with_underscores(text);
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_reverse_words() {
        let text = "hello world";
        let result = reverse_words(text);
        assert_eq!(result, "world hello");
    }

    #[test]
    fn test_remove_special_characters() {
        let text = "hello, world!";
        let result = remove_special_characters(text);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_to_title_case() {
        let text = "hello world";
        let result = to_title_case(text);
        assert_eq!(result, "Hello World");
    }
}

