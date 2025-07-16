//! JIS X 0213 Kanji character set support
//!
//! This module provides kanji characters defined in JIS X 0213 standard.
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! use japanese_codepoints::jisx0213kanji::JisX0213Kanji;
//!
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! let kanji = JisX0213Kanji::new();
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! assert!(kanji.contains("亜愛安以伊位一乙王黄"));
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! assert!(kanji.contains("堯槇遙瑤凜熙")); // Level 3-4 kanji unique to JIS X 0213
//! ```
//!
//! # Level 1, 2, 3, and 4 Kanji
//!
//! JIS X 0213 defines four levels of kanji:
//!
//! - **Level 1**: 2,965 characters (same as JIS X 0208 Level 1)
//! - **Level 2**: 3,390 characters (same as JIS X 0208 Level 2)
//! - **Level 3**: 1,259 characters (new in JIS X 0213)
//! - **Level 4**: 2,436 characters (new in JIS X 0213)
//!
//! ```rust
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! use japanese_codepoints::jisx0213kanji::JisX0213Kanji;
//!
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! let kanji = JisX0213Kanji::new();
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! let codepoints = kanji.codepoints_vec();
//! # #[cfg(feature = "codepoints-jisx0213kanji")]
//! assert_eq!(codepoints.len(), 10050); // Total kanji count in JIS X 0213
//! ```

use crate::CodePoints;

/// JIS X 0213 Kanji character set
///
/// Contains Level 1, 2, 3, and 4 kanji from JIS X 0213:2004 standard
/// Total of 10,050 kanji characters
#[derive(Debug, Clone)]
pub struct JisX0213Kanji {
    all: CodePoints,
}

impl JisX0213Kanji {
    /// Create a new JIS X 0213 kanji character set instance
    pub fn new() -> Self {
        Self {
            all: CodePoints::new(crate::data::jisx0213kanji::JISX0213_KANJI.to_vec()),
        }
    }

    /// Get all kanji codepoints as `Vec<u32>`
    pub fn codepoints_vec(&self) -> Vec<u32> {
        self.all.iter().copied().collect()
    }

    /// Check if a string consists entirely of JIS X 0213 kanji characters
    pub fn contains(&self, s: &str) -> bool {
        self.all.contains(s)
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
    fn test_jisx0213kanji_new() {
        let kanji = JisX0213Kanji::new();
        assert_eq!(kanji.codepoints_vec().len(), 10050);
    }

    #[test]
    fn test_jisx0213kanji_default() {
        let kanji = JisX0213Kanji::default();
        assert_eq!(kanji.codepoints_vec().len(), 10050);
    }

    #[test]
    fn test_jisx0213kanji_contains() {
        let kanji = JisX0213Kanji::new();

        // Test some common kanji
        assert!(kanji.contains("亜"));
        assert!(kanji.contains("愛"));
        assert!(kanji.contains("安"));
        assert!(kanji.contains("以"));
        assert!(kanji.contains("伊"));
        assert!(kanji.contains("位"));
        assert!(kanji.contains("一"));
        assert!(kanji.contains("乙"));
        assert!(kanji.contains("王"));
        assert!(kanji.contains("黄"));

        // Test some Level 3 and 4 kanji
        assert!(kanji.contains("堯"));
        assert!(kanji.contains("槇"));
        assert!(kanji.contains("遙"));
        assert!(kanji.contains("瑤"));
        assert!(kanji.contains("凜"));
        assert!(kanji.contains("熙"));

        // Test mixed strings with Level 1-2 kanji
        assert!(kanji.contains("亜愛安以伊位一乙王黄"));

        // Test Level 3-4 kanji unique to JIS X 0213
        assert!(kanji.contains("堯槇遙瑤凜熙"));
        assert!(kanji.contains("龕龠龢")); // Only test characters that exist in JIS X 0213

        // Test strings containing non-kanji
        assert!(!kanji.contains("亜A愛"));
        assert!(!kanji.contains("123"));
        assert!(kanji.contains("")); // Empty string contains no invalid characters
    }
}
