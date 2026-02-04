# Japanese Codepoints

[![crates.io](https://img.shields.io/crates/v/japanese-codepoints.svg)](https://crates.io/crates/japanese-codepoints)
[![docs.rs](https://docs.rs/japanese-codepoints/badge.svg)](https://docs.rs/japanese-codepoints)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Dependency Status](https://img.shields.io/badge/dependencies-zero-brightgreen.svg)](./Cargo.toml)

A high-performance, zero-dependency Rust library for Japanese character validation and code point handling based on JIS standards.

This library provides collections of Unicode code points for various Japanese character sets, with advanced validation macros and static caching for optimal performance. Perfect for input validation in systems with strict character requirements (e.g., legacy mainframe integration, payment systems, government applications).

> **Rust port of** [`terasoluna-gfw-codepoints`](https://github.com/terasolunaorg/terasoluna-gfw) (Java) - reimagined with Rust's type system and zero-cost abstractions.

## Features

- **High Performance**: Static caching via `OnceLock` eliminates repeated allocations (1900× faster than creating new instances)
- **Zero Dependencies**: No external dependencies for faster compile times and smaller binaries
- **Type Safety**: Leverages Rust's ownership system to prevent runtime errors
- **Feature Flags**: Compile only the character sets you need
- **Validation Macros**: Ergonomic macros for common validation patterns
- **Zero-Copy Operations**: Efficient set operations (union, intersection, difference)

## Character Sets

The library is organized using feature flags to keep it lightweight. You only need to enable the character sets you require.

| Feature Flag               | Characters | Description                                                                  |
| :------------------------- | :--------- | :--------------------------------------------------------------------------- |
| `default` (`codepoints`)   | 225        | Core `CodePoints` struct + ASCII control/printable                           |
| `codepoints-jisx0201`      | 158        | JIS X 0201: Half-width Katakana (63) + Latin letters (95)                    |
| `codepoints-jisx0208`      | ~700       | JIS X 0208: Hiragana, Katakana, Latin, Greek, Cyrillic, symbols, box-drawing |
| `codepoints-jisx0208kanji` | 6,355      | JIS X 0208: Level 1 & 2 Kanji                                                |
| `codepoints-jisx0213kanji` | 10,050     | JIS X 0213: Level 1-4 Kanji (extends JIS X 0208)                             |
| `full`                     | ~17,500    | All character sets                                                           |

### When to use which feature?

- **Web forms with Japanese input**: `codepoints-jisx0208` (Hiragana/Katakana)
- **Legacy system integration**: `codepoints-jisx0201` (halfwidth) + `codepoints-jisx0208kanji`
- **Full Japanese text support**: `full` or specific kanji features
- **ASCII-only validation**: `default` (no extra features needed)

## Quick Start

### Requirements

- **Rust**: 1.70+ (for `std::sync::OnceLock`)
- **Edition**: 2021

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
# Minimal: ASCII validation only
japanese-codepoints = "0.1.0"

# Specific character sets (recommended)
japanese-codepoints = { version = "0.1.0", features = ["codepoints-jisx0208", "codepoints-jisx0208kanji"] }

# Everything (largest binary size)
japanese-codepoints = { version = "0.1.0", features = ["full"] }

# Disable default features for minimal build
japanese-codepoints = { version = "0.1.0", default-features = false, features = ["codepoints-jisx0208"] }
```

## Usage Examples

### Basic Validation

```rust
use japanese_codepoints::jisx0208::Hiragana;

// Use cached instance for best performance
let hiragana = Hiragana::cached();

// Validate string contains only hiragana
assert!(hiragana.contains("あいうえお"));
assert!(!hiragana.contains("あいうえおA")); // 'A' is not hiragana

// Get detailed error information
if let Some((invalid, pos)) = hiragana.codepoints().first_excluded_with_position("あいうえおA") {
    println!("Invalid char '{}' at position {}",
             char::from_u32(invalid).unwrap(), pos);
    // Output: Invalid char 'A' at position 5
}
```

### Validation Macros (Recommended)

```rust
use japanese_codepoints::{validate_hiragana, validate_katakana, validate_japanese_mixed};

// Simple validation
validate_hiragana!("あいうえお")?;  // OK
validate_katakana!("アイウエオ")?;  // OK

// Mixed validation (Hiragana + Katakana + ASCII)
validate_japanese_mixed!("こんにちはHello")?;  // OK
validate_japanese_mixed!("こんにちは漢字")?;   // Error: contains kanji
```

### Multi-Set Validation

```rust
use japanese_codepoints::{contains_all_in_any, CodePoints};
use japanese_codepoints::jisx0208::{Hiragana, Katakana};

let allowed = [
    Hiragana::cached().codepoints(),
    Katakana::cached().codepoints(),
    CodePoints::ascii_printable_cached(),
];

// Each character must be in at least one set
assert!(contains_all_in_any("こんにちはHello", &allowed));  // ✓ Hiragana + ASCII
assert!(contains_all_in_any("アイウエオ", &allowed));       // ✓ Katakana
assert!(!contains_all_in_any("こんにちは漢字", &allowed));  // ✗ Kanji not in any set
```

### Set Operations

```rust
use japanese_codepoints::jisx0208::{Hiragana, Katakana};

let hiragana = Hiragana::cached();
let katakana = Katakana::cached();

// Union: Combine sets
let all_kana = hiragana.codepoints().union(katakana.codepoints());
assert!(all_kana.contains("あいうアイウ"));

// Custom character set
let no_a = hiragana.codepoints().difference(&CodePoints::from_string("あ"));
assert!(!no_a.contains("あ"));
assert!(no_a.contains("いうえお"));
```

## Comparison with Java Original

| Feature            | Java (terasoluna-gfw)            | Rust (this crate)       |
| :----------------- | :------------------------------- | :---------------------- |
| **Caching**        | `ConcurrentHashMap` + Reflection | `OnceLock` (lock-free)  |
| **Memory Safety**  | Runtime checks                   | Compile-time guarantees |
| **Dependencies**   | Jakarta Validation + Spring      | Zero dependencies       |
| **Set Operations** | Basic (union, intersect)         | Full set algebra        |
| **Error Handling** | Exceptions                       | `Result<T, E>`          |
| **Validation**     | Annotation-based                 | Macros + Functions      |
| **Binary Size**    | JVM + dependencies               | Native, minimal         |

## Testing

```bash
# All tests
cargo test --all-features

# Documentation tests
cargo test --doc --all-features

# Examples
cargo test --examples --all-features

# Check all feature combinations
cargo hack check --feature-powerset
```

## Examples

```bash
# Comprehensive demo
cargo run --example comprehensive_validation_example --features full

# Individual character sets
cargo run --example ascii_example
cargo run --example jisx0201_example --features codepoints-jisx0201
cargo run --example jisx0208_example --features codepoints-jisx0208
cargo run --example jisx0208kanji_example --features codepoints-jisx0208kanji
cargo run --example jisx0213kanji_example --features codepoints-jisx0213kanji
```

## Architecture

- Core Layer (always included)
  - CodePoints (HashSet<u32> wrapper)
  - Set operations (union, intersection, diff, etc.)
  - ASCII sets (control, printable, all, crlf)
- Character Set Modules (feature-gated)
  - jisx0201: Latin + Halfwidth Katakana
  - jisx0208: Hiragana, Katakana, Symbols, etc.
  - jisx0208kanji: 6,355 JIS X 0208 Kanji
  - jisx0213kanji: 10,050 JIS X 0213 Kanji
- Validation Layer
  - ValidationError (structured errors)
  - Macros (validate_hiragana!, etc.)

## Common Pitfalls

### UTF-8 vs Unicode Code Points

This library validates **Unicode code points** (scalar values), not bytes:

```rust
// ✓ Works: validates Unicode characters
let hiragana = Hiragana::cached();
hiragana.contains("あ");  // U+3042

// ✗ Not for byte validation
// For byte-level validation, use encoding_rs or similar
```

### Cached vs New

Always use `cached()` in production:

```rust
// ✗ Creates new HashSet every call
let h = Hiragana::new();

// ✓ Zero-allocation after first call
let h = Hiragana::cached();
```

### Feature Flags

Don't forget to enable features:

```rust
// Cargo.toml
[dependencies]
// ✗ This won't compile if you use JIS X 0208
japanese-codepoints = "0.1.0"

// ✓ Enable the features you need
japanese-codepoints = { version = "0.1.0", features = ["codepoints-jisx0208"] }
```

## License

This project is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT license** ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

This is a Rust port of the [terasoluna gfw codepoints](https://github.com/terasolunaorg/terasoluna-gfw/tree/master/terasoluna-gfw-common-libraries/terasoluna-gfw-codepoints) library. Special thanks to the original authors for the excellent Java implementation.

---

**[Full Documentation](https://docs.rs/japanese-codepoints)** | **[Crates.io](https://crates.io/crates/japanese-codepoints)** | **[Repository](https://github.com/yoshisuproject/japanese-codepoints)**
