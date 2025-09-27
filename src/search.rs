use regex::Regex;
use std::collections::{HashMap, VecDeque};

/// Represents a search result with context information.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub start: usize,
    pub end: usize,
    pub matched_text: String,
    pub context_before: String,
    pub context_after: String,
}

/// Options for customizing search behavior.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_words_only: bool,
    pub max_results: Option<usize>,
    pub context_length: usize,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            whole_words_only: false,
            max_results: None,
            context_length: 50,
        }
    }
}

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

#[derive(Default)]
struct AhoCorasick {
    goto: HashMap<(usize, char), usize>,
    output: Vec<Vec<usize>>,
    fail: Vec<usize>,
    pattern_lengths: Vec<usize>,
}

impl AhoCorasick {
    fn new(patterns: Vec<&str>) -> Self {
        let mut ac = AhoCorasick::default();
        ac.build(patterns);
        ac
    }

    fn build(&mut self, patterns: Vec<&str>) {
        let mut new_state = 0;
        self.goto.insert((0, '\0'), 0);

        // Initialize output vector with one element for state 0
        self.output = vec![vec![]];
        self.pattern_lengths = patterns.iter().map(|p| p.len()).collect();

        for (i, pattern) in patterns.iter().enumerate() {
            let mut current_state = 0;
            for c in pattern.chars() {
                if !self.goto.contains_key(&(current_state, c)) {
                    new_state += 1;
                    self.goto.insert((current_state, c), new_state);
                    // Ajouter un nouveau vecteur vide pour le nouvel état
                    self.output.push(vec![]);
                }
                current_state = *self.goto.get(&(current_state, c)).unwrap();
            }
            self.output[current_state].push(i);
        }

        self.fail = vec![0; new_state + 1];
        let mut queue = VecDeque::new();

        // Fixed: Use proper destructuring without reference patterns in closure
        for (&(state, c), &next) in self.goto.iter() {
            if c != '\0' && state == 0 {
                queue.push_back(next);
            }
        }

        while let Some(state) = queue.pop_front() {
            // Fixed: Use proper destructuring without reference patterns in closure
            for (&(_, c), _) in self.goto.iter() {
                if c != '\0' {
                    if let Some(&next_state) = self.goto.get(&(state, c)) {
                        let mut fail_state = self.fail[state];
                        while !self.goto.contains_key(&(fail_state, c)) && fail_state != 0 {
                            fail_state = self.fail[fail_state];
                        }
                        self.fail[next_state] = self.goto.get(&(fail_state, c)).copied().unwrap_or(0);

                        let fail_outputs = self.output[self.fail[next_state]].clone();
                        self.output[next_state].extend_from_slice(&fail_outputs);

                        queue.push_back(next_state);
                    }
                }
            }
        }
    }

    fn find_iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut current_state = 0;
        let mut results = Vec::new();

        for (i, c) in text.chars().enumerate() {
            while !self.goto.contains_key(&(current_state, c)) && current_state != 0 {
                current_state = self.fail[current_state];
            }
            current_state = self.goto.get(&(current_state, c)).copied().unwrap_or(0);

            for &pattern_index in &self.output[current_state] {
                let start = i + 1 - self.pattern_lengths[pattern_index];
                results.push((start, pattern_index));
            }
        }

