//! JIS X 0201 character set support.
//!
//! JIS X 0201 defines two sub-sets:
//!
//! | Type | Contents |
//! |---|---|
//! | [`LatinLetters`] | ASCII printable characters with `\` → `¥` and `~` → `‾` |
//! | [`Katakana`] | Halfwidth katakana (U+FF61–U+FF9F) |
//! | [`JisX0201`] | Union of the two above |
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::jisx0201::{Katakana, LatinLetters};
//!
//! assert!(Katakana::cached().contains("ｱｲｳｴｵ"));
//! assert!(LatinLetters::cached().contains("Hello¥‾"));
//! ```

// ── boilerplate macro (same pattern as jisx0208) ──────────────────────────────

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
    /// JIS X 0201 **Katakana** (halfwidth kana) character set.
    ///
    /// Contains all 63 halfwidth katakana characters from U+FF61 to U+FF9F,
    /// including the halfwidth punctuation marks `｡｢｣､･` and the voiced /
    /// semi-voiced marks `ﾞﾟ`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0201::Katakana;
    ///
    /// let k = Katakana::cached();
    /// assert!(k.contains("ｱｲｳｴｵ"));
    /// assert!(k.contains("｡｢｣､･"));
    /// assert!(!k.contains("あいうえお")); // fullwidth hiragana
    /// assert!(!k.contains("アイウエオ")); // fullwidth katakana
    /// ```
    Katakana => crate::data::jisx0201::KATAKANA
}

charset! {
    /// JIS X 0201 **Latin letters** character set.
    ///
    /// This is almost identical to ASCII printable (U+0020–U+007E), but with
    /// two substitutions mandated by the standard:
    ///
    /// * U+005C (`\`) is replaced by U+00A5 (`¥`, yen sign).
    /// * U+007E (`~`) is replaced by U+203E (`‾`, overline).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::jisx0201::LatinLetters;
    ///
    /// let l = LatinLetters::cached();
    /// assert!(l.contains("Hello World"));
    /// assert!(l.contains("¥100"));  // yen sign allowed
    /// assert!(l.contains("‾"));     // overline allowed
    /// assert!(!l.contains("\\")); // backslash NOT in JIS X 0201 Latin
    /// ```
    LatinLetters => crate::data::jisx0201::LATIN_LETTERS
}

// ── composite: full JIS X 0201 ────────────────────────────────────────────────

/// Complete JIS X 0201 character set (Latin letters ∪ halfwidth katakana).
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::jisx0201::JisX0201;
///
/// let full = JisX0201::cached();
/// assert!(full.contains("Hello¥｡｢｣ｱｲｳ"));
/// assert!(!full.contains("あいうえお")); // fullwidth hiragana
/// ```
pub struct JisX0201 {
    codepoints: crate::CodePoints,
}

impl JisX0201 {
    /// Creates a new JIS X 0201 character set by combining the Latin and
    /// Katakana sub-tables.
    pub fn new() -> Self {
        use crate::data::jisx0201::*;
        use std::collections::HashSet;

        let mut all = HashSet::new();
        all.extend(LATIN_LETTERS.iter());
        all.extend(KATAKANA.iter());

        Self {
            codepoints: crate::CodePoints::new(all.into_iter().collect()),
        }
    }

    /// Returns a cached static reference to the full JIS X 0201 character set.
    pub fn cached() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<JisX0201> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Returns `true` if every character in `text` belongs to JIS X 0201.
    pub fn contains(&self, text: &str) -> bool {
        self.codepoints.contains(text)
    }

    /// Returns the underlying [`crate::CodePoints`] collection.
    pub fn codepoints(&self) -> &crate::CodePoints {
        &self.codepoints
    }

    /// Validates that every character in `text` belongs to JIS X 0201.
    pub fn validate(&self, text: &str) -> Result<(), crate::validation::ValidationError> {
        self.codepoints.validate(text)
    }
}

impl Default for JisX0201 {
    fn default() -> Self {
        Self::new()
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_katakana() {
        let k = Katakana::new();
        assert!(!k.codepoints().is_empty());
        assert!(k.contains("ｱｲｳｴｵ"));
        assert!(k.contains("｡｢｣､･"));
        assert!(!k.contains("あいうえお")); // fullwidth hiragana
        assert!(!k.contains("アイウエオ")); // fullwidth katakana
        assert!(!k.contains("Hello")); // latin
    }

    #[test]
    fn test_latin_letters() {
        let l = LatinLetters::new();
        assert!(!l.codepoints().is_empty());
        assert!(l.contains("Hello"));
        assert!(l.contains("¥100")); // yen sign
        assert!(l.contains("‾")); // overline
        assert!(!l.contains("\\")); // backslash replaced by ¥
        assert!(!l.contains("ｱｲｳｴｵ")); // halfwidth katakana
        assert!(!l.contains("あいうえお")); // fullwidth hiragana
    }

    #[test]
    fn test_jisx0201_composite() {
        let full = JisX0201::new();
        assert!(full.contains("Hello¥"));
        assert!(full.contains("ｱｲｳｴｵ"));
        assert!(full.contains("¥｡｢｣"));
        assert!(!full.contains("あいうえお"));
        assert!(!full.contains("アイウエオ"));
        assert!(!full.contains("漢字"));
    }

    #[test]
    fn test_cached_identity() {
        assert!(std::ptr::eq(Katakana::cached(), Katakana::cached()));
        assert!(std::ptr::eq(LatinLetters::cached(), LatinLetters::cached()));
        assert!(std::ptr::eq(JisX0201::cached(), JisX0201::cached()));
    }

    #[test]
    fn test_validate() {
        assert!(Katakana::cached().validate("ｱｲｳ").is_ok());
        let err = Katakana::cached().validate("ｱｲA").unwrap_err();
        assert_eq!(err.code_point, 0x41); // 'A'
        assert_eq!(err.position, 2);
    }
}
