//! JIS X 0208 character set support (excluding kanji).
//!
//! This module provides the following character sets defined by JIS X 0208:
//!
//! | Type | Description |
//! |---|---|
//! | [`Hiragana`] | Hiragana (ひらがな), U+3041–U+3093 |
//! | [`Katakana`] | Katakana (カタカナ), U+30A1–U+30F6 |
//! | [`LatinLetters`] | Fullwidth Latin letters and digits |
//! | [`GreekLetters`] | Uppercase and lowercase Greek letters |
//! | [`CyrillicLetters`] | Uppercase and lowercase Cyrillic letters |
//! | [`SpecialChars`] | Punctuation, symbols, arrows, stars … |
//! | [`BoxDrawingChars`] | Box-drawing and line characters |
//! | [`JisX0208`] | Union of all of the above |
//!
//! Every type exposes the same four-method interface:
//!
//! * `new()` — allocate a fresh instance.
//! * `cached()` — return a `&'static` reference (zero-cost after the first
//!   call).
//! * `contains(text)` — check whether every character in `text` is in the set.
//! * `codepoints()` — access the underlying [`CodePoints`][crate::CodePoints].
//! * `validate(text)` — like `contains`, but returns a
//!   [`ValidationError`][crate::ValidationError] on failure.
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::jisx0208::{Hiragana, Katakana, LatinLetters};
//!
//! assert!(Hiragana::cached().contains("あいうえお"));
//! assert!(Katakana::cached().contains("アイウエオ"));
//! assert!(LatinLetters::cached().contains("ＡＢＣ"));
//! ```

// ── boilerplate macro ─────────────────────────────────────────────────────────
// Generates a character-set struct with new / cached / contains / codepoints /
// validate / Default.  Only used within this module.

