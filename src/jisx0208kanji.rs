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

define_codepoint_set! {
    /// JIS X 0208 Kanji character set (Level 1 + Level 2).
    ///
    /// Contains 6 355 kanji characters as specified in JIS X 0208.
    pub struct JisX0208Kanji => crate::data::jisx0208kanji::JISX0208_CHARS;
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
