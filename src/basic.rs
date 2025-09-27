use regex::Regex;
use std::collections::HashMap;

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

/// Validates if a string is a valid email address.
///
/// # Arguments
///
/// * `email` - A string slice to validate.
///
/// # Returns
///
/// * A `bool` indicating whether the email is valid.
///
/// # Examples
///
/// ```
/// assert_eq!(loki_text::basic::is_valid_email("test@example.com"), true);
/// assert_eq!(loki_text::basic::is_valid_email("invalid-email"), false);
/// ```
pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    email_regex.is_match(email)
}

/// Validates if a string is a valid URL.
///
/// # Arguments
///
/// * `url` - A string slice to validate.
///
/// # Returns
///
/// * A `bool` indicating whether the URL is valid.
///
/// # Examples
///
/// ```
/// assert_eq!(loki_text::basic::is_valid_url("https://example.com"), true);
/// assert_eq!(loki_text::basic::is_valid_url("not-a-url"), false);
/// ```
pub fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(
        r"^https?://[^\s/$.?#].[^\s]*$"
    ).unwrap();
    url_regex.is_match(url)
}

/// Validates if a string contains only digits.
///
/// # Arguments
///
/// * `text` - A string slice to validate.
///
/// # Returns
///
/// * A `bool` indicating whether the text contains only digits.
pub fn is_numeric(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c.is_ascii_digit())
}

/// Validates if a string contains only alphabetic characters.
///
/// # Arguments
///
/// * `text` - A string slice to validate.
///
/// # Returns
///
/// * A `bool` indicating whether the text contains only letters.
pub fn is_alphabetic(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c.is_alphabetic())
}

/// Validates if a string contains only alphanumeric characters.
///
/// # Arguments
///
/// * `text` - A string slice to validate.
///
/// # Returns
///
/// * A `bool` indicating whether the text is alphanumeric.
pub fn is_alphanumeric(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c.is_alphanumeric())
}

/// Calculates basic text statistics.
///
/// # Arguments
///
/// * `text` - A string slice to analyze.
///
/// # Returns
///
/// * A `TextStats` struct containing various metrics.
///
/// # Examples
///
/// ```
/// let stats = loki_text::basic::text_statistics("Hello world!");
/// assert_eq!(stats.character_count, 12);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TextStats {
    pub character_count: usize,
    pub word_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub line_count: usize,
    pub average_word_length: f64,
    pub reading_time_minutes: f64,
}

pub fn text_statistics(text: &str) -> TextStats {
    let character_count = text.chars().count();
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();
    
    let sentence_count = text.matches(&['.', '!', '?'][..]).count();
    let paragraph_count = text.split("\n\n").filter(|p| !p.trim().is_empty()).count();
    let line_count = text.lines().count();
    
    let average_word_length = if word_count > 0 {
        words.iter().map(|w| w.len()).sum::<usize>() as f64 / word_count as f64
    } else {
        0.0
    };
    
    // Assuming average reading speed of 250 words per minute
    let reading_time_minutes = word_count as f64 / 250.0;
    
    TextStats {
        character_count,
        word_count,
        sentence_count: sentence_count.max(1),
        paragraph_count: paragraph_count.max(1),
        line_count: line_count.max(1),
        average_word_length,
        reading_time_minutes,
    }
}

/// Calculates the readability score using Flesch Reading Ease formula.
///
/// # Arguments
///
/// * `text` - A string slice to analyze.
///
/// # Returns
///
/// * A `f64` representing the readability score (0-100, higher = easier).
pub fn readability_score(text: &str) -> f64 {
    let stats = text_statistics(text);
    
    if stats.sentence_count == 0 || stats.word_count == 0 {
        return 0.0;
    }
    
    let avg_sentence_length = stats.word_count as f64 / stats.sentence_count as f64;
    let syllable_count = count_syllables(text);
    let avg_syllables_per_word = syllable_count as f64 / stats.word_count as f64;
    
    // Flesch Reading Ease formula
    206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word)
}