macro_rules! charset {
    (
        $( #[$doc:meta] )*
        $name:ident => $data:path
    ) => {
        $( #[$doc] )*
        pub struct $name {
            codepoints: crate::CodePoints,
        }

        impl $name {
            /// Creates a new instance of this character set.
            pub fn new() -> Self {
                Self {
                    codepoints: crate::CodePoints::from_slice($data),
                }
            }

            /// Returns a cached static reference to this character set.
            ///
            /// The instance is initialized on first access via
            /// [`std::sync::OnceLock`]; subsequent calls return the same
            /// reference with no allocation.
            pub fn cached() -> &'static Self {
                static INSTANCE: std::sync::OnceLock<$name> = std::sync::OnceLock::new();
                INSTANCE.get_or_init(Self::new)
            }

            /// Returns `true` if every character in `text` belongs to this set.
            pub fn contains(&self, text: &str) -> bool {
                self.codepoints.contains(text)
            }

            /// Returns the underlying [`crate::CodePoints`] collection.
            pub fn codepoints(&self) -> &crate::CodePoints {
                &self.codepoints
            }

            /// Validates that every character in `text` belongs to this set.
            ///
            /// Returns `Ok(())` on success, or a [`crate::ValidationError`]
            /// identifying the first character that does not belong.
            pub fn validate(&self, text: &str) -> Result<(), crate::validation::ValidationError> {
                self.codepoints.validate(text)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

// ── leaf character sets ───────────────────────────────────────────────────────

charset! {
    /// JIS X 0208 **Hiragana** (ひらがな) character set.
    ///
    /// Contains all 83 hiragana characters from U+3041 to U+3093.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::Hiragana;
    ///
    /// let h = Hiragana::cached();
    /// assert!(h.contains("あいうえお"));
    /// assert!(!h.contains("アイウエオ")); // katakana
    /// ```
    Hiragana => crate::data::jisx0208::HIRAGANA
}

charset! {
    /// JIS X 0208 **Katakana** (カタカナ) character set.
    ///
    /// Contains katakana characters from U+30A1 to U+30F6.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::Katakana;
    ///
    /// let k = Katakana::cached();
    /// assert!(k.contains("アイウエオ"));
    /// assert!(!k.contains("あいうえお")); // hiragana
    /// ```
    Katakana => crate::data::jisx0208::KATAKANA
}

charset! {
    /// JIS X 0208 **fullwidth Latin** letters and digits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::LatinLetters;
    ///
    /// let l = LatinLetters::cached();
    /// assert!(l.contains("ＡＢＣａｂｃ１２３"));
    /// assert!(!l.contains("ABC")); // halfwidth
    /// ```
    LatinLetters => crate::data::jisx0208::LATIN_LETTERS
}

charset! {
    /// JIS X 0208 **Greek** letters (upper- and lower-case).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::GreekLetters;
    ///
    /// assert!(GreekLetters::cached().contains("ΑΒΓαβγ"));
    /// ```
    GreekLetters => crate::data::jisx0208::GREEK_LETTERS
}

charset! {
    /// JIS X 0208 **Cyrillic** letters (upper- and lower-case).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::CyrillicLetters;
    ///
    /// assert!(CyrillicLetters::cached().contains("АБВабв"));
    /// ```
    CyrillicLetters => crate::data::jisx0208::CYRILLIC_LETTERS
}

charset! {
    /// JIS X 0208 **special characters** — punctuation, symbols, arrows, stars,
    /// and similar glyphs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::SpecialChars;
    ///
    /// assert!(SpecialChars::cached().contains("、。☆★→←"));
    /// ```
    SpecialChars => crate::data::jisx0208::SPECIAL_CHARS
}

charset! {
    /// JIS X 0208 **box-drawing** characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0208::BoxDrawingChars;
    ///
    /// assert!(BoxDrawingChars::cached().contains("─│┌┐└┘├┤"));
    /// ```
    BoxDrawingChars => crate::data::jisx0208::BOX_DRAWING_CHARS
}

// ── composite: full JIS X 0208 (non-kanji) ────────────────────────────────────

/// Complete JIS X 0208 character set **excluding kanji**.
///
/// This is the union of [`Hiragana`], [`Katakana`], [`LatinLetters`],
/// [`GreekLetters`], [`CyrillicLetters`], [`SpecialChars`], and
/// [`BoxDrawingChars`].
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0208::JisX0208;
///
/// let full = JisX0208::cached();
/// assert!(full.contains("あいうアイウＡＢＣΑΒΓАБВ、。☆─│┌"));
/// assert!(!full.contains("漢字")); // kanji not included
/// ```
pub struct JisX0208 {
    codepoints: crate::CodePoints,
}

impl JisX0208 {
    /// Creates a new JIS X 0208 (non-kanji) character set by combining all
    /// sub-tables.
    pub fn new() -> Self {
        use crate::data::jisx0208::*;
        use std::collections::HashSet;

        let mut all = HashSet::new();
        all.extend(HIRAGANA.iter());
        all.extend(KATAKANA.iter());
        all.extend(LATIN_LETTERS.iter());
        all.extend(GREEK_LETTERS.iter());
        all.extend(CYRILLIC_LETTERS.iter());
        all.extend(SPECIAL_CHARS.iter());
        all.extend(BOX_DRAWING_CHARS.iter());

        Self {
            codepoints: crate::CodePoints::new(all.into_iter().collect()),
        }
    }

    /// Returns a cached static reference to the full JIS X 0208 (non-kanji)
    /// character set.
    pub fn cached() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<JisX0208> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Returns `true` if every character in `text` belongs to JIS X 0208.
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying [`crate::CodePoints`] collection.
    pub fn codepoints(&self) -> &crate::CodePoints {
        &self.codepoints
    }

    /// Validates that every character in `text` belongs to JIS X 0208.
    pub fn validate(&self, text: &str) -> Result<(), crate::validation::ValidationError> {
        self.codepoints.validate(text)
    }
}

impl Default for JisX0208 {
    fn default() -> Self {
        Self::new()
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── leaf sets ───────────────────────────────────────────────────────

    #[test]
    fn test_hiragana() {
        let h = Hiragana::new();
        assert!(!h.codepoints().is_empty());
        assert!(h.contains("あいうえお"));
        assert!(!h.contains("アイウエオ"));
    }

    #[test]
    fn test_katakana() {
        let k = Katakana::new();
        assert!(!k.codepoints().is_empty());
        assert!(k.contains("アイウエオ"));
        assert!(!k.contains("あいうえお"));
    }

    #[test]
    fn test_latin_letters() {
        let l = LatinLetters::new();
        assert!(!l.codepoints().is_empty());
        assert!(l.contains("ＡＢＣａｂｃ１２３"));
        assert!(!l.contains("ABC")); // halfwidth
    }

    #[test]
    fn test_greek_letters() {
        let g = GreekLetters::new();
        assert!(!g.codepoints().is_empty());
        assert!(g.contains("ΑΒΓαβγ"));
    }

    #[test]
    fn test_cyrillic_letters() {
        let c = CyrillicLetters::new();
        assert!(!c.codepoints().is_empty());
        assert!(c.contains("АБВабв"));
    }

    #[test]
    fn test_special_chars() {
        let s = SpecialChars::new();
        assert!(!s.codepoints().is_empty());
        assert!(s.contains("、。☆★→←"));
    }

    #[test]
    fn test_box_drawing_chars() {
        let b = BoxDrawingChars::new();
        assert!(!b.codepoints().is_empty());
        assert!(b.contains("─│┌┐└┘├┤"));
    }

    // ── composite ───────────────────────────────────────────────────────

    #[test]
    fn test_jisx0208_composite() {
        let full = JisX0208::new();
        assert!(!full.codepoints().is_empty());
        assert!(full.contains("あいうえお")); // hiragana
        assert!(full.contains("アイウエオ")); // katakana
        assert!(full.contains("ＡＢＣ")); // fullwidth latin
        assert!(full.contains("ΑΒΓ")); // greek
        assert!(full.contains("АБВ")); // cyrillic
        assert!(full.contains("、。☆★")); // special
        assert!(full.contains("─│┌┐")); // box drawing
        assert!(!full.contains("漢字")); // kanji excluded
    }

    // ── cached identity ─────────────────────────────────────────────────

    #[test]
    fn test_cached_identity() {
        assert!(std::ptr::eq(Hiragana::cached(), Hiragana::cached()));
        assert!(std::ptr::eq(Katakana::cached(), Katakana::cached()));
        assert!(std::ptr::eq(LatinLetters::cached(), LatinLetters::cached()));
        assert!(std::ptr::eq(GreekLetters::cached(), GreekLetters::cached()));
        assert!(std::ptr::eq(
            CyrillicLetters::cached(),
            CyrillicLetters::cached()
        ));
        assert!(std::ptr::eq(SpecialChars::cached(), SpecialChars::cached()));
        assert!(std::ptr::eq(
            BoxDrawingChars::cached(),
            BoxDrawingChars::cached()
        ));
        assert!(std::ptr::eq(JisX0208::cached(), JisX0208::cached()));
    }

    #[test]
    fn test_cached_equals_new() {
        assert_eq!(Hiragana::cached().codepoints(), &Hiragana::new().codepoints);
        assert_eq!(Katakana::cached().codepoints(), &Katakana::new().codepoints);
    }

    // ── validate ────────────────────────────────────────────────────────

    #[test]
    fn test_validate_hiragana() {
        assert!(Hiragana::cached().validate("あいうえお").is_ok());
        let err = Hiragana::cached().validate("あいA").unwrap_err();
        assert_eq!(err.code_point, 0x41); // 'A'
        assert_eq!(err.position, 2);
    }
}
