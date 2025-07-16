//! JIS X 0201 character set support
//!
//! This module provides support for JIS X 0201 character set, which includes:
//! - Latin letters (ASCII + some special characters)
//! - Halfwidth katakana characters
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::jisx0201::{Katakana, LatinLetters};
//!
//! let katakana = Katakana::new();
//! assert!(katakana.contains("ｱｲｳｴｵ"));
//!
//! let latin = LatinLetters::new();
//! assert!(latin.contains("Hello World"));
//! assert!(latin.contains("¥")); // Yen sign
//! ```

use crate::codepoints::CodePoints;
use std::sync::OnceLock;

/// JIS X 0201 Katakana (halfwidth kana) character set
///
/// Contains all halfwidth katakana characters from 0xFF61 to 0xFF9F.
/// This includes special characters like ｡｢｣､･ and all halfwidth katakana.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0201::Katakana;
///
/// let katakana = Katakana::new();
/// assert!(katakana.contains("ｱｲｳｴｵ"));
/// assert!(katakana.contains("｡｢｣､･"));
/// assert!(!katakana.contains("あいうえお")); // Fullwidth hiragana
/// ```
pub struct Katakana {
    codepoints: CodePoints,
}

impl Katakana {
    /// Creates a new Katakana character set
    pub fn new() -> Self {
        use crate::data::jisx0201::KATAKANA;
        Self {
            codepoints: CodePoints::new(KATAKANA.to_vec()),
        }
    }

    /// Returns a cached instance of the Katakana character set.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return a reference to the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0201::Katakana;
    ///
    /// let katakana1 = Katakana::cached();
    /// let katakana2 = Katakana::cached();
    /// // Both references point to the same cached instance
    /// assert!(katakana1.contains("ｱｲｳｴｵ"));
    /// ```
    pub fn cached() -> &'static Katakana {
        static KATAKANA: OnceLock<Katakana> = OnceLock::new();
        KATAKANA.get_or_init(|| Katakana::new())
    }

    /// Checks if the given string contains only katakana characters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for Katakana {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0201 Latin letters character set
///
/// Contains ASCII printable characters plus some JIS-specific characters:
/// - Standard ASCII printable characters (0x0020-0x007E)
/// - Yen sign (¥, 0x00A5)
/// - Overline (‾, 0x203E)
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0201::LatinLetters;
///
/// let latin = LatinLetters::new();
/// assert!(latin.contains("Hello World"));
/// assert!(latin.contains("¥")); // Yen sign
/// assert!(latin.contains("‾")); // Overline
/// assert!(!latin.contains("あいうえお")); // Japanese characters
/// ```
pub struct LatinLetters {
    codepoints: CodePoints,
}

impl LatinLetters {
    /// Creates a new Latin letters character set
    pub fn new() -> Self {
        use crate::data::jisx0201::LATIN_LETTERS;
        Self {
            codepoints: CodePoints::new(LATIN_LETTERS.to_vec()),
        }
    }

    /// Returns a cached instance of the LatinLetters character set.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return a reference to the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0201::LatinLetters;
    ///
    /// let latin1 = LatinLetters::cached();
    /// let latin2 = LatinLetters::cached();
    /// // Both references point to the same cached instance
    /// assert!(latin1.contains("Hello¥"));
    /// ```
    pub fn cached() -> &'static LatinLetters {
        static LATIN_LETTERS: OnceLock<LatinLetters> = OnceLock::new();
        LATIN_LETTERS.get_or_init(|| LatinLetters::new())
    }

    /// Checks if the given string contains only Latin letters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for LatinLetters {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0201 complete character set
///
/// Contains all JIS X 0201 characters (Latin letters + Katakana).
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0201::JisX0201;
///
/// let jisx0201 = JisX0201::new();
/// assert!(jisx0201.contains("Hello World"));
/// assert!(jisx0201.contains("ｱｲｳｴｵ"));
/// assert!(jisx0201.contains("¥｡｢｣､･"));
/// ```
pub struct JisX0201 {
    codepoints: CodePoints,
}

impl JisX0201 {
    /// Creates a new JIS X 0201 character set
    pub fn new() -> Self {
        use crate::data::jisx0201::ALL_JISX0201;
        Self {
            codepoints: CodePoints::new(ALL_JISX0201.to_vec()),
        }
    }

    /// Checks if the given string contains only JIS X 0201 characters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for JisX0201 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "codepoints-jisx0201")]
    fn test_katakana_new() {
        let katakana = Katakana::new();
        assert!(!katakana.codepoints().is_empty());
    }

    #[test]
    #[cfg(feature = "codepoints-jisx0201")]
    fn test_latin_letters_new() {
        let latin = LatinLetters::new();
        assert!(!latin.codepoints().is_empty());
    }

    #[test]
    #[cfg(feature = "codepoints-jisx0201")]
    fn test_katakana_contains() {
        let katakana = Katakana::new();
        assert!(katakana.contains("ｱｲｳｴｵ"));
        assert!(katakana.contains("｡｢｣､･"));
        assert!(!katakana.contains("あいうえお")); // Fullwidth hiragana
        assert!(!katakana.contains("アイウエオ")); // Fullwidth katakana
        assert!(!katakana.contains("Hello")); // Latin letters
    }

    #[test]
    #[cfg(feature = "codepoints-jisx0201")]
    fn test_latin_letters_contains() {
        let latin = LatinLetters::new();
        assert!(latin.contains("Hello"));
        assert!(latin.contains("¥100"));
        assert!(latin.contains("‾")); // Overline
        assert!(!latin.contains("ｱｲｳｴｵ")); // Halfwidth katakana
        assert!(!latin.contains("あいうえお")); // Fullwidth hiragana
    }
}
