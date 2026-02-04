//! JIS X 0213 Kanji character set support.
//!
//! JIS X 0213:2004 extends JIS X 0208 with two additional tiers:
//!
//! | Tier | Characters | Notes |
//! |---|---|---|
//! | Level 1 | 2 965 | Same as JIS X 0208 Level 1 |
//! | Level 2 | 3 390 | Same as JIS X 0208 Level 2 |
//! | Level 3 | 1 259 | New in JIS X 0213 (Plane 1) |
//! | Level 4 | 2 436 | New in JIS X 0213 (Plane 2) |
//!
//! [`JisX0213Kanji`] covers all four tiers (10 050 characters in total).
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::jisx0213kanji::JisX0213Kanji;
//!
//! let kanji = JisX0213Kanji::cached();
//! assert!(kanji.contains("亜愛安"));          // Level 1
//! assert!(kanji.contains("堯槇遙瑤凜熙"));   // Level 3–4 additions
//! ```

use crate::CodePoints;

/// JIS X 0213 Kanji character set (Level 1 + 2 + 3 + 4).
///
/// Contains 10 050 kanji characters as specified in JIS X 0213:2004.
#[derive(Debug, Clone)]
pub struct JisX0213Kanji {
    codepoints: CodePoints,
}

impl JisX0213Kanji {
    /// Creates a new JIS X 0213 Kanji character set.
    pub fn new() -> Self {
        Self {
            codepoints: CodePoints::from_slice(crate::data::jisx0213kanji::JISX0213_KANJI),
        }
    }

    /// Returns a cached static reference to the JIS X 0213 Kanji set.
    ///
    /// The instance is initialized on first access; subsequent calls return
    /// the same reference with no allocation.
    pub fn cached() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<JisX0213Kanji> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Returns `true` if every character in `text` is a JIS X 0213 kanji.
    pub fn contains(&self, s: &str) -> bool {
        self.codepoints.contains(s)
    }

    /// Returns the underlying [`CodePoints`] collection.
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }

    /// Returns all kanji code points as a `Vec<u32>`.
    ///
    /// > **Note:** the order of elements is **not** guaranteed.  Use `.len()`
    /// > for counting; prefer [`Self::codepoints`] for membership checks.
    pub fn codepoints_vec(&self) -> Vec<u32> {
        self.codepoints.iter().copied().collect()
    }

    /// Validates that every character in `text` is a JIS X 0213 kanji.
    ///
    /// Returns `Ok(())` on success, or a [`crate::ValidationError`]
    /// identifying the first non-kanji character.
    pub fn validate(&self, text: &str) -> Result<(), crate::validation::ValidationError> {
        self.codepoints.validate(text)
    }
}

impl Default for JisX0213Kanji {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(JisX0213Kanji::new().codepoints_vec().len(), 10050);
    }

    #[test]
    fn test_default_equals_new() {
        assert_eq!(
            JisX0213Kanji::default().codepoints_vec().len(),
            JisX0213Kanji::new().codepoints_vec().len()
        );
    }

    #[test]
    fn test_level1_kanji() {
        let kanji = JisX0213Kanji::new();
        assert!(kanji.contains("亜愛安以伊位一乙王黄"));
    }

    #[test]
    fn test_level3_4_kanji() {
        let kanji = JisX0213Kanji::new();
        // Characters unique to JIS X 0213
        assert!(kanji.contains("堯槇遙瑤凜熙"));
        assert!(kanji.contains("龕龠龢"));
        assert!(kanji.contains("俱剝頰"));
    }

    #[test]
    fn test_non_kanji_rejected() {
        let kanji = JisX0213Kanji::new();
        assert!(!kanji.contains("亜A愛"));
        assert!(!kanji.contains("123"));
        assert!(kanji.contains("")); // empty string
    }

    #[test]
    fn test_cached_identity() {
        assert!(std::ptr::eq(
            JisX0213Kanji::cached(),
            JisX0213Kanji::cached()
        ));
    }

    #[test]
    fn test_validate() {
        assert!(JisX0213Kanji::cached().validate("亜愛").is_ok());
        let err = JisX0213Kanji::cached().validate("亜x").unwrap_err();
        assert_eq!(err.code_point, 0x78); // 'x'
        assert_eq!(err.position, 1);
    }
}