fn count_syllables(text: &str) -> usize {
    let words: Vec<&str> = text.split_whitespace().collect();
    words.iter().map(|word| count_word_syllables(word)).sum()
}

fn count_word_syllables(word: &str) -> usize {
    let word = word.to_lowercase();
    let vowels = "aeiouy";
    let mut syllables = 0;
    let mut prev_was_vowel = false;
    
    for ch in word.chars() {
        let is_vowel = vowels.contains(ch);
        if is_vowel && !prev_was_vowel {
            syllables += 1;
        }
        prev_was_vowel = is_vowel;
    }
    
    // Handle silent 'e'
    if word.ends_with('e') && syllables > 1 {
        syllables -= 1;
    }
    
    syllables.max(1)
}

/// Cleans text by removing extra whitespace, special characters, etc.
///
/// # Arguments
///
/// * `text` - A string slice to clean.
///
/// # Returns
///
/// * A `String` containing the cleaned text.
pub fn clean_text(text: &str) -> String {
    // Remove extra whitespace
    let cleaned = text.split_whitespace().collect::<Vec<&str>>().join(" ");
    
    // Remove common unwanted characters but keep punctuation
    let re = Regex::new(r"[^\w\s\.,!?;:'\x22()-]").unwrap();
    re.replace_all(&cleaned, "").to_string()
}

/// Extracts text from simple HTML by removing tags.
///
/// # Arguments
///
/// * `html` - A string slice containing HTML.
///
/// # Returns
///
/// * A `String` containing the extracted text.
pub fn extract_text_from_html(html: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    let without_tags = re.replace_all(html, " ");
    
    // Decode common HTML entities
    let decoded = without_tags
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\x22")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");
    
    // Clean up extra whitespace
    decoded.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Generates random text of specified length using given characters.
///
/// # Arguments
///
/// * `length` - Length of the text to generate.
/// * `charset` - Characters to use for generation.
///
/// # Returns
///
/// * A `String` containing random text.
pub fn generate_random_text(length: usize, charset: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let chars: Vec<char> = charset.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    
    let mut result = String::with_capacity(length);
    let mut seed = {
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().hash(&mut hasher);
        hasher.finish()
    };
    
    for _ in 0..length {
        // Simple linear congruential generator
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let index = (seed as usize) % chars.len();
        result.push(chars[index]);
    }
    
    result
}

/// Generates Lorem Ipsum placeholder text.
///
/// # Arguments
///
/// * `word_count` - Number of words to generate.
///
/// # Returns
///
/// * A `String` containing Lorem Ipsum text.
pub fn generate_lorem_ipsum(word_count: usize) -> String {
    let lorem_words = [
        "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
        "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
        "magna", "aliqua", "enim", "ad", "minim", "veniam", "quis", "nostrud",
        "exercitation", "ullamco", "laboris", "nisi", "aliquip", "ex", "ea", "commodo",
        "consequat", "duis", "aute", "irure", "in", "reprehenderit", "voluptate",
        "velit", "esse", "cillum", "fugiat", "nulla", "pariatur", "excepteur", "sint",
        "occaecat", "cupidatat", "non", "proident", "sunt", "culpa", "qui", "officia",
        "deserunt", "mollit", "anim", "id", "est", "laborum"
    ];
    
    let mut result = Vec::new();
    for i in 0..word_count {
        result.push(lorem_words[i % lorem_words.len()]);
    }
    
    result.join(" ")
}

/// Calculates the entropy of text (measure of randomness).
///
/// # Arguments
///
/// * `text` - A string slice to analyze.
///
/// # Returns
///
/// * A `f64` representing the entropy value.
pub fn calculate_entropy(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }
    
    let mut char_counts = HashMap::new();
    let total_chars = text.chars().count();
    
    for ch in text.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }
    
    let mut entropy = 0.0;
    for &count in char_counts.values() {
        let probability = count as f64 / total_chars as f64;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }
    
    entropy
}

