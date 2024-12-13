use regex::Regex;
use aho_corasick::AhoCorasick;

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
/// let result = loki_text::search::find_pattern(text, pattern);
/// assert_eq!(result, Some("brown".to_string()));
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
/// let result = loki_text::search::replace_pattern(text, pattern, replacement);
/// assert_eq!(result, "The quick red fox jumps over the lazy dog");
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
/// let result = loki_text::search::count_pattern(text, pattern);
/// assert_eq!(result, 2);
/// ```
pub fn count_pattern(text: &str, pattern: &str) -> usize {
    let re = Regex::new(&format!("(?i){}", pattern)).unwrap();
    re.find_iter(text).count()
}

/// Finds the first occurrence of a substring using the Knuth-Morris-Pratt (KMP) algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the substring to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the starting index of the first occurrence of the substring, or `None` if not found.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = "quick";
/// let result = loki_text::search::kmp_search(text, pattern);
/// assert_eq!(result, Some(4));
/// ```
pub fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let mut lps = vec![0; pattern_bytes.len()];
    let mut j = 0;

    // Preprocess the pattern to compute the lps (longest prefix suffix) array
    let mut i = 1;
    while i < pattern_bytes.len() {
        if pattern_bytes[i] == pattern_bytes[j] {
            j += 1;
            lps[i] = j;
            i += 1;
        } else {
            if j != 0 {
                j = lps[j - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    // Search the pattern in the text
    let mut i = 0;
    let mut j = 0;
    while i < text_bytes.len() {
        if pattern_bytes[j] == text_bytes[i] {
            i += 1;
            j += 1;
        }
        if j == pattern_bytes.len() {
            return Some(i - j);
        } else if i < text_bytes.len() && pattern_bytes[j] != text_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    None
}

/// Finds the first occurrence of a substring using the Boyer-Moore algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the substring to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the starting index of the first occurrence of the substring, or `None` if not found.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = "quick";
/// let result = loki_text::search::boyer_moore_search(text, pattern);
/// assert_eq!(result, Some(4));
/// ```
pub fn boyer_moore_search(text: &str, pattern: &str) -> Option<usize> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let m = pattern_bytes.len();
    let n = text_bytes.len();

    if m == 0 || n == 0 || m > n {
        return None;
    }

    let mut bad_char = vec![m; 256];

    // Preprocess the pattern to compute the bad character heuristic
    for i in 0..m {
        bad_char[pattern_bytes[i] as usize] = (m - 1 - i) as usize;
    }

    let mut s = 0;
    while s <= (n - m) {
        let mut j = m - 1;
        while j > 0 && pattern_bytes[j as usize] == text_bytes[s + j] {
            j -= 1;
        }
        if j == 0 {
            return Some(s);
        } else {
            s += std::cmp::max(1, j as isize - bad_char[text_bytes[s + j] as usize] as isize) as usize;
        }
    }
    None
}

/// Finds the first occurrence of a substring using the Boyer-Moore-Horspool algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the substring to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the starting index of the first occurrence of the substring, or `None` if not found.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = "quick";
/// let result = loki_text::search::boyer_moore_horspool_search(text, pattern);
/// assert_eq!(result, Some(4));
/// ```
pub fn boyer_moore_horspool_search(text: &str, pattern: &str) -> Option<usize> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let m = pattern_bytes.len();
    let n = text_bytes.len();

    if m == 0 || n == 0 || m > n {
        return None;
    }

    let mut shift_table = vec![m; 256];
    for i in 0..m - 1 {
        shift_table[pattern_bytes[i] as usize] = m - 1 - i;
    }

    let mut s = 0;
    while s <= n - m {
        let mut j = m - 1;
        while j > 0 && pattern_bytes[j] == text_bytes[s + j] {
            j -= 1;
        }
        if j == 0 && pattern_bytes[j] == text_bytes[s + j] {
            return Some(s);
        } else {
            s += shift_table[text_bytes[s + m - 1] as usize];
        }
    }
    None
}

/// Finds the first occurrence of a substring using the Z algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the substring to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the starting index of the first occurrence of the substring, or `None` if not found.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = "quick";
/// let result = loki_text::search::z_algorithm_search(text, pattern);
/// assert_eq!(result, Some(4));
/// ```
pub fn z_algorithm_search(text: &str, pattern: &str) -> Option<usize> {
    let concat = format!("{}{}", pattern, text);
    let concat_bytes = concat.as_bytes();
    let n = concat_bytes.len();
    let m = pattern.len();

    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i > r {
            l = i;
            r = i;
            while r < n && concat_bytes[r] == concat_bytes[r - l] {
                r += 1;
            }
            z[i] = r - l;
            r -= 1;
        } else {
            let k = i - l;
            if z[k] < r - i + 1 {
                z[i] = z[k];
            } else {
                l = i;
                while r < n && concat_bytes[r] == concat_bytes[r - l] {
                    r += 1;
                }
                z[i] = r - l;
                r -= 1;
            }
        }
    }

    for i in m..n {
        if z[i] == m {
            return Some(i - m);
        }
    }
    None
}

/// Finds all occurrences of substrings using the Aho-Corasick algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `patterns` - A vector of string slices that holds the substrings to search for.
///
/// # Returns
///
/// * A `Vec<(usize, &str)>` containing the starting indices and the corresponding patterns found in the text.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let patterns = vec!["quick", "fox", "dog"];
/// let result = loki_text::search::aho_corasick_search(&text, patterns);
/// assert_eq!(result, vec![(4, "quick"), (16, "fox"), (40, "dog")]);
/// ```
pub fn aho_corasick_search<'a>(text: &'a str, patterns: Vec<&'a str>) -> Vec<(usize, &'a str)> {
    // Unwrap the Result or handle potential errors
    let ac = AhoCorasick::new(patterns.clone()).expect("Failed to create Aho-Corasick automaton");
    
    let mut results = Vec::new();
    for mat in ac.find_iter(text) {
        // Use the patterns vector to get the actual string pattern
        let pattern = patterns[mat.pattern().as_usize()];
        results.push((mat.start(), pattern));
    }
    results
}

