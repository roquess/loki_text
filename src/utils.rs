use std::collections::HashMap;
use std::fs;
use std::io::{self, Write, BufReader, BufRead};
use std::path::Path;

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

/// Represents text processing performance metrics.
#[derive(Debug, Clone)]
pub struct ProcessingMetrics {
    pub duration_ms: u128,
    pub input_size: usize,
    pub output_size: usize,
    pub throughput_mb_per_second: f64,
}

/// Measures the performance of a text processing operation.
///
/// # Arguments
///
/// * `operation` - A closure that performs text processing.
/// * `input` - The input text to measure.
///
/// # Returns
///
/// * A `ProcessingMetrics` struct containing performance data.
///
/// # Examples
///
/// ```
/// let metrics = loki_text::utils::measure_performance(
///     |text| text.to_uppercase(),
///     "hello world"
/// );
/// ```
pub fn measure_performance<F, T>(operation: F, input: &str) -> (T, ProcessingMetrics)
where
    F: FnOnce(&str) -> T,
{
    let start = std::time::Instant::now();
    let result = operation(input);
    let duration = start.elapsed();
    
    let input_size = input.len();
    let output_size = std::mem::size_of_val(&result);
    let duration_ms = duration.as_millis();
    let throughput = if duration_ms > 0 {
        (input_size as f64 / 1_000_000.0) / (duration_ms as f64 / 1000.0)
    } else {
        0.0
    };
    
    let metrics = ProcessingMetrics {
        duration_ms,
        input_size,
        output_size,
        throughput_mb_per_second: throughput,
    };
    
    (result, metrics)
}

/// Writes text to a file with proper error handling.
///
/// # Arguments
///
/// * `path` - Path to the file to write.
/// * `content` - Text content to write.
///
/// # Returns
///
/// * A `Result<(), io::Error>` indicating success or failure.
pub fn write_text_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// Appends text to a file.
///
/// # Arguments
///
/// * `path` - Path to the file to append to.
/// * `content` - Text content to append.
///
/// # Returns
///
/// * A `Result<(), io::Error>` indicating success or failure.
pub fn append_text_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    
    file.write_all(content.as_bytes())
}

/// Processes a text file line by line for memory efficiency.
///
/// # Arguments
///
/// * `path` - Path to the file to process.
/// * `processor` - A closure that processes each line.
///
/// # Returns
///
/// * A `Result<Vec<T>, io::Error>` containing processed results.
pub fn process_file_lines<P, F, T>(path: P, mut processor: F) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    F: FnMut(&str) -> T,
{
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        results.push(processor(&line));
    }
    
    Ok(results)
}

/// Safely truncates text to a maximum byte size, ensuring UTF-8 validity.
///
/// # Arguments
///
/// * `text` - Text to truncate.
/// * `max_bytes` - Maximum number of bytes.
///
/// # Returns
///
/// * A `String` truncated to the maximum byte size.
pub fn safe_truncate_bytes(text: &str, max_bytes: usize) -> String {
    if text.len() <= max_bytes {
        return text.to_string();
    }
    
    // Find the last valid UTF-8 boundary
    let mut boundary = max_bytes;
    while boundary > 0 && !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    
    text[..boundary].to_string()
}

/// Splits text into chunks of approximately equal size.
///
/// # Arguments
///
/// * `text` - Text to split.
/// * `chunk_size` - Approximate size of each chunk.
///
/// # Returns
///
/// * A `Vec<String>` containing the text chunks.
pub fn chunk_text(text: &str, chunk_size: usize) -> Vec<String> {
    if chunk_size == 0 || text.is_empty() {
        return vec![text.to_string()];
    }
    
    let mut chunks = Vec::new();
    let mut start = 0;
    
    while start < text.len() {
        let mut end = std::cmp::min(start + chunk_size, text.len());
        
        // Adjust to word boundary if possible
        if end < text.len() {
            while end > start && !text.chars().nth(end).unwrap_or(' ').is_whitespace() {
                end -= 1;
            }
            if end == start {
                end = std::cmp::min(start + chunk_size, text.len());
            }
        }
        
        chunks.push(text[start..end].trim().to_string());
        start = end;
        
        // Skip whitespace
        while start < text.len() && text.chars().nth(start).unwrap_or('\0').is_whitespace() {
            start += 1;
        }
    }
    
    chunks
}