/// Finds common prefixes between multiple strings.
///
/// # Arguments
///
/// * `strings` - A slice of string slices to compare.
///
/// # Returns
///
/// * A `String` containing the longest common prefix.
pub fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    if strings.len() == 1 {
        return strings[0].to_string();
    }
    
    let first = strings[0];
    let mut prefix_length = 0;
    
    for (i, ch) in first.chars().enumerate() {
        if strings[1..].iter().all(|s| {
            s.chars().nth(i).map_or(false, |c| c == ch)
        }) {
            prefix_length = i + 1;
        } else {
            break;
        }
    }
    
    first.chars().take(prefix_length).collect()
}

/// Splits text into sentences using common sentence terminators.
///
/// # Arguments
///
/// * `text` - A string slice to split into sentences.
///
/// # Returns
///
/// * A `Vec<String>` containing the sentences.
pub fn split_into_sentences(text: &str) -> Vec<String> {
    let sentence_regex = Regex::new(r"[.!?]+\s+").unwrap();
    sentence_regex
        .split(text)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

/// Removes duplicate lines from text while preserving order.
///
/// # Arguments
///
/// * `text` - A string slice containing multiple lines.
///
/// # Returns
///
/// * A `String` with duplicate lines removed.
pub fn remove_duplicate_lines(text: &str) -> String {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    
    for line in text.lines() {
        if seen.insert(line.to_string()) {
            result.push(line);
        }
    }
    
    result.join("\n")
}

/// Sorts lines of text alphabetically.
///
/// # Arguments
///
/// * `text` - A string slice containing multiple lines.
/// * `reverse` - Whether to sort in reverse order.
///
/// # Returns
///
/// * A `String` with lines sorted alphabetically.
pub fn sort_lines(text: &str, reverse: bool) -> String {
    let mut lines: Vec<&str> = text.lines().collect();
    if reverse {
        lines.sort_by(|a, b| b.cmp(a));
    } else {
        lines.sort();
    }
    lines.join("\n")
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

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@domain.com"));
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://test.org/path?query=1"));
        assert!(!is_valid_url("not-a-url"));
        assert!(!is_valid_url("ftp://example.com"));
    }

    #[test]
    fn test_validation_functions() {
        assert!(is_numeric("12345"));
        assert!(!is_numeric("123a5"));
        assert!(is_alphabetic("hello"));
        assert!(!is_alphabetic("hello123"));
        assert!(is_alphanumeric("hello123"));
        assert!(!is_alphanumeric("hello-123"));
    }

    #[test]
    fn test_clean_text() {
        let cleaned = clean_text("Hello,   world!   Extra   spaces.");
        assert_eq!(cleaned, "Hello, world! Extra spaces.");
    }

    #[test]
    fn test_extract_text_from_html() {
        let html = "<p>Hello <strong>world</strong>!</p>";
        assert_eq!(extract_text_from_html(html), "Hello world !");
    }

    #[test]
    fn test_generate_lorem_ipsum() {
        let lorem = generate_lorem_ipsum(5);
        assert_eq!(lorem, "lorem ipsum dolor sit amet");
    }

    #[test]
    fn test_calculate_entropy() {
        let entropy1 = calculate_entropy("aaaa");
        let entropy2 = calculate_entropy("abcd");
        assert!(entropy2 > entropy1); // More random text has higher entropy
    }

    #[test]
    fn test_longest_common_prefix() {
        let strings = ["prefix123", "prefix456", "prefix789"];
        assert_eq!(longest_common_prefix(&strings), "prefix");
    }

    #[test]
    fn test_split_into_sentences() {
        let text = "First sentence. Second sentence! Third sentence?";
        let sentences = split_into_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "First sentence");
    }

    #[test]
    fn test_sort_lines() {
        let text = "zebra\napple\nbanana";
        let sorted = sort_lines(text, false);
        assert_eq!(sorted, "apple\nbanana\nzebra");
    }
}

