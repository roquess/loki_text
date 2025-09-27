use regex::Regex;
use std::collections::HashMap;

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

/// Converts a string to PascalCase.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the PascalCase text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::to_pascal_case(text);
/// assert_eq!(result, "HelloWorld");
/// ```
pub fn to_pascal_case(text: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

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

/// Converts a string to SCREAMING_SNAKE_CASE.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the SCREAMING_SNAKE_CASE text.
///
/// # Examples
///
/// ```
/// let text = "Hello World";
/// let result = loki_text::transform::to_screaming_snake_case(text);
/// assert_eq!(result, "HELLO_WORLD");
/// ```
pub fn to_screaming_snake_case(text: &str) -> String {
    to_snake_case(text).to_uppercase()
}

/// Converts a string to alternating case.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the alternating case text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::to_alternating_case(text);
/// assert_eq!(result, "hElLo WoRlD");
/// ```
pub fn to_alternating_case(text: &str) -> String {
    let mut result = String::new();
    let mut letter_index = 0;

    for c in text.chars() {
        if c.is_alphabetic() {
            if letter_index % 2 == 0 {
                result.push_str(&c.to_lowercase().collect::<String>());
            } else {
                result.push_str(&c.to_uppercase().collect::<String>());
            }
            letter_index += 1;
        } else {
            result.push(c);
        }
    }

    result
}

/// Inverts the case of each character in a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to invert.
///
/// # Returns
///
/// * A `String` containing the inverted case text.
///
/// # Examples
///
/// ```
/// let text = "Hello World";
/// let result = loki_text::transform::invert_case(text);
/// assert_eq!(result, "hELLO wORLD");
/// ```
pub fn invert_case(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

/// Normalizes whitespace by replacing multiple spaces with single spaces.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to normalize.
///
/// # Returns
///
/// * A `String` containing the normalized text.
///
/// # Examples
///
/// ```
/// let text = "hello    world   test";
/// let result = loki_text::transform::normalize_whitespace(text);
/// assert_eq!(result, "hello world test");
/// ```
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Truncates a string to a maximum length.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to truncate.
/// * `max_length` - The maximum length of the resulting string.
///
/// # Returns
///
/// * A `String` containing the truncated text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::truncate(text, 5);
/// assert_eq!(result, "hello");
/// ```
pub fn truncate(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        text.chars().take(max_length).collect()

    }
}

/// Repeats each character in a string n times.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
/// * `times` - The number of times to repeat each character.
///
/// # Returns
///
/// * A `String` containing the text with repeated characters.
///
/// # Examples
///
/// ```
/// let text = "hello";
/// let result = loki_text::transform::repeat_chars(text, 2);
/// assert_eq!(result, "hheelllloo");
/// ```
pub fn repeat_chars(text: &str, times: usize) -> String {
    text.chars()
        .map(|c| c.to_string().repeat(times))
        .collect()
}

/// Removes all vowels from a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
///
/// # Returns
///
/// * A `String` containing the text without vowels.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::remove_vowels(text);
/// assert_eq!(result, "hll wrld");
/// ```
pub fn remove_vowels(text: &str) -> String {
    text.chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect()
}

/// Removes all consonants from a string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
///
/// # Returns
///
/// * A `String` containing the text without consonants.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::remove_consonants(text);
/// assert_eq!(result, "eo o");
/// ```
pub fn remove_consonants(text: &str) -> String {
    text.chars()
        .filter(|c| "aeiouAEIOU ".contains(*c) || !c.is_alphabetic())
        .collect()
}

/// Converts text to basic leetspeak.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the leetspeak text.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::transform::to_leet_speak(text);
/// assert_eq!(result, "h3ll0 w0rld");
/// ```
pub fn to_leet_speak(text: &str) -> String {
    text.chars()
        .map(|c| match c.to_ascii_lowercase() {
            'a' => '4',
            'e' => '3',
            'i' => '1',
            'o' => '0',
            's' => '5',
            't' => '7',
            _ => c,
        })
        .collect()
}


/// Removes accents and diacritics from text (basic ASCII transliteration).
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to process.
///
/// # Returns
///
/// * A `String` containing the text without accents.
///
/// # Examples
///
/// ```
/// let text = "Café, naïve, résumé";
/// let result = loki_text::transform::remove_accents(text);
/// assert_eq!(result, "Cafe, naive, resume");
/// ```
pub fn remove_accents(text: &str) -> String {
    let accent_map: HashMap<char, char> = [
        ('à', 'a'), ('á', 'a'), ('â', 'a'), ('ã', 'a'), ('ä', 'a'), ('å', 'a'),
        ('è', 'e'), ('é', 'e'), ('ê', 'e'), ('ë', 'e'),
        ('ì', 'i'), ('í', 'i'), ('î', 'i'), ('ï', 'i'),
        ('ò', 'o'), ('ó', 'o'), ('ô', 'o'), ('õ', 'o'), ('ö', 'o'),
        ('ù', 'u'), ('ú', 'u'), ('û', 'u'), ('ü', 'u'),
        ('ý', 'y'), ('ÿ', 'y'),
        ('ñ', 'n'), ('ç', 'c'),
        ('À', 'A'), ('Á', 'A'), ('Â', 'A'), ('Ã', 'A'), ('Ä', 'A'), ('Å', 'A'),
        ('È', 'E'), ('É', 'E'), ('Ê', 'E'), ('Ë', 'E'),
        ('Ì', 'I'), ('Í', 'I'), ('Î', 'I'), ('Ï', 'I'),
        ('Ò', 'O'), ('Ó', 'O'), ('Ô', 'O'), ('Õ', 'O'), ('Ö', 'O'),
        ('Ù', 'U'), ('Ú', 'U'), ('Û', 'U'), ('Ü', 'U'),
        ('Ý', 'Y'), ('Ÿ', 'Y'),
        ('Ñ', 'N'), ('Ç', 'C'),
    ].iter().cloned().collect();

    text.chars()
        .map(|c| *accent_map.get(&c).unwrap_or(&c))
        .collect()
}

/// Generates a URL-friendly slug from text.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to convert.
///
/// # Returns
///
/// * A `String` containing the slug.
///
/// # Examples
///
/// ```
/// let text = "Hello, World! This is a Test.";
/// let result = loki_text::transform::to_slug(text);
/// assert_eq!(result, "hello-world-this-is-a-test");
/// ```
pub fn to_slug(text: &str) -> String {
    let cleaned = remove_accents(text);
    let re = Regex::new(r"[^a-zA-Z0-9\s-]").unwrap();
    let without_special = re.replace_all(&cleaned, "");
    
    without_special
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
}

/// Calculates the Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
///
/// # Returns
///
/// * A `usize` representing the Levenshtein distance.
///
/// # Examples
///
/// ```
/// let result = loki_text::transform::levenshtein_distance("kitten", "sitting");
/// assert_eq!(result, 3);
/// ```
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    if len1 == 0 { return len2; }
    if len2 == 0 { return len1; }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 { matrix[i][0] = i; }
    for j in 0..=len2 { matrix[0][j] = j; }
    
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1       // insertion
                ),
                matrix[i - 1][j - 1] + cost    // substitution
            );
        }
    }
    
    matrix[len1][len2]
}

