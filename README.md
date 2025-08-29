# loki_text

loki_text is a Rust library designed for advanced string manipulation. It offers features for pattern searching and replacement, as well as various text transformation operations. Inspired by the Norse god Loki, known for his cunning and manipulations, this library aims to provide powerful and flexible tools for working with strings.

## Features

- **Pattern Searching and Replacement**: Use regular expressions to find and replace patterns within strings.
- **Text Transformation**: Includes functions to reverse strings, check for palindromes, remove punctuation, extract numbers, and capitalize words.
- **Basic String Manipulation**: Functions for splitting, joining, converting to uppercase/lowercase, and trimming whitespace.

## Installation

Add `loki_text` to your `Cargo.toml`:

```toml
[dependencies]
loki_text = "0.1.5"
```

## Examples

### Pattern Searching

```rust
use loki_text::search::find_pattern;

let text = "The quick brown fox jumps over the lazy dog";
let pattern = r"quick\s(\w+)";
let result = find_pattern(text, pattern);
assert_eq!(result, Some("brown".to_string()));
```

#### Features

- **Knuth-Morris-Pratt (KMP) Algorithm**: A classic algorithm for substring search with linear time complexity.
- **Boyer-Moore Algorithm**: An efficient algorithm that skips characters while searching for a pattern.
- **Boyer-Moore-Horspool Algorithm**: A simplified version of the Boyer-Moore algorithm that is easier to implement and understand.
- **Z-Algorithm**: A linear time algorithm used for pattern searching and other string processing tasks.
- **Aho-Corasick Algorithm**: A multi-pattern search algorithm that builds a finite state machine from a set of keywords.
- **Rabin-Karp Algorithm**: A string-searching algorithm that uses hashing to find any one of a set of pattern strings in a text.


### Text Transformation

```rust
use loki_text::transform::reverse_string;

let text = "hello world";
let result = reverse_string(text);
assert_eq!(result, "dlrow olleh");
Basic String Manipulation

use loki_text::basic::to_uppercase;

let text = "hello world";
let result = to_uppercase(text);
assert_eq!(result, "HELLO WORLD");
```

## Contributing

Contributions are welcome! Please feel free to submit issues and enhancement requests.

## License

See the [LICENSE](https://github.com/roquess/loki_text/blob/main/LICENSE) file for more details.

## Future Development

loki_text is an evolving library. Future updates will include more advanced string manipulation functions, improved performance, and additional utilities to make working with strings in Rust even more powerful and convenient. Stay tuned for updates and new features!