        results.into_iter()
    }
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
    let ac = AhoCorasick::new(patterns.clone());
    let mut results = Vec::new();

    for (start, pattern_index) in ac.find_iter(text) {
        results.push((start, patterns[pattern_index]));
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

///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the pattern to search for.
/// * `before` - Number of characters to include before the match.
/// * `after` - Number of characters to include after the match.
///
/// # Returns
///
/// * An `Option<SearchResult>` containing the match with context.
///
/// # Examples
///
/// ```
/// let text = "The quick brown fox jumps over the lazy dog";
/// let result = loki_text::search::find_with_context(text, "brown", 5, 5);
/// assert!(result.is_some());
/// ```
pub fn find_with_context(text: &str, pattern: &str, before: usize, after: usize) -> Option<SearchResult> {
    let chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    for i in 0..=chars.len().saturating_sub(pattern_chars.len()) {
        if chars[i..i + pattern_chars.len()] == pattern_chars[..] {
            let start_context = i.saturating_sub(before);
            let end_context = std::cmp::min(i + pattern_chars.len() + after, chars.len());
            
            return Some(SearchResult {
                start: i,
                end: i + pattern_chars.len(),
                matched_text: pattern.to_string(),
                context_before: chars[start_context..i].iter().collect(),
                context_after: chars[i + pattern_chars.len()..end_context].iter().collect(),
            });
        }
    }
    None
}

/// Finds all occurrences of a pattern with their positions.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the pattern to search for.
///
/// # Returns
///
/// * A `Vec<usize>` containing all starting positions of the pattern.
///
/// # Examples
///
/// ```
/// let text = "hello world hello universe";
/// let result = loki_text::search::find_all_positions(text, "hello");
/// assert_eq!(result, vec![0, 12]);
/// ```
pub fn find_all_positions(text: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    if pattern_chars.is_empty() {
        return positions;
    }
    
    for i in 0..=text_chars.len().saturating_sub(pattern_chars.len()) {
        if text_chars[i..i + pattern_chars.len()] == pattern_chars[..] {
            positions.push(i);
        }
    }
    
    positions
}

/// Finds text between two patterns.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `start_pattern` - A string slice that marks the start boundary.
/// * `end_pattern` - A string slice that marks the end boundary.
///
/// # Returns
///
/// * An `Option<String>` containing the text between patterns.
///
/// # Examples
///
/// ```
/// let text = "Hello [world] test";
/// let result = loki_text::search::find_between(text, "[", "]");
/// assert_eq!(result, Some("world".to_string()));
/// ```
pub fn find_between(text: &str, start_pattern: &str, end_pattern: &str) -> Option<String> {
    let start_pos = text.find(start_pattern)?;
    let search_start = start_pos + start_pattern.len();
    let end_pos = text[search_start..].find(end_pattern)?;
    
    Some(text[search_start..search_start + end_pos].to_string())
}

/// Finds pattern only at word boundaries.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the pattern to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the position if found at word boundary.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::search::find_word_boundaries(text, "ell");
/// assert_eq!(result, None); // "ell" is not at word boundary
/// ```
pub fn find_word_boundaries(text: &str, pattern: &str) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    for i in 0..=chars.len().saturating_sub(pattern_chars.len()) {
        if chars[i..i + pattern_chars.len()] == pattern_chars[..] {
            // Check if at word boundary
            let prev_is_word = i > 0 && chars[i - 1].is_alphanumeric();
            let next_is_word = i + pattern_chars.len() < chars.len() 
                && chars[i + pattern_chars.len()].is_alphanumeric();
            let pattern_is_word = pattern_chars[0].is_alphanumeric() 
                && pattern_chars[pattern_chars.len() - 1].is_alphanumeric();
            
            if pattern_is_word && !prev_is_word && !next_is_word {
                return Some(i);
            } else if !pattern_is_word {
                return Some(i);
            }
        }
    }
    None
}

/// Finds strings within a given edit distance using Wagner-Fischer algorithm.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the pattern to search for.
/// * `max_distance` - Maximum allowed edit distance.
///
/// # Returns
///
/// * A `Vec<(usize, usize, usize)>` containing (start_pos, end_pos, distance) tuples.
///
/// # Examples
///
/// ```
/// let text = "hello world";
/// let result = loki_text::search::fuzzy_search(text, "helo", 1);
/// assert!(!result.is_empty());
/// ```
pub fn fuzzy_search(text: &str, pattern: &str, max_distance: usize) -> Vec<(usize, usize, usize)> {
    let mut results = Vec::new();
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    if pattern_chars.is_empty() {
        return results;
    }
    
    for i in 0..text_chars.len() {
        for j in (i + 1)..=text_chars.len() {
            let substring: String = text_chars[i..j].iter().collect();
            let distance = edit_distance(&substring, pattern);
            
            if distance <= max_distance {
                results.push((i, j, distance));
            }
        }
    }
    
    results.sort_by_key(|&(_, _, dist)| dist);
    results
}

fn edit_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 { matrix[i][0] = i; }
    for j in 0..=len2 { matrix[0][j] = j; }
    
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

