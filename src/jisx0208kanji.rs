//! JIS X 0208 Kanji character set support
//!
//! This module provides kanji characters defined in JIS X 0208 standard.
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "codepoints-jisx0208kanji")]
//! use japanese_codepoints::jisx0208kanji::JisX0208Kanji;
//!
//! # #[cfg(feature = "codepoints-jisx0208kanji")]
//! let kanji = JisX0208Kanji::new();
//! # #[cfg(feature = "codepoints-jisx0208kanji")]
//! assert!(kanji.contains("亜愛安以伊位一乙王黄"));
//! ```
//!
//! # Level 1 and Level 2 Kanji
//!
//! JIS X 0208 defines two levels of kanji:
//!
//! - **Level 1**: 2,965 characters (rows 16-47)
//! - **Level 2**: 3,390 characters (rows 48-84)
//!
//! ```rust
//! # #[cfg(feature = "codepoints-jisx0208kanji")]
//! use japanese_codepoints::jisx0208kanji::JisX0208Kanji;
//!
//! # #[cfg(feature = "codepoints-jisx0208kanji")]
//! let kanji = JisX0208Kanji::new();
//! # #[cfg(feature = "codepoints-jisx0208kanji")]
//! let codepoints = kanji.codepoints_vec();
//! ```

use crate::CodePoints;

/// JIS X 0208 Kanji character set
///
/// Contains Level 1 kanji (rows 16-47) and Level 2 kanji (rows 48-84) from JIS X 0208 standard
/// Total of 6,355 kanji characters
#[derive(Debug, Clone)]
pub struct JisX0208Kanji {
    pub all: CodePoints,
}

impl JisX0208Kanji {
    /// Create a new JIS X 0208 kanji character set instance
    pub fn new() -> Self {
        Self {
            all: CodePoints::new(crate::data::jisx0208kanji::JISX0208_CHARS.to_vec()),
        }
    }

    /// Get all kanji codepoints as `Vec<u32>`
    pub fn codepoints_vec(&self) -> Vec<u32> {
        self.all.iter().copied().collect()
    }

    /// Check if a string consists entirely of JIS X 0208 kanji characters
    pub fn contains(&self, s: &str) -> bool {
        self.all.contains(s)
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
    fn test_jisx0208kanji_new() {
        let kanji = JisX0208Kanji::new();
        assert_eq!(kanji.codepoints_vec().len(), 6355);
    }

    #[test]
    fn test_jisx0208kanji_default() {
        let kanji = JisX0208Kanji::default();
        assert_eq!(kanji.codepoints_vec().len(), 6355);
    }

    #[test]
    fn test_jisx0208kanji_contains_common_kanji() {
        let kanji = JisX0208Kanji::new();
        let codepoints = kanji.codepoints_vec();

        // Test some common kanji
        assert!(codepoints.contains(&0x4E9C)); // 亜
        assert!(codepoints.contains(&0x611B)); // 愛
        assert!(codepoints.contains(&0x5B89)); // 安
        assert!(codepoints.contains(&0x4EE5)); // 以
        assert!(codepoints.contains(&0x4F0A)); // 伊
        assert!(codepoints.contains(&0x4F4D)); // 位
        assert!(codepoints.contains(&0x4E00)); // 一
        assert!(codepoints.contains(&0x4E59)); // 乙
        assert!(codepoints.contains(&0x738B)); // 王
        assert!(codepoints.contains(&0x9EC4)); // 黄
    }

    #[test]
    fn test_jisx0208kanji_contains_level2_kanji() {
        let kanji = JisX0208Kanji::new();
        let codepoints = kanji.codepoints_vec();

        // Test some Level 2 kanji
        assert!(codepoints.contains(&0x582F)); // 堯 (84-01)
        assert!(codepoints.contains(&0x69C7)); // 槇 (84-02)
        assert!(codepoints.contains(&0x9059)); // 遙 (84-03)
        assert!(codepoints.contains(&0x7464)); // 瑤 (84-04)
        assert!(codepoints.contains(&0x51DC)); // 凜 (84-05)
        assert!(codepoints.contains(&0x7199)); // 熙 (84-06)
    }

    #[test]
    fn test_jisx0208kanji_contains() {
        let kanji = JisX0208Kanji::new();

        // Test containing kanji
        assert!(kanji.contains("亜愛安以伊位一乙王黄"));

        // Test not containing non-kanji
        assert!(!kanji.contains("ABC123"));

        // Test mixed content
        assert!(!kanji.contains("亜ABC愛"));
    }
}