/// Finds the first occurrence of a substring using the Rabin-Karp algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the substring to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the starting index of the first occurrence of the substring, or `None` if not found.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let pattern = "quick";
/// let result = loki_text::search::rabin_karp_search(text, pattern);
/// assert_eq!(result, Some(4));
/// ```
pub fn rabin_karp_search(text: &str, pattern: &str) -> Option<usize> {
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let m = pattern_bytes.len();
    let n = text_bytes.len();
    let q = 101; // A prime number
    let d = 256; // Number of characters in the input alphabet

    if m == 0 || n == 0 || m > n {
        return None;
    }

    let mut p = 0; // Hash value for pattern
    let mut t = 0; // Hash value for text
    let mut h = 1;

    for _ in 0..m - 1 {
        h = (h * d) % q;
    }

    for i in 0..m {
        p = (d * p + pattern_bytes[i] as u64) % q;
        t = (d * t + text_bytes[i] as u64) % q;
    }

    for s in 0..=n - m {
        if p == t {
            let mut j = 0;
            while j < m && pattern_bytes[j] == text_bytes[s + j] {
                j += 1;
            }
            if j == m {
                return Some(s);
            }
        }
        if s < n - m {
            t = (d * (t + q - (text_bytes[s] as u64 * h) % q) + text_bytes[s + m] as u64) % q;
        }
    }
    None
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
    fn test_kmp_search() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = "quick";
        let result = kmp_search(text, pattern);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_boyer_moore_search() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = "quick";
        let result = boyer_moore_search(text, pattern);
        assert_eq!(result, Some(4));
    }
    
    #[test]
    fn test_boyer_moore_horspool_search() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = "quick";
        let result = boyer_moore_horspool_search(text, pattern);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_z_algorithm_search() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = "quick";
        let result = z_algorithm_search(text, pattern);
        assert_eq!(result, Some(4));
    }
    
    #[test]
    fn test_aho_corasick_search() {
        let text = "The quick brown fox jumps over the lazy dog";
        let patterns = vec!["quick", "fox", "dog"];
        let result = aho_corasick_search(text, patterns);
        assert_eq!(result, vec![(4, "quick"), (16, "fox"), (40, "dog")]);
    }

    #[test]
    fn test_rabin_karp_search() {
        let text = "The quick brown fox jumps over the lazy dog";
        let pattern = "quick";
        let result = rabin_karp_search(text, pattern);
        assert_eq!(result, Some(4));
    }

}

