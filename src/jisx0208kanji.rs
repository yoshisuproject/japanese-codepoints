//! JIS X 0208 Kanji character set support.
//!
//! JIS X 0208 defines two tiers of kanji:
//!
//! * **Level 1** — 2 965 characters (rows 16–47), sorted by reading.
//! * **Level 2** — 3 390 characters (rows 48–84), sorted by radical.
//!
//! [`JisX0208Kanji`] covers both tiers (6 355 characters in total).
//!
//! # Examples
//!
//! ```rust
//! use japanese_codepoints::jisx0208kanji::JisX0208Kanji;
//!
//! let kanji = JisX0208Kanji::cached();
//! assert!(kanji.contains("亜愛安以伊位一乙王黄"));
//! assert!(!kanji.contains("ABC"));
//! ```

use crate::CodePoints;

/// JIS X 0208 Kanji character set (Level 1 + Level 2).
///
/// Contains 6 355 kanji characters as specified in JIS X 0208.
#[derive(Debug, Clone)]
pub struct JisX0208Kanji {
    codepoints: CodePoints,
}

impl JisX0208Kanji {
    /// Creates a new JIS X 0208 Kanji character set.
    pub fn new() -> Self {
        Self {
            codepoints: CodePoints::from_slice(crate::data::jisx0208kanji::JISX0208_CHARS),
        }
    }

    /// Returns a cached static reference to the JIS X 0208 Kanji set.
    ///
    /// The instance is initialized on first access; subsequent calls return
    /// the same reference with no allocation.
    pub fn cached() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<JisX0208Kanji> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Returns `true` if every character in `text` is a JIS X 0208 kanji.
    pub fn contains(&self, s: &str) -> bool {
        self.codepoints.contains(s)
    }

    /// Returns the underlying [`CodePoints`] collection.
    pub fn codepoints(&self) -> &CodePoints {
        &self.codepoints
    }

    /// Returns all kanji code points as a `Vec<u32>`.
    ///
    /// > **Note:** the order of elements is **not** guaranteed (determined by
    /// > the internal `HashSet`).  Use `.len()` on the result if you only need
    /// > the count; prefer [`Self::codepoints`] for membership checks.
    pub fn codepoints_vec(&self) -> Vec<u32> {
        self.codepoints.iter().copied().collect()
    }

    /// Validates that every character in `text` is a JIS X 0208 kanji.
    ///
    /// Returns `Ok(())` on success, or a [`crate::ValidationError`]
    /// identifying the first non-kanji character.
    pub fn validate(&self, text: &str) -> Result<(), crate::validation::ValidationError> {
        self.codepoints.validate(text)
    }
}

impl Default for JisX0208Kanji {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let kanji = JisX0208Kanji::new();
        assert_eq!(kanji.codepoints_vec().len(), 6355);
    }

    #[test]
    fn test_default_equals_new() {
        assert_eq!(
            JisX0208Kanji::default().codepoints_vec().len(),
            JisX0208Kanji::new().codepoints_vec().len()
        );
    }

    #[test]
    fn test_common_level1_kanji() {
        let kanji = JisX0208Kanji::new();
        // Spot-check well-known Level 1 kanji
        for &cp in &[
            0x4E9C, // 亜
            0x611B, // 愛
            0x5B89, // 安
            0x4EE5, // 以
            0x4F0A, // 伊
            0x4F4D, // 位
            0x4E00, // 一
            0x4E59, // 乙
            0x738B, // 王
            0x9EC4, // 黄
        ] {
            assert!(
                kanji.codepoints().iter().any(|&x| x == cp),
                "missing Level 1 kanji U+{:04X}",
                cp
            );
        }
    }

    #[test]
    fn test_level2_kanji() {
        let kanji = JisX0208Kanji::new();
        // Row 84 samples
        for &cp in &[
            0x582F, // 堯
            0x69C7, // 槇
            0x9059, // 遙
            0x7464, // 瑤
            0x51DC, // 凜
            0x7199, // 熙
        ] {
            assert!(
                kanji.codepoints().iter().any(|&x| x == cp),
                "missing Level 2 kanji U+{:04X}",
                cp
            );
        }
    }

    #[test]
    fn test_contains_strings() {
        let kanji = JisX0208Kanji::new();
        assert!(kanji.contains("亜愛安以伊位一乙王黄"));
        assert!(!kanji.contains("ABC123"));
        assert!(!kanji.contains("亜ABC愛")); // mixed → false
    }

    #[test]
    fn test_cached_identity() {
        assert!(std::ptr::eq(
            JisX0208Kanji::cached(),
            JisX0208Kanji::cached()
        ));
    }

    #[test]
    fn test_validate() {
        assert!(JisX0208Kanji::cached().validate("亜愛安").is_ok());
        let err = JisX0208Kanji::cached().validate("亜A愛").unwrap_err();
        assert_eq!(err.code_point, 0x41); // 'A'
        assert_eq!(err.position, 1);
    }
}