/// Creates a frequency map of words in text.
///
/// # Arguments
///
/// * `text` - Text to analyze.
///
/// # Returns
///
/// * A `HashMap<String, usize>` containing word frequencies.
pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut frequencies = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let cleaned_word = word.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        
        if !cleaned_word.is_empty() {
            *frequencies.entry(cleaned_word).or_insert(0) += 1;
        }
    }
    
    frequencies
}

/// Finds the most common words in text.
///
/// # Arguments
///
/// * `text` - Text to analyze.
/// * `limit` - Maximum number of words to return.
///
/// # Returns
///
/// * A `Vec<(String, usize)>` containing words and their frequencies, sorted by frequency.
pub fn most_common_words(text: &str, limit: usize) -> Vec<(String, usize)> {
    let frequencies = word_frequency(text);
    let mut word_counts: Vec<(String, usize)> = frequencies.into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1));
    word_counts.truncate(limit);
    word_counts
}

/// Compares two texts and returns similarity metrics.
///
/// # Arguments
///
/// * `text1` - First text to compare.
/// * `text2` - Second text to compare.
///
/// # Returns
///
/// * A `TextSimilarity` struct containing various similarity metrics.
#[derive(Debug, Clone)]
pub struct TextSimilarity {
    pub jaccard_similarity: f64,
    pub word_overlap_ratio: f64,
    pub character_similarity: f64,
    pub common_words: Vec<String>,
}

pub fn compare_texts(text1: &str, text2: &str) -> TextSimilarity {
    let words1: std::collections::HashSet<String> = text1
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();
    
    let words2: std::collections::HashSet<String> = text2
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect();
    
    let intersection: std::collections::HashSet<_> = words1.intersection(&words2).collect();
    let union: std::collections::HashSet<_> = words1.union(&words2).collect();
    
    let jaccard_similarity = if union.is_empty() {
        0.0
    } else {
        intersection.len() as f64 / union.len() as f64
    };
    
    let word_overlap_ratio = if words1.len() + words2.len() == 0 {
        0.0
    } else {
        (intersection.len() * 2) as f64 / (words1.len() + words2.len()) as f64
    };
    
    // Character-level similarity (simplified)
    let chars1: std::collections::HashSet<char> = text1.chars().collect();
    let chars2: std::collections::HashSet<char> = text2.chars().collect();
    let char_intersection: std::collections::HashSet<_> = chars1.intersection(&chars2).collect();
    let char_union: std::collections::HashSet<_> = chars1.union(&chars2).collect();
    
    let character_similarity = if char_union.is_empty() {
        0.0
    } else {
        char_intersection.len() as f64 / char_union.len() as f64
    };
    
    let common_words: Vec<String> = intersection.into_iter().cloned().collect();
    
    TextSimilarity {
        jaccard_similarity,
        word_overlap_ratio,
        character_similarity,
        common_words,
    }
}

/// Normalizes Unicode text using different normalization forms.
///
/// # Arguments
///
/// * `text` - Text to normalize.
/// * `form` - Normalization form ("NFC", "NFD", "NFKC", "NFKD").
///
/// # Returns
///
/// * A `String` containing normalized text (basic implementation).
pub fn normalize_unicode(text: &str, form: &str) -> String {
    match form {
        "NFC" => text.chars().collect(), // Simplified - real implementation would use proper Unicode normalization
        "NFD" => text.chars().collect(), // Simplified
        "NFKC" => text.chars().collect(), // Simplified
        "NFKD" => text.chars().collect(), // Simplified
        _ => text.to_string(),
    }
}

/// Escapes text for safe inclusion in various contexts.
///
/// # Arguments
///
/// * `text` - Text to escape.
/// * `context` - Escape context ("html", "json", "csv", "regex").
///
/// # Returns
///
/// * A `String` containing escaped text.
pub fn escape_text(text: &str, context: &str) -> String {
    match context {
        "html" => text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;"),
        "json" => text
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t"),
        "csv" => {
            if text.contains(',') || text.contains('"') || text.contains('\n') {
                format!("\"{}\"", text.replace('"', "\"\""))
            } else {
                text.to_string()
            }
        }
        "regex" => text
            .replace('\\', "\\\\")
            .replace('.', "\\.")
            .replace('^', "\\^")
            .replace('$', "\\$")
            .replace('*', "\\*")
            .replace('+', "\\+")
            .replace('?', "\\?")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('|', "\\|"),
        _ => text.to_string(),
    }
}