/// Calculates string similarity based on Levenshtein distance (0.0 to 1.0).
///
/// # Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
///
/// # Returns
///
/// * A `f64` representing similarity (1.0 = identical, 0.0 = completely different).
///
/// # Examples
///
/// ```
/// let result = loki_text::transform::similarity("kitten", "sitting");
/// assert!((result - 0.57).abs() < 0.01); // approximately 0.57
/// ```
pub fn similarity(s1: &str, s2: &str) -> f64 {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let max_len = std::cmp::max(len1, len2);
    
    if max_len == 0 { return 1.0; }
    
    let distance = levenshtein_distance(s1, s2);
    1.0 - (distance as f64 / max_len as f64)
}

/// Implements the Soundex phonetic algorithm.
///
/// # Arguments
///
/// * `text` - A string slice to convert to Soundex code.
///
/// # Returns
///
/// * A `String` containing the Soundex code (4 characters).
///
/// # Examples
///
/// ```
/// let result = loki_text::transform::soundex("Robert");
/// assert_eq!(result, "R163");
/// ```
pub fn soundex(text: &str) -> String {
    if text.is_empty() { return "0000".to_string(); }
    
    let chars: Vec<char> = text.to_uppercase().chars().collect();
    let first_char = chars[0];
    
    if !first_char.is_alphabetic() { return "0000".to_string(); }
    
    let mut code = vec![first_char];
    let mut prev_code = get_soundex_code(first_char);
    
    for &ch in chars.iter().skip(1) {
        let current_code = get_soundex_code(ch);
        if current_code != '0' && current_code != prev_code {
            code.push(current_code);
            if code.len() == 4 { break; }
        }
        if ch.is_alphabetic() { prev_code = current_code; }
    }
    
    while code.len() < 4 { code.push('0'); }
    
    code.iter().collect()
}

