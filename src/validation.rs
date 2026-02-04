//! Validation utilities for code-point collections.
//!
//! This module provides:
//!
//! * [`ValidationError`] – a structured error returned when a string contains
//!   characters outside an allowed set.
//! * [`validate_all_in_any`] – validate text against the *union* of several
//!   character sets simultaneously.
//! * Convenience macros for common Japanese character-set checks.

use std::fmt;

use crate::CodePoints;

// ── error type ────────────────────────────────────────────────────────────────

/// Describes a single code-point validation failure.
///
/// A `ValidationError` pinpoints the exact character that caused the check to
/// fail, its position in the input string, and a human-readable message.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::CodePoints;
///
/// let cp = CodePoints::ascii_printable();
/// let err = cp.validate("hello\0world").unwrap_err();
/// assert_eq!(err.code_point, 0);   // NULL character
/// assert_eq!(err.position, 5);     // index of '\0'
/// assert!(err.to_string().contains("U+0000"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// The Unicode code point that is not allowed by the character set.
    pub code_point: u32,
    /// Zero-based *character* index (not byte index) within the input string.
    pub position: usize,
    /// A human-readable description of the error.
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ValidationError {}

impl ValidationError {
    /// Creates a `ValidationError` for the given code point and character index.
    pub fn new(code_point: u32, position: usize) -> Self {
        let ch = char::from_u32(code_point).unwrap_or('\u{FFFD}');
        Self {
            code_point,
            position,
            message: format!(
                "invalid character '{}' (U+{:04X}) at position {}",
                ch, code_point, position
            ),
        }
    }

    /// Creates a `ValidationError` with an explicit message, overriding the
    /// default formatting.
    pub fn with_message(code_point: u32, position: usize, message: impl Into<String>) -> Self {
        Self {
            code_point,
            position,
            message: message.into(),
        }
    }
}

// ── multi-set validation ──────────────────────────────────────────────────────

/// Validates that **every** character in `text` belongs to **at least one** of
/// the provided character sets.
///
/// This is the idiomatic way to validate text that may legitimately contain
/// characters from multiple scripts — for example Japanese hiragana mixed with
/// ASCII punctuation.
///
/// # Edge cases
///
/// * An empty `text` returns `Ok(())` (vacuously valid).
/// * An empty `sets` slice returns `Err` for any non-empty `text`.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::{CodePoints, validation::validate_all_in_any};
///
/// let hiragana = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
/// let katakana = CodePoints::new(vec![0x30A2]);          // ア
///
/// assert!(validate_all_in_any("あア", &[&hiragana, &katakana]).is_ok());
/// assert!(validate_all_in_any("あx", &[&hiragana, &katakana]).is_err());
/// ```
pub fn validate_all_in_any(text: &str, sets: &[&CodePoints]) -> Result<(), ValidationError> {
    for (i, c) in text.chars().enumerate() {
        if !sets.iter().any(|set| set.contains_char(c)) {
            return Err(ValidationError::new(c as u32, i));
        }
    }
    Ok(())
}

// ── macros ────────────────────────────────────────────────────────────────────

/// Validates that `$value` contains only code points present in `$codepoints`.
///
/// Returns `Ok(())` on success; `Err([`ValidationError`])` on failure.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::{validate_codepoints, CodePoints};
///
/// let cp = CodePoints::ascii_printable();
/// assert!(validate_codepoints!("hello", &cp).is_ok());
/// assert!(validate_codepoints!("hello\0", &cp).is_err());
/// ```
#[macro_export]
macro_rules! validate_codepoints {
    ($value:expr, $codepoints:expr) => {
        $codepoints.validate($value)
    };
}

/// Extended validation with additional patterns.
///
/// ## Custom error message
///
/// ```rust
/// use japanese_codepoints::{validate_codepoints_advanced, CodePoints};
///
/// let cp = CodePoints::ascii_printable();
/// let r = validate_codepoints_advanced!("hi", &cp, "Only ASCII allowed");
/// assert!(r.is_ok());
/// ```
///
/// ## Detailed (default position-aware message)
///
/// ```rust
/// use japanese_codepoints::{validate_codepoints_advanced, CodePoints};
///
/// let cp = CodePoints::ascii_printable();
/// let r = validate_codepoints_advanced!("hi\0there", detailed &cp);
/// assert!(r.is_err());
/// ```
#[macro_export]
macro_rules! validate_codepoints_advanced {
    // Custom error message — overrides the default ValidationError message.
    ($value:expr, $codepoints:expr, $error_msg:expr) => {
        $codepoints.validate($value).map_err(|mut e| {
            e.message = $error_msg.to_string();
            e
        })
    };

    // Detailed — identical to validate_codepoints! but kept for symmetry.
    ($value:expr, detailed $codepoints:expr) => {
        $codepoints.validate($value)
    };
}

// ── feature-gated convenience macros ─────────────────────────────────────────

/// Validates that `$value` contains only JIS X 0208 **hiragana** characters.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "codepoints-jisx0208")]
/// use japanese_codepoints::validate_hiragana;
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_hiragana!("あいうえお").is_ok());
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_hiragana!("Hello").is_err());
/// ```
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_hiragana {
    ($value:expr) => {
        $crate::jisx0208::Hiragana::cached().validate($value)
    };
}