/// Converts text encoding between different character sets (basic implementation).
///
/// # Arguments
///
/// * `text` - Text to convert.
/// * `from_encoding` - Source encoding.
/// * `to_encoding` - Target encoding.
///
/// # Returns
///
/// * A `Result<String, String>` containing converted text or error.
pub fn convert_encoding(text: &str, from_encoding: &str, to_encoding: &str) -> Result<String, String> {
    // Basic implementation - in practice you'd use a proper encoding library
    match (from_encoding.to_lowercase().as_str(), to_encoding.to_lowercase().as_str()) {
        ("utf-8", "ascii") => {
            if text.is_ascii() {
                Ok(text.to_string())
            } else {
                Err("Text contains non-ASCII characters".to_string())
            }
        }
        ("ascii", "utf-8") => Ok(text.to_string()),
        (from, to) if from == to => Ok(text.to_string()),
        _ => Err(format!("Conversion from {} to {} not supported", from_encoding, to_encoding)),
    }
}

/// Validates text against common patterns and constraints.
///
/// # Arguments
///
/// * `text` - Text to validate.
/// * `rules` - Validation rules to apply.
///
/// # Returns
///
/// * A `ValidationResult` containing validation status and errors.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationRules {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub required_patterns: Vec<String>,
    pub forbidden_patterns: Vec<String>,
    pub allowed_chars: Option<String>,
}

pub fn validate_text(text: &str, rules: &ValidationRules) -> ValidationResult {
    let mut errors = Vec::new();
    
    if let Some(min_len) = rules.min_length {
        if text.len() < min_len {
            errors.push(format!("Text too short (minimum {} characters)", min_len));
        }
    }
    
    if let Some(max_len) = rules.max_length {
        if text.len() > max_len {
            errors.push(format!("Text too long (maximum {} characters)", max_len));
        }
    }
    
    for pattern in &rules.required_patterns {
        if !text.contains(pattern) {
            errors.push(format!("Required pattern '{}' not found", pattern));
        }
    }
    
    for pattern in &rules.forbidden_patterns {
        if text.contains(pattern) {
            errors.push(format!("Forbidden pattern '{}' found", pattern));
        }
    }
    
    if let Some(ref allowed) = rules.allowed_chars {
        for ch in text.chars() {
            if !allowed.contains(ch) {
                errors.push(format!("Character '{}' is not allowed", ch));
                break;
            }
        }
    }
    
    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Generates a simple checksum for text integrity verification.
///
/// # Arguments
///
/// * `text` - Text to checksum.
///
/// # Returns
///
/// * A `String` containing the checksum.
pub fn text_checksum(text: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:x}", hasher.finish())
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

    #[test]
    fn test_safe_truncate_bytes() {
        let text = "Hello 世界";
        let truncated = safe_truncate_bytes(text, 8);
        assert_eq!(truncated, "Hello ");
    }

    #[test]
    fn test_chunk_text() {
        let text = "The quick brown fox jumps over the lazy dog";
        let chunks = chunk_text(text, 15);
        assert!(chunks.len() > 1);
        assert!(chunks.iter().all(|chunk| chunk.len() <= 20)); // Some flexibility for word boundaries
    }

    #[test]
    fn test_word_frequency() {
        let text = "hello world hello";
        let freq = word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&1));
    }

    #[test]
    fn test_most_common_words() {
        let text = "the quick brown fox jumps over the lazy dog the";
        let common = most_common_words(text, 3);
        assert_eq!(common[0].0, "the");
        assert_eq!(common[0].1, 3);
    }

    #[test]
    fn test_compare_texts() {
        let similarity = compare_texts("hello world", "hello universe");
        assert!(similarity.jaccard_similarity > 0.0);
        assert!(similarity.common_words.contains(&"hello".to_string()));
    }

    #[test]
    fn test_escape_text() {
        assert_eq!(escape_text("<script>", "html"), "&lt;script&gt;");
        assert_eq!(escape_text("hello\nworld", "json"), "hello\\nworld");
        assert_eq!(escape_text("hello,world", "csv"), "\"hello,world\"");
    }

    #[test]
    fn test_validate_text() {
        let rules = ValidationRules {
            min_length: Some(5),
            max_length: Some(10),
            required_patterns: vec!["hello".to_string()],
            forbidden_patterns: vec!["bad".to_string()],
            allowed_chars: None,
        };
        
        let result = validate_text("hello", &rules);
        assert!(result.is_valid);
        
        let result = validate_text("hi", &rules);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_text_checksum() {
        let checksum1 = text_checksum("hello");
        let checksum2 = text_checksum("hello");
        let checksum3 = text_checksum("world");
        
        assert_eq!(checksum1, checksum2);
        assert_ne!(checksum1, checksum3);
    }
}