fn get_soundex_code(ch: char) -> char {
    match ch.to_ascii_uppercase() {
        'B' | 'F' | 'P' | 'V' => '1',
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
        'D' | 'T' => '3',
        'L' => '4',
        'M' | 'N' => '5',
        'R' => '6',
        _ => '0',
    }
}

/// Wraps text to a specified line width.
///
/// # Arguments
///
/// * `text` - A string slice to wrap.
/// * `width` - The maximum line width.
///
/// # Returns
///
/// * A `String` with lines wrapped to the specified width.
///
/// # Examples
///
/// ```
/// let text = "This is a very long line that should be wrapped";
/// let result = loki_text::transform::wrap_text(text, 20);
/// ```
pub fn wrap_text(text: &str, width: usize) -> String {
    if width == 0 { return text.to_string(); }
    
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines.join("\n")
}

/// Truncates text with an ellipsis if it exceeds the maximum length.
///
/// # Arguments
///
/// * `text` - A string slice to truncate.
/// * `max_length` - The maximum length including ellipsis.
///
/// # Returns
///
/// * A `String` truncated with ellipsis if necessary.
///
/// # Examples
///
/// ```
/// let text = "This is a long sentence";
/// let result = loki_text::transform::truncate_with_ellipsis(text, 10);
/// assert_eq!(result, "This is...");
/// ```
pub fn truncate_with_ellipsis(text: &str, max_length: usize) -> String {
    if text.chars().count() <= max_length {
        text.to_string()
    } else if max_length <= 3 {
        "...".chars().take(max_length).collect()
    } else {
        let truncated: String = text.chars().take(max_length - 3).collect();
        format!("{}...", truncated)
    }
}

/// Converts text to proper title case with exceptions for common articles and prepositions.
///
/// # Arguments
///
/// * `text` - A string slice to convert.
///
/// # Returns
///
/// * A `String` in proper title case.
///
/// # Examples
///
/// ```
/// let text = "the lord of the rings";
/// let result = loki_text::transform::to_proper_title_case(text);
/// assert_eq!(result, "The Lord of the Rings");
/// ```
pub fn to_proper_title_case(text: &str) -> String {
    let small_words = ["a", "an", "and", "as", "at", "but", "by", "for", "if", "in", 
                      "is", "it", "nor", "of", "on", "or", "so", "the", "to", "up"];
    
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut result = Vec::new();
    
    for (i, word) in words.iter().enumerate() {
        let lower_word = word.to_lowercase();
        if i == 0 || i == words.len() - 1 || !small_words.contains(&lower_word.as_str()) {
            result.push(capitalize_first_letter(word));
        } else {
            result.push(lower_word);
        }
    }
    
    result.join(" ")
}

fn capitalize_first_letter(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

/// Extracts initials from a name.
///
/// # Arguments
///
/// * `name` - A string slice containing a name.
///
/// # Returns
///
/// * A `String` containing the initials.
///
/// # Examples
///
/// ```
/// let name = "John Doe Smith";
/// let result = loki_text::transform::extract_initials(name);
/// assert_eq!(result, "J.D.S.");
/// ```
pub fn extract_initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| format!("{}.", c.to_uppercase()))
        .collect()
}

/// Counts words in a text.
///
/// # Arguments
///
/// * `text` - A string slice to count words in.
///
/// # Returns
///
/// * A `usize` representing the word count.
///
/// # Examples
///
/// ```
/// let text = "Hello world, this is a test";
/// let result = loki_text::transform::word_count(text);
/// assert_eq!(result, 6);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Extracts email addresses from text.
///
/// # Arguments
///
/// * `text` - A string slice to extract emails from.
///
/// # Returns
///
/// * A `Vec<String>` containing the extracted email addresses.
///
/// # Examples
///
/// ```
/// let text = "Contact us at john@example.com or support@test.org";
/// let result = loki_text::transform::extract_emails(text);
/// assert_eq!(result, vec!["john@example.com", "support@test.org"]);
/// ```
pub fn extract_emails(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
    re.find_iter(text).map(|mat| mat.as_str().to_string()).collect()
}