/// Validates that `$value` contains only JIS X 0208 **katakana** characters.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "codepoints-jisx0208")]
/// use japanese_codepoints::validate_katakana;
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_katakana!("アイウエオ").is_ok());
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_katakana!("あいうえお").is_err());
/// ```
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_katakana {
    ($value:expr) => {
        $crate::jisx0208::Katakana::cached().validate($value)
    };
}

/// Validates that `$value` contains only **hiragana or katakana** characters.
///
/// Each character must belong to at least one of the two sets; mixing is
/// allowed.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "codepoints-jisx0208")]
/// use japanese_codepoints::validate_japanese_kana;
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_japanese_kana!("あいアイ").is_ok());
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_japanese_kana!("Hello").is_err());
/// ```
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_japanese_kana {
    ($value:expr) => {{
        let sets: &[&$crate::CodePoints] = &[
            $crate::jisx0208::Hiragana::cached().codepoints(),
            $crate::jisx0208::Katakana::cached().codepoints(),
        ];
        $crate::validation::validate_all_in_any($value, sets)
    }};
}

/// Validates that `$value` contains only **hiragana, katakana, or ASCII
/// printable** characters.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "codepoints-jisx0208")]
/// use japanese_codepoints::validate_japanese_mixed;
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_japanese_mixed!("こんにちはHello").is_ok());
/// # #[cfg(feature = "codepoints-jisx0208")]
/// assert!(validate_japanese_mixed!("漢字").is_err());
/// ```
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_japanese_mixed {
    ($value:expr) => {{
        let sets: &[&$crate::CodePoints] = &[
            $crate::jisx0208::Hiragana::cached().codepoints(),
            $crate::jisx0208::Katakana::cached().codepoints(),
            $crate::CodePoints::ascii_printable_cached(),
        ];
        $crate::validation::validate_all_in_any($value, sets)
    }};
}

/// Validates that `$value` contains only JIS X 0201 **halfwidth katakana**.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "codepoints-jisx0201")]
/// use japanese_codepoints::validate_jisx0201_katakana;
/// # #[cfg(feature = "codepoints-jisx0201")]
/// assert!(validate_jisx0201_katakana!("ｱｲｳｴｵ").is_ok());
/// # #[cfg(feature = "codepoints-jisx0201")]
/// assert!(validate_jisx0201_katakana!("アイウエオ").is_err());
/// ```
#[cfg(feature = "codepoints-jisx0201")]
#[macro_export]
macro_rules! validate_jisx0201_katakana {
    ($value:expr) => {
        $crate::jisx0201::Katakana::cached().validate($value)
    };
}

/// Validates that `$value` contains only JIS X 0201 **Latin letters**.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "codepoints-jisx0201")]
/// use japanese_codepoints::validate_jisx0201_latin;
/// # #[cfg(feature = "codepoints-jisx0201")]
/// assert!(validate_jisx0201_latin!("Hello¥").is_ok());
/// # #[cfg(feature = "codepoints-jisx0201")]
/// assert!(validate_jisx0201_latin!("こんにちは").is_err());
/// ```
#[cfg(feature = "codepoints-jisx0201")]
#[macro_export]
macro_rules! validate_jisx0201_latin {
    ($value:expr) => {
        $crate::jisx0201::LatinLetters::cached().validate($value)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let e = ValidationError::new(0x3046, 2);
        assert!(e.to_string().contains("U+3046"));
        assert!(e.to_string().contains("position 2"));
    }

    #[test]
    fn test_validation_error_with_message() {
        let e = ValidationError::with_message(0x41, 0, "custom msg");
        assert_eq!(e.message, "custom msg");
        assert_eq!(e.code_point, 0x41);
    }

    #[test]
    fn test_validate_all_in_any() {
        let hira = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let kata = CodePoints::new(vec![0x30A2, 0x30A4]); // ア, イ

        assert!(validate_all_in_any("あア", &[&hira, &kata]).is_ok());
        assert!(validate_all_in_any("あい", &[&hira]).is_ok());
        assert!(validate_all_in_any("", &[&hira]).is_ok());

        let err = validate_all_in_any("あx", &[&hira, &kata]).unwrap_err();
        assert_eq!(err.code_point, 0x78); // 'x'
        assert_eq!(err.position, 1);
    }

    #[test]
    fn test_validate_all_in_any_empty_sets() {
        assert!(validate_all_in_any("", &[]).is_ok()); // empty text, empty sets → vacuously ok
        assert!(validate_all_in_any("a", &[]).is_err());
    }

    #[test]
    fn test_validate_all_in_any_three_sets() {
        let hira = CodePoints::new(vec![0x3042]); // あ
        let kata = CodePoints::new(vec![0x30A2]); // ア
        let ascii = CodePoints::ascii_printable();

        assert!(validate_all_in_any("あアA", &[&hira, &kata, &ascii]).is_ok());
        // π (U+03C0) not in any set
        assert!(validate_all_in_any("あアAπ", &[&hira, &kata, &ascii]).is_err());
    }
}
