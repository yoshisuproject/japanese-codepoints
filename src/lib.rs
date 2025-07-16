//! Japanese Code Points Library
//!
//! A Rust library for handling Japanese character code points based on JIS standards.
//!
//! # Features
//!
//! - `codepoints`: Basic ASCII and control characters (default)
//! - `codepoints-jisx0201`: JIS X 0201 character set
//! - `codepoints-jisx0208`: JIS X 0208 character set (without kanji)
//! - `codepoints-jisx0208kanji`: JIS X 0208 kanji characters
//! - `codepoints-jisx0213kanji`: JIS X 0213 extended kanji characters
//! - `full`: All character sets
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::CodePoints;
//!
//! // Basic usage
//! let cp = CodePoints::new(vec![0x3041, 0x3042]); // あ, い
//! assert!(cp.contains("あ"));
//! assert!(!cp.contains("う"));
//! ```
//!
//! ```rust
//! # #[cfg(feature = "codepoints-jisx0208")]
//! use japanese_codepoints::jisx0208::Hiragana;
//!
//! # #[cfg(feature = "codepoints-jisx0208")]
//! let hiragana = Hiragana::new();
//! # #[cfg(feature = "codepoints-jisx0208")]
//! assert!(hiragana.contains("あいうえお"));
//! ```

pub mod codepoints;
pub mod data;

#[cfg(feature = "codepoints-jisx0201")]
pub mod jisx0201;

#[cfg(feature = "codepoints-jisx0208")]
pub mod jisx0208;

#[cfg(feature = "codepoints-jisx0208kanji")]
pub mod jisx0208kanji;

#[cfg(feature = "codepoints-jisx0213kanji")]
pub mod jisx0213kanji;

// Re-export main types
pub use codepoints::CodePoints;
// Re-export specific character sets when features are enabled
#[cfg(feature = "codepoints-jisx0201")]
pub use jisx0201::{JisX0201, Katakana as JisX0201Katakana, LatinLetters as JisX0201LatinLetters};
#[cfg(feature = "codepoints-jisx0208")]
pub use jisx0208::{
    BoxDrawingChars, CyrillicLetters, GreekLetters, Hiragana, JisX0208, Katakana, LatinLetters,
    SpecialChars,
};
#[cfg(feature = "codepoints-jisx0208kanji")]
pub use jisx0208kanji::JisX0208Kanji;
#[cfg(feature = "codepoints-jisx0213kanji")]
pub use jisx0213kanji::JisX0213Kanji;

pub mod validation;