/// Extracts URLs from text.
///
/// # Arguments
///
/// * `text` - A string slice to extract URLs from.
///
/// # Returns
///
/// * A `Vec<String>` containing the extracted URLs.
///
/// # Examples
///
/// ```
/// let text = "Visit https://example.com or http://test.org";
/// let result = loki_text::transform::extract_urls(text);
/// assert_eq!(result, vec!["https://example.com", "http://test.org"]);
/// ```
pub fn extract_urls(text: &str) -> Vec<String> {
    let re = Regex::new(r"https?://[^\s]+").unwrap();
    re.find_iter(text).map(|mat| mat.as_str().to_string()).collect()
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

    #[test]
    fn test_to_pascal_case() {
        let text = "hello world";
        assert_eq!(to_pascal_case(text), "HelloWorld");
        
        let text = "hello_world_test";
        assert_eq!(to_pascal_case(text), "HelloWorldTest");
    }

    #[test]
    fn test_to_screaming_snake_case() {
        let text = "Hello World";
        assert_eq!(to_screaming_snake_case(text), "HELLO_WORLD");
        
        let text = "camelCaseText";
        assert_eq!(to_screaming_snake_case(text), "CAMEL_CASE_TEXT");
    }

    #[test]
    fn test_to_alternating_case() {
        let text = "hello";
        assert_eq!(to_alternating_case(text), "hElLo");
        
        let text = "test";
        assert_eq!(to_alternating_case(text), "tEsT");
    }

    #[test]
    fn test_invert_case() {
        let text = "Hello World";
        assert_eq!(invert_case(text), "hELLO wORLD");
        
        let text = "tEST";
        assert_eq!(invert_case(text), "Test");
    }

    #[test]
    fn test_normalize_whitespace() {
        let text = "hello    world   test";
        assert_eq!(normalize_whitespace(text), "hello world test");
        
        let text = "  hello world  ";
        assert_eq!(normalize_whitespace(text), "hello world");
    }

    #[test]
    fn test_truncate() {
        let text = "hello world";
        assert_eq!(truncate(text, 5), "hello");
        
        let text = "hi";
        assert_eq!(truncate(text, 5), "hi");
    }

    #[test]
    fn test_repeat_chars() {
        let text = "hello";
        assert_eq!(repeat_chars(text, 2), "hheelllloo");
        
        let text = "hi";
        assert_eq!(repeat_chars(text, 3), "hhhiii");
    }

    #[test]
    fn test_remove_vowels() {
        let text = "hello world";
        assert_eq!(remove_vowels(text), "hll wrld");
        
        let text = "AEIOU";
        assert_eq!(remove_vowels(text), "");
    }

    #[test]
    fn test_remove_consonants() {
        let text = "hello world";
        assert_eq!(remove_consonants(text), "eo o");
        
        let text = "bcdfg";
        assert_eq!(remove_consonants(text), "");
    }

    #[test]
    fn test_to_leet_speak() {
        let text = "hello world";
        assert_eq!(to_leet_speak(text), "h3ll0 w0rld");
        
        let text = "test";
        assert_eq!(to_leet_speak(text), "7357");
    }

    #[test]
    fn test_remove_accents() {
        assert_eq!(remove_accents("Café"), "Cafe");
        assert_eq!(remove_accents("naïve"), "naive");
        assert_eq!(remove_accents("résumé"), "resume");
    }

    #[test]
    fn test_to_slug() {
        assert_eq!(to_slug("Hello, World!"), "hello-world");
        assert_eq!(to_slug("This is a Test"), "this-is-a-test");
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }

    #[test]
    fn test_similarity() {
        let sim = similarity("kitten", "kitten");
        assert!((sim - 1.0).abs() < 0.01);
        
        let sim = similarity("", "");
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_soundex() {
        assert_eq!(soundex("Robert"), "R163");
        assert_eq!(soundex("Rupert"), "R163");
        assert_eq!(soundex(""), "0000");
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a test";
        let result = wrap_text(text, 10);
        assert!(result.contains("\n"));
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("Hello World", 8), "Hello...");
        assert_eq!(truncate_with_ellipsis("Hi", 10), "Hi");
    }

    #[test]
    fn test_proper_title_case() {
        assert_eq!(to_proper_title_case("the lord of the rings"), "The Lord of the Rings");
    }

    #[test]
    fn test_extract_initials() {
        assert_eq!(extract_initials("John Doe"), "J.D.");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("Hello world test"), 3);
    }

    #[test]
    fn test_extract_emails() {
        let text = "Email me at test@example.com";
        assert_eq!(extract_emails(text), vec!["test@example.com"]);
    }

    #[test]
    fn test_extract_urls() {
        let text = "Visit https://example.com";
        assert_eq!(extract_urls(text), vec!["https://example.com"]);
    }
}

