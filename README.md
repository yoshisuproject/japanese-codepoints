# Japanese Codepoints

[![crates.io](https://img.shields.io/crates/v/japanese-codepoints.svg)](https://crates.io/crates/japanese-codepoints)
[![docs.rs](https://docs.rs/japanese-codepoints/badge.svg)](https://docs.rs/japanese-codepoints)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust library for Japanese character validation and code point handling based on JIS standards.

This library provides collections of Unicode code points for various Japanese character sets, with advanced validation macros and static caching for optimal performance. Perfect for input validation in systems with strict character requirements (e.g., legacy mainframe integration). It is a Rust port of the Java library `terasoluna-gfw-codepoints`.

## Character Sets

The library is organized using feature flags to keep it lightweight. You only need to enable the character sets you require.

| Feature Flag               | Description                                                          |
| -------------------------- | -------------------------------------------------------------------- |
| `default` (`codepoints`)   | Core `CodePoints` struct and basic ASCII sets.                       |
| `codepoints-jisx0201`      | JIS X 0201: Half-width Katakana and Latin letters.                   |
| `codepoints-jisx0208`      | JIS X 0208: Hiragana, Katakana, special characters, etc. (no Kanji). |
| `codepoints-jisx0208kanji` | JIS X 0208: Level 1 and 2 Kanji (6,355 characters).                  |
| `codepoints-jisx0213kanji` | JIS X 0213: Level 1, 2, 3, and 4 Kanji (10,050 characters).          |
| `full`                     | Enables all the features above.                                      |

## Installation

Add this to your `Cargo.toml`. Enable the features you need.

```toml
[dependencies]
# Minimal installation
japanese-codepoints = "0.1.0"

# To enable specific character sets, add features
# For example, to get JIS X 0208 Hiragana, Katakana, and Kanji:
japanese-codepoints = { version = "0.1.0", features = ["codepoints-jisx0208", "codepoints-jisx0208kanji"] }

# To enable everything
japanese-codepoints = { version = "0.1.0", features = ["full"] }
```

## Quick Start

### Basic Character Set Validation

```rust
// This requires the `codepoints-jisx0208` feature
use japanese_codepoints::jisx0208::Hiragana;

// Create instance (or use cached version for better performance)
let hiragana = Hiragana::new();
let hiragana_cached = Hiragana::cached(); // 1900+ times faster!

// Check if a string contains only hiragana characters
assert!(hiragana.contains("あいうえお"));
assert!(!hiragana.contains("あいうえおA")); // Contains 'A'

// Find the first invalid character and its position
if let Some((invalid_char, position)) = hiragana.codepoints().first_excluded_with_position("あいうえおA") {
    println!("Invalid char '{}' at position {}",
             char::from_u32(invalid_char).unwrap(), position); // 'A' at position 5
}

// Get all invalid characters
let excluded: Vec<u32> = hiragana.codepoints().all_excluded("いろはABにほへとC");
println!("Invalid codepoints: {:?}", excluded); // [65, 66, 67] (A, B, C)
```

### Validation Macros (Recommended)

The easiest way to validate Japanese text:

```rust
use japanese_codepoints::{validate_hiragana, validate_katakana, validate_japanese_mixed};

// Simple hiragana validation
validate_hiragana!("あいうえお")?; // OK
validate_hiragana!("Hello")?;      // Error

// Katakana validation
validate_katakana!("アイウエオ")?; // OK

// Mixed Japanese + ASCII validation
validate_japanese_mixed!("こんにちはHello")?;  // OK
validate_japanese_mixed!("こんにちは漢字")?;    // Error (contains kanji)

// Advanced validation with custom error messages
use japanese_codepoints::validate_codepoints_advanced;

validate_codepoints_advanced!("hello", ascii_printable)?;
validate_codepoints_advanced!("hello", hiragana.codepoints().clone(), "Only hiragana allowed")?;
```

### Multi-Character Set Validation

Validate against multiple character sets - perfect for real-world Japanese text:

```rust
// This requires `codepoints-jisx0208` and `codepoints-jisx0208kanji` features
use japanese_codepoints::{jisx0208::Hiragana, jisx0208::Katakana, CodePoints};

// Use cached versions for better performance
let hiragana = Hiragana::cached();
let katakana = Katakana::cached();
let ascii = CodePoints::ascii_printable_cached();

// Create collection of allowed character sets
let allowed = [
    hiragana.codepoints().clone(),
    katakana.codepoints().clone(),
    ascii.clone()
];

let mixed_text = "こんにちはHello";
let japanese_only = "こんにちはアリガトウ";
let invalid_text = "こんにちは漢字";

// Check if text contains only characters from any of the allowed sets
assert!(CodePoints::contains_all_in_any(mixed_text, &allowed));
assert!(CodePoints::contains_all_in_any(japanese_only, &allowed));
assert!(!CodePoints::contains_all_in_any(invalid_text, &allowed)); // Contains kanji
```

### Set Operations

The `CodePoints` struct supports standard set operations to create custom validation rules:

```rust
use japanese_codepoints::{jisx0208::Hiragana, jisx0208::Katakana, CodePoints};

let hiragana = Hiragana::cached();
let katakana = Katakana::cached();

// Union: Combine character sets
let kana = hiragana.codepoints().union(katakana.codepoints());
assert!(kana.contains("あいうアイウ"));
assert!(!kana.contains("あいうABC"));

// Difference: Remove specific characters
let a_char = CodePoints::from_string("あ");
let without_a = hiragana.codepoints().difference(&a_char);
assert!(!without_a.contains("あ"));
assert!(without_a.contains("いうえお"));

// Intersection: Find common characters (hiragana ∩ katakana = empty)
let common = hiragana.codepoints().intersection(katakana.codepoints());
assert!(common.is_empty());

// Symmetric difference: Characters in either set but not both
let sym_diff = hiragana.codepoints().symmetric_difference(katakana.codepoints());
assert!(sym_diff.contains("あ")); // Only in hiragana
assert!(sym_diff.contains("ア")); // Only in katakana
```

## Examples

### Comprehensive Validation Demo

See all features in action:

```bash
cargo run --example comprehensive_validation_example --features "codepoints-jisx0208,codepoints-jisx0201"
```

This example demonstrates:

- ✨ All validation macros
- 🚀 Caching performance (1900+ times faster!)
- 🔧 Multi-character set validation
- 📊 Performance benchmarks

### Individual Character Set Examples

```bash
# Basic CodePoints operations
cargo run --example codepoints_example

# JIS X 0201 halfwidth characters
cargo run --example jisx0201_example --features "codepoints-jisx0201"

# JIS X 0208 characters (hiragana, katakana, etc.)
cargo run --example jisx0208_example --features "codepoints-jisx0208"

# JIS X 0208 Kanji characters
cargo run --example jisx0208kanji_example --features "codepoints-jisx0208kanji"

# JIS X 0213 extended Kanji
cargo run --example jisx0213kanji_example --features "codepoints-jisx0213kanji"
```

## Performance

This library is designed for high-performance applications:

```bash
# Run comprehensive benchmarks
cargo bench --features "full"
```

## Testing

Run the comprehensive test suite:

```bash
# Test all features
cargo test --all-features

# Test specific features only
cargo test --features "codepoints-jisx0208"

# Include documentation tests
cargo test --doc --all-features
```

All examples are also tested to ensure they work correctly:

```bash
# Test that all examples compile and run
cargo test --examples --all-features
```

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