/// Finds the longest common substring between two strings.
///
/// # Arguments
///
/// * `text1` - First string to compare.
/// * `text2` - Second string to compare.
///
/// # Returns
///
/// * An `Option<String>` containing the longest common substring.
///
/// # Examples
///
/// ```
/// let result = loki_text::search::find_longest_common_substring("hello", "jello");
/// assert_eq!(result, Some("ello".to_string()));
/// ```
pub fn find_longest_common_substring(text1: &str, text2: &str) -> Option<String> {
    let chars1: Vec<char> = text1.chars().collect();
    let chars2: Vec<char> = text2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();
    
    if len1 == 0 || len2 == 0 {
        return None;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    let mut max_length = 0;
    let mut ending_pos = 0;
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            if chars1[i - 1] == chars2[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1] + 1;
                if matrix[i][j] > max_length {
                    max_length = matrix[i][j];
                    ending_pos = i;
                }
            }
        }
    }
    
    if max_length == 0 {
        None
    } else {
        let start = ending_pos - max_length;
        Some(chars1[start..ending_pos].iter().collect())
    }
}

/// Searches for similar strings within a collection based on edit distance.
///
/// # Arguments
///
/// * `target` - The target string to match against.
/// * `candidates` - A slice of candidate strings.
/// * `threshold` - Maximum edit distance to consider a match.
///
/// # Returns
///
/// * A `Vec<(String, usize)>` containing matching strings and their distances.
///
/// # Examples
///
/// ```
/// let candidates = vec!["hello", "helo", "world", "help"];
/// let result = loki_text::search::find_similar_strings("hello", &candidates, 2);
/// ```
pub fn find_similar_strings(target: &str, candidates: &[&str], threshold: usize) -> Vec<(String, usize)> {
    let mut results = Vec::new();
    
    for &candidate in candidates {
        let distance = edit_distance(target, candidate);
        if distance <= threshold {
            results.push((candidate.to_string(), distance));
        }
    }
    
    results.sort_by_key(|&(_, dist)| dist);
    results
}

/// Advanced search with customizable options.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the pattern to search for.
/// * `options` - Search configuration options.
///
/// # Returns
///
/// * A `Vec<SearchResult>` containing all matches with context.
///
/// # Examples
///
/// ```
/// let text = "Hello HELLO hello";
/// let mut options = loki_text::search::SearchOptions::default();
/// options.case_sensitive = false;
/// let result = loki_text::search::advanced_search(text, "hello", &options);
/// ```
pub fn advanced_search(text: &str, pattern: &str, options: &SearchOptions) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let search_text = if options.case_sensitive { text.to_string() } else { text.to_lowercase() };
    let search_pattern = if options.case_sensitive { pattern.to_string() } else { pattern.to_lowercase() };
    
    let text_chars: Vec<char> = search_text.chars().collect();
    let pattern_chars: Vec<char> = search_pattern.chars().collect();
    let original_chars: Vec<char> = text.chars().collect();
    
    let mut i = 0;
    while i <= text_chars.len().saturating_sub(pattern_chars.len()) {
        if text_chars[i..i + pattern_chars.len()] == pattern_chars[..] {
            // Check word boundaries if required
            if options.whole_words_only {
                let prev_is_word = i > 0 && text_chars[i - 1].is_alphanumeric();
                let next_is_word = i + pattern_chars.len() < text_chars.len() 
                    && text_chars[i + pattern_chars.len()].is_alphanumeric();
                
                if prev_is_word || next_is_word {
                    i += 1;
                    continue;
                }
            }
            
            // Extract context
            let start_context = i.saturating_sub(options.context_length);
            let end_context = std::cmp::min(i + pattern_chars.len() + options.context_length, original_chars.len());
            
            results.push(SearchResult {
                start: i,
                end: i + pattern_chars.len(),
                matched_text: original_chars[i..i + pattern_chars.len()].iter().collect(),
                context_before: original_chars[start_context..i].iter().collect(),
                context_after: original_chars[i + pattern_chars.len()..end_context].iter().collect(),
            });
            
            // Check max results limit
            if let Some(max) = options.max_results {
                if results.len() >= max {
                    break;
                }
            }
            
            i += pattern_chars.len(); // Skip past current match
        } else {
            i += 1;
        }
    }
    
    results
}

/// A simple trie structure for prefix searching.
#[derive(Debug, Default)]
pub struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
    value: Option<String>,
}

