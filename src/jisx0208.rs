//! JIS X 0208 character set support
//!
//! This module provides support for JIS X 0208 character set (excluding kanji), which includes:
//! - Hiragana (ひらがな)
//! - Katakana (カタカナ)
//! - Latin letters (fullwidth)
//! - Greek letters
//! - Cyrillic letters
//! - Special characters
//! - Box drawing characters
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::jisx0208::{Hiragana, Katakana, LatinLetters};
//!
//! let hiragana = Hiragana::new();
//! assert!(hiragana.contains("あいうえお"));
//!
//! let katakana = Katakana::new();
//! assert!(katakana.contains("アイウエオ"));
//!
//! let latin = LatinLetters::new();
//! assert!(latin.contains("ＡＢＣ"));
//! ```

use crate::codepoints::CodePoints;
use std::sync::OnceLock;

/// JIS X 0208 Hiragana (ひらがな) character set
///
/// Contains all hiragana characters from 0x3041 to 0x3093.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::Hiragana;
///
/// let hiragana = Hiragana::new();
/// assert!(hiragana.contains("あいうえお"));
/// assert!(hiragana.contains("かきくけこ"));
/// assert!(!hiragana.contains("アイウエオ")); // Fullwidth katakana
/// ```
pub struct Hiragana {
    codepoints: CodePoints,
}

impl Hiragana {
    /// Creates a new Hiragana character set
    pub fn new() -> Self {
        use crate::data::jisx0208::HIRAGANA;
        Self {
            codepoints: CodePoints::new(HIRAGANA.to_vec()),
        }
    }

    /// Returns a cached instance of the Hiragana character set.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return a reference to the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::Hiragana;
    ///
    /// let hiragana1 = Hiragana::cached();
    /// let hiragana2 = Hiragana::cached();
    /// // Both references point to the same cached instance
    /// assert!(hiragana1.contains("あいうえお"));
    /// ```
    pub fn cached() -> &'static Hiragana {
        static HIRAGANA: OnceLock<Hiragana> = OnceLock::new();
        HIRAGANA.get_or_init(|| Hiragana::new())
    }

    /// Checks if the given string contains only hiragana characters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for Hiragana {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0208 Katakana (カタカナ) character set
///
/// Contains all katakana characters from 0x30A1 to 0x30F6.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::Katakana;
///
/// let katakana = Katakana::new();
/// assert!(katakana.contains("アイウエオ"));
/// assert!(katakana.contains("カキクケコ"));
/// assert!(!katakana.contains("あいうえお")); // Hiragana
/// ```
pub struct Katakana {
    codepoints: CodePoints,
}

