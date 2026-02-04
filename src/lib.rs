//! # japanese-codepoints
//!
//! A Rust library for validating and working with Japanese character code
//! points based on JIS standards.
//!
//! ## Character sets
//!
//! | Feature | Module | Description |
//! |---|---|---|
//! | *(default)* | — | ASCII control / printable via [`CodePoints`] |
//! | `codepoints-jisx0201` | [`jisx0201`] | Latin letters and halfwidth katakana |
//! | `codepoints-jisx0208` | [`jisx0208`] | Hiragana, katakana, Latin, Greek, Cyrillic, symbols |
//! | `codepoints-jisx0208kanji` | [`jisx0208kanji`] | 6 355 kanji (JIS X 0208 Level 1 & 2) |
//! | `codepoints-jisx0213kanji` | [`jisx0213kanji`] | 10 050 kanji (JIS X 0213 Level 1–4) |
//! | `full` | — | All of the above |
//!
//! ## Quick start
//!
//! ```rust
//! use japanese_codepoints::CodePoints;
//!
//! let allowed = CodePoints::new(vec![0x3041, 0x3042]); // ぁ, あ
//! assert!(allowed.contains("あ"));
//! assert!(!allowed.contains("う"));
//! ```
//!
//! ## Multi-set validation
//!
//! Use [`contains_all_in_any`] to check whether every character in a string
//! belongs to at least one of several character sets:
//!
//! ```rust
//! use japanese_codepoints::{CodePoints, contains_all_in_any};
//!
//! let hiragana = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
//! let katakana = CodePoints::new(vec![0x30A2, 0x30A4]); // ア, イ
//! assert!(contains_all_in_any("あア", &[&hiragana, &katakana]));
//! ```
//!
//! For a version that returns a structured error, see
//! [`validation::validate_all_in_any`].

pub mod codepoints;
pub mod data;
pub mod validation;

#[cfg(feature = "codepoints-jisx0201")]
pub mod jisx0201;

#[cfg(feature = "codepoints-jisx0208")]
pub mod jisx0208;

#[cfg(feature = "codepoints-jisx0208kanji")]
pub mod jisx0208kanji;

#[cfg(feature = "codepoints-jisx0213kanji")]
pub mod jisx0213kanji;

// ── re-exports ────────────────────────────────────────────────────────────────

pub use codepoints::{contains_all_in_any, CodePoints};
pub use validation::ValidationError;

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