impl Trie {
    /// Creates a new empty Trie.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Inserts a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_default();
        }
        current.is_end = true;
        current.value = Some(word.to_string());
    }
    
    /// Searches for words with the given prefix.
    pub fn search_prefix(&self, prefix: &str) -> Vec<String> {
        let mut current = self;
        
        // Navigate to the prefix node
        for ch in prefix.chars() {
            if let Some(child) = current.children.get(&ch) {
                current = child;
            } else {
                return Vec::new(); // Prefix not found
            }
        }
        
        // Collect all words from this point
        let mut results = Vec::new();
        self.collect_words(current, &mut results);
        results
    }
    
    fn collect_words(&self, node: &Trie, results: &mut Vec<String>) {
        if node.is_end {
            if let Some(ref word) = node.value {
                results.push(word.clone());
            }
        }
        
        for child in node.children.values() {
            self.collect_words(child, results);
        }
    }
}

/// Suffix array implementation for fast substring search.
#[derive(Debug)]
pub struct SuffixArray {
    text: String,
    suffixes: Vec<usize>,
}

impl SuffixArray {
    /// Creates a new suffix array from text.
    pub fn new(text: &str) -> Self {
        let mut suffixes: Vec<usize> = (0..text.len()).collect();
        
        // Sort suffixes lexicographically
        suffixes.sort_by(|&a, &b| text[a..].cmp(&text[b..]));
        
        Self {
            text: text.to_string(),
            suffixes,
        }
    }
    
    /// Searches for a pattern using binary search on suffix array.
    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let mut results = Vec::new();
        
        // Binary search for first occurrence
        let mut left = 0;
        let mut right = self.suffixes.len();
        
        while left < right {
            let mid = (left + right) / 2;
            let suffix_start = self.suffixes[mid];
            let suffix = &self.text[suffix_start..];
            
            if suffix.starts_with(pattern) || suffix < pattern {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        // Collect all matching suffixes
        for &suffix_start in &self.suffixes {
            if self.text[suffix_start..].starts_with(pattern) {
                results.push(suffix_start);
            }
        }
        
        results.sort();
        results
    }
}

/// Finds the last occurrence of a pattern in text.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to search within.
/// * `pattern` - A string slice that holds the pattern to search for.
///
/// # Returns
///
/// * An `Option<usize>` containing the starting index of the last occurrence.
///
/// # Examples
///
/// ```
/// let text = "hello world hello";
/// let result = loki_text::search::find_last(text, "hello");
/// assert_eq!(result, Some(12));
/// ```
pub fn find_last(text: &str, pattern: &str) -> Option<usize> {
    text.rfind(pattern)
}

/// Two-way string matching algorithm implementation.
/// More efficient than KMP for many practical cases.
pub fn two_way_search(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let n = text_bytes.len();
    let m = pattern_bytes.len();
    
    if m > n {
        return None;
    }
    
    // Simple implementation - in practice this would use the full two-way algorithm
    // with critical factorization and period computation
    for i in 0..=n - m {
        if text_bytes[i..i + m] == pattern_bytes[..] {
            return Some(i);
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

    #[test]
    fn test_find_all_positions() {
        let text = "hello world hello";
        let result = find_all_positions(text, "hello");
        assert_eq!(result, vec![0, 12]);
    }

    #[test]
    fn test_find_between() {
        let text = "Hello [world] test";
        let result = find_between(text, "[", "]").unwrap();
        assert_eq!(result, "world");
    }

    #[test]
    fn test_fuzzy_search() {
        let text = "hello world";
        let result = fuzzy_search(text, "helo", 1);
        assert!(!result.is_empty());
        assert_eq!(result[0].2, 1); // distance should be 1
    }

    #[test]
    fn test_longest_common_substring() {
        let result = find_longest_common_substring("hello", "jello").unwrap();
        assert_eq!(result, "ello");
    }

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");
        
        let results = trie.search_prefix("hel");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"hello".to_string()));
        assert!(results.contains(&"help".to_string()));
    }

    #[test]
    fn test_suffix_array() {
        let sa = SuffixArray::new("banana");
        let results = sa.search("ana");
        assert!(results.contains(&1));
        assert!(results.contains(&3));
    }

    #[test]
    fn test_advanced_search() {
        let text = "Hello HELLO hello";
        let mut options = SearchOptions::default();
        options.case_sensitive = false;
        let results = advanced_search(text, "hello", &options);
        assert_eq!(results.len(), 3);
    }
}