impl Katakana {
    /// Creates a new Katakana character set
    pub fn new() -> Self {
        use crate::data::jisx0208::KATAKANA;
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
    /// use japanese_codepoints::jisx0208::Katakana;
    ///
    /// let katakana1 = Katakana::cached();
    /// let katakana2 = Katakana::cached();
    /// // Both references point to the same cached instance
    /// assert!(katakana1.contains("アイウエオ"));
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

/// JIS X 0208 Latin letters (fullwidth) character set
///
/// Contains fullwidth Latin letters and digits.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::LatinLetters;
///
/// let latin = LatinLetters::new();
/// assert!(latin.contains("ＡＢＣ"));
/// assert!(latin.contains("ａｂｃ"));
/// assert!(latin.contains("１２３"));
/// assert!(!latin.contains("ABC")); // Halfwidth
/// ```
pub struct LatinLetters {
    codepoints: CodePoints,
}

impl LatinLetters {
    /// Creates a new Latin letters character set
    pub fn new() -> Self {
        use crate::data::jisx0208::LATIN_LETTERS;
        Self {
            codepoints: CodePoints::new(LATIN_LETTERS.to_vec()),
        }
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

/// JIS X 0208 Greek letters character set
///
/// Contains uppercase and lowercase Greek letters.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::GreekLetters;
///
/// let greek = GreekLetters::new();
/// assert!(greek.contains("ΑΒΓ"));
/// assert!(greek.contains("αβγ"));
/// ```
pub struct GreekLetters {
    codepoints: CodePoints,
}

impl GreekLetters {
    /// Creates a new Greek letters character set
    pub fn new() -> Self {
        use crate::data::jisx0208::GREEK_LETTERS;
        Self {
            codepoints: CodePoints::new(GREEK_LETTERS.to_vec()),
        }
    }

    /// Checks if the given string contains only Greek letters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for GreekLetters {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0208 Cyrillic letters character set
///
/// Contains uppercase and lowercase Cyrillic letters.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::CyrillicLetters;
///
/// let cyrillic = CyrillicLetters::new();
/// assert!(cyrillic.contains("АБВ"));
/// assert!(cyrillic.contains("абв"));
/// ```
pub struct CyrillicLetters {
    codepoints: CodePoints,
}

impl CyrillicLetters {
    /// Creates a new Cyrillic letters character set
    pub fn new() -> Self {
        use crate::data::jisx0208::CYRILLIC_LETTERS;
        Self {
            codepoints: CodePoints::new(CYRILLIC_LETTERS.to_vec()),
        }
    }

    /// Checks if the given string contains only Cyrillic letters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for CyrillicLetters {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0208 Special characters character set
///
/// Contains various special symbols, punctuation, and mathematical symbols.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::SpecialChars;
///
/// let special = SpecialChars::new();
/// assert!(special.contains("、。"));
/// assert!(special.contains("☆★"));
/// assert!(special.contains("→←"));
/// ```
pub struct SpecialChars {
    codepoints: CodePoints,
}

impl SpecialChars {
    /// Creates a new Special characters set
    pub fn new() -> Self {
        use crate::data::jisx0208::SPECIAL_CHARS;
        Self {
            codepoints: CodePoints::new(SPECIAL_CHARS.to_vec()),
        }
    }

    /// Checks if the given string contains only special characters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for SpecialChars {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0208 Box drawing characters character set
///
/// Contains various box drawing and line characters.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::BoxDrawingChars;
///
/// let box_drawing = BoxDrawingChars::new();
/// assert!(box_drawing.contains("─│┌┐"));
/// assert!(box_drawing.contains("└┘├┤"));
/// ```
pub struct BoxDrawingChars {
    codepoints: CodePoints,
}

impl BoxDrawingChars {
    /// Creates a new Box drawing characters set
    pub fn new() -> Self {
        use crate::data::jisx0208::BOX_DRAWING_CHARS;
        Self {
            codepoints: CodePoints::new(BOX_DRAWING_CHARS.to_vec()),
        }
    }

    /// Checks if the given string contains only box drawing characters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for BoxDrawingChars {
    fn default() -> Self {
        Self::new()
    }
}

/// JIS X 0208 complete character set (excluding kanji)
///
/// Contains all JIS X 0208 characters (hiragana, katakana, latin, greek, cyrillic, special, box drawing).
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::JisX0208;
///
/// let jisx0208 = JisX0208::new();
/// assert!(jisx0208.contains("あいうえお"));
/// assert!(jisx0208.contains("アイウエオ"));
/// assert!(jisx0208.contains("ＡＢＣ"));
/// assert!(jisx0208.contains("ΑΒΓ"));
/// assert!(jisx0208.contains("、。☆★"));
/// ```
pub struct JisX0208 {
    codepoints: CodePoints,
}

impl JisX0208 {
    /// Creates a new JIS X 0208 character set
    pub fn new() -> Self {
        use crate::data::jisx0208::ALL_JISX0208;
        Self {
            codepoints: CodePoints::new(ALL_JISX0208.to_vec()),
        }
    }

    /// Checks if the given string contains only JIS X 0208 characters
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying code points
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }
}

impl Default for JisX0208 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hiragana_new() {
        let hiragana = Hiragana::new();
        assert!(!hiragana.codepoints().is_empty());
    }

    #[test]
    fn test_hiragana_contains() {
        let hiragana = Hiragana::new();
        assert!(hiragana.contains("あいうえお"));
        assert!(!hiragana.contains("アイウエオ"));
    }

    #[test]
    fn test_katakana_new() {
        let katakana = Katakana::new();
        assert!(!katakana.codepoints().is_empty());
    }

    #[test]
    fn test_katakana_contains() {
        let katakana = Katakana::new();
        assert!(katakana.contains("アイウエオ"));
        assert!(!katakana.contains("あいうえお"));
    }

    #[test]
    fn test_latin_letters_new() {
        let latin = LatinLetters::new();
        assert!(!latin.codepoints().is_empty());
    }

    #[test]
    fn test_latin_letters_contains() {
        let latin = LatinLetters::new();
        assert!(latin.contains("ＡＢＣ"));
        assert!(latin.contains("ａｂｃ"));
        assert!(latin.contains("１２３"));
        assert!(!latin.contains("ABC"));
    }

    #[test]
    fn test_greek_letters_new() {
        let greek = GreekLetters::new();
        assert!(!greek.codepoints().is_empty());
    }

    #[test]
    fn test_greek_letters_contains() {
        let greek = GreekLetters::new();
        assert!(greek.contains("ΑΒΓ"));
        assert!(greek.contains("αβγ"));
    }

    #[test]
    fn test_cyrillic_letters_new() {
        let cyrillic = CyrillicLetters::new();
        assert!(!cyrillic.codepoints().is_empty());
    }

    #[test]
    fn test_cyrillic_letters_contains() {
        let cyrillic = CyrillicLetters::new();
        assert!(cyrillic.contains("АБВ"));
        assert!(cyrillic.contains("абв"));
    }

    #[test]
    fn test_special_chars_new() {
        let special = SpecialChars::new();
        assert!(!special.codepoints().is_empty());
    }

    #[test]
    fn test_special_chars_contains() {
        let special = SpecialChars::new();
        assert!(special.contains("、。"));
        assert!(special.contains("☆★"));
        assert!(special.contains("→←"));
    }

    #[test]
    fn test_box_drawing_chars_new() {
        let box_drawing = BoxDrawingChars::new();
        assert!(!box_drawing.codepoints().is_empty());
    }

    #[test]
    fn test_box_drawing_chars_contains() {
        let box_drawing = BoxDrawingChars::new();
        assert!(box_drawing.contains("─│┌┐"));
        assert!(box_drawing.contains("└┘├┤"));
    }

    #[test]
    fn test_jisx0208_new() {
        let jisx0208 = JisX0208::new();
        assert!(!jisx0208.codepoints().is_empty());
    }

    #[test]
    fn test_jisx0208_contains() {
        let jisx0208 = JisX0208::new();
        assert!(jisx0208.contains("あいうえお"));
        assert!(jisx0208.contains("アイウエオ"));
        assert!(jisx0208.contains("ＡＢＣ"));
        assert!(jisx0208.contains("ΑΒΓ"));
        assert!(jisx0208.contains("、。☆★"));
        assert!(!jisx0208.contains("漢字")); // Kanji not included
    }

    #[test]
    fn test_cached_methods() {
        // Test that cached methods return the same instance
        let hiragana1 = Hiragana::cached();
        let hiragana2 = Hiragana::cached();
        assert!(std::ptr::eq(hiragana1, hiragana2));

        let katakana1 = Katakana::cached();
        let katakana2 = Katakana::cached();
        assert!(std::ptr::eq(katakana1, katakana2));

        // Test functionality is the same as non-cached versions
        assert_eq!(hiragana1.codepoints(), &Hiragana::new().codepoints);
        assert_eq!(katakana1.codepoints(), &Katakana::new().codepoints);

        // Test that cached instances work correctly
        assert!(hiragana1.contains("あいうえお"));
        assert!(katakana1.contains("アイウエオ"));
        assert!(!hiragana1.contains("アイウエオ")); // Katakana in hiragana checker
        assert!(!katakana1.contains("あいうえお")); // Hiragana in katakana checker
    }
}
