//! Core code points functionality
//!
//! This module provides the main `CodePoints` struct and related functionality
//! for handling character code points.

use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

use crate::data::ascii;

/// Represents a collection of Unicode code points.
///
/// This struct provides functionality for checking if strings contain only
/// the specified code points, and for performing set operations on code point collections.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::CodePoints;
///
/// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
/// assert!(cp.contains("あ"));
/// assert!(cp.contains("い"));
/// assert!(!cp.contains("う"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodePoints {
    /// The set of allowed code points
    codepoints: HashSet<u32>,
}

impl CodePoints {
    /// Creates a new `CodePoints` instance from a vector of code points.
    ///
    /// # Arguments
    ///
    /// * `codepoints` - A vector of Unicode code points (u32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert!(cp.contains("あ"));
    /// ```
    pub fn new(codepoints: Vec<u32>) -> Self {
        Self {
            codepoints: codepoints.into_iter().collect(),
        }
    }

    /// Creates a new `CodePoints` instance from a string.
    ///
    /// This method extracts all unique code points from the given string.
    ///
    /// # Arguments
    ///
    /// * `s` - A string containing the code points
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::from_string("あい");
    /// assert!(cp.contains("あ"));
    /// assert!(cp.contains("い"));
    /// ```
    pub fn from_string(s: &str) -> Self {
        let codepoints: HashSet<u32> = s.chars().map(|c| c as u32).collect();
        Self { codepoints }
    }

    /// Checks if the given string contains only code points from this collection.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check
    ///
    /// # Returns
    ///
    /// `true` if all characters in the string are in this code point collection,
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert!(cp.contains("あ"));
    /// assert!(cp.contains("あい"));
    /// assert!(!cp.contains("あいう"));
    /// ```
    pub fn contains(&self, s: &str) -> bool {
        s.chars().all(|c| self.codepoints.contains(&(c as u32)))
    }

    /// Returns the first code point in the string that is not in this collection, along with its character index.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check
    ///
    /// # Returns
    ///
    /// `Some((code_point, char_index))` if a disallowed character is found, where `char_index` is the index of the character (not byte index) in the string.
    /// Returns `None` if all characters are allowed.
    ///
    /// # Note
    ///
    /// The returned index is the character index (as in `.chars().enumerate()`), not the byte index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert_eq!(cp.first_excluded_with_position("あい"), None);
    /// assert_eq!(cp.first_excluded_with_position("あいう"), Some((0x3046, 2))); // う at char index 2
    /// ```
    pub fn first_excluded_with_position(&self, s: &str) -> Option<(u32, usize)> {
        s.chars().enumerate().find_map(|(char_idx, c)| {
            let cp = c as u32;
            if !self.codepoints.contains(&cp) {
                Some((cp, char_idx))
            } else {
                None
            }
        })
    }

    /// Returns the first code point in the string that is not in this collection.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check
    ///
    /// # Returns
    ///
    /// `Some(code_point)` if a disallowed character is found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert_eq!(cp.first_excluded("あいう"), Some(0x3046)); // う
    /// assert_eq!(cp.first_excluded("あい"), None);
    /// ```
    pub fn first_excluded(&self, s: &str) -> Option<u32> {
        self.first_excluded_with_position(s).map(|(cp, _)| cp)
    }

    /// Returns all unique code points in the string that are not in this collection.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check
    ///
    /// # Returns
    ///
    /// A vector of unique excluded code points (no duplicates, order not guaranteed).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let excluded = cp.all_excluded("あいうえ");
    /// assert_eq!(excluded, vec![0x3046, 0x3048]); // う, え
    /// ```
    pub fn all_excluded(&self, s: &str) -> Vec<u32> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for c in s.chars() {
            let cp = c as u32;
            if !self.codepoints.contains(&cp) && seen.insert(cp) {
                result.push(cp);
            }
        }
        result
    }

    /// Returns the union of this code point collection with another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `CodePoints` instance
    ///
    /// # Returns
    ///
    /// A new `CodePoints` instance containing all code points from both collections.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let union = cp1.union(&cp2);
    /// assert!(union.contains("あいう"));
    /// ```
    pub fn union(&self, other: &CodePoints) -> CodePoints {
        let mut codepoints = self.codepoints.clone();
        codepoints.extend(&other.codepoints);
        CodePoints { codepoints }
    }

    /// Returns the intersection of this code point collection with another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `CodePoints` instance
    ///
    /// # Returns
    ///
    /// A new `CodePoints` instance containing only code points present in both collections.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let intersection = cp1.intersection(&cp2);
    /// assert!(intersection.contains("い"));
    /// assert!(!intersection.contains("あ"));
    /// assert!(!intersection.contains("う"));
    /// ```
    pub fn intersection(&self, other: &CodePoints) -> CodePoints {
        let codepoints: HashSet<u32> = self
            .codepoints
            .intersection(&other.codepoints)
            .cloned()
            .collect();
        CodePoints { codepoints }
    }

    /// Returns the difference of this code point collection with another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `CodePoints` instance
    ///
    /// # Returns
    ///
    /// A new `CodePoints` instance containing code points in this collection
    /// but not in the other.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let difference = cp1.difference(&cp2);
    /// assert!(difference.contains("あ"));
    /// assert!(!difference.contains("い"));
    /// ```
    pub fn difference(&self, other: &CodePoints) -> CodePoints {
        let codepoints: HashSet<u32> = self
            .codepoints
            .difference(&other.codepoints)
            .cloned()
            .collect();
        CodePoints { codepoints }
    }

    /// Returns the number of code points in this collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
    /// assert_eq!(cp.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.codepoints.len()
    }

    /// Returns `true` if this collection contains no code points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![]);
    /// assert!(cp.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.codepoints.is_empty()
    }

    /// Returns an iterator over the code points in this collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let mut iter = cp.iter();
    /// let first = iter.next();
    /// let second = iter.next();
    /// assert_eq!(iter.next(), None);
    /// assert!(first.is_some());
    /// assert!(second.is_some());
    /// ```
    pub fn iter(&self) -> std::collections::hash_set::Iter<u32> {
        self.codepoints.iter()
    }

    // ASCII character set factory methods

    /// Creates a new CodePoints instance with ASCII control characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_control();
    /// assert!(cp.contains("\n"));
    /// assert!(cp.contains("\r"));
    /// assert!(!cp.contains("a"));
    /// ```
    pub fn ascii_control() -> Self {
        Self::new(ascii::CONTROL_CHARS.to_vec())
    }

    /// Returns a cached instance of ASCII control characters CodePoints.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::ascii_control_cached();
    /// let cp2 = CodePoints::ascii_control_cached();
    /// // Both instances share the same underlying data
    /// ```
    pub fn ascii_control_cached() -> &'static CodePoints {
        static ASCII_CONTROL: OnceLock<CodePoints> = OnceLock::new();
        ASCII_CONTROL.get_or_init(|| Self::ascii_control())
    }

    /// Creates a new CodePoints instance with ASCII printable characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_printable();
    /// assert!(cp.contains("Hello"));
    /// assert!(cp.contains("123"));
    /// assert!(!cp.contains("あ"));
    /// ```
    pub fn ascii_printable() -> Self {
        Self::new(ascii::PRINTABLE_CHARS.to_vec())
    }

    /// Returns a cached instance of ASCII printable characters CodePoints.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::ascii_printable_cached();
    /// let cp2 = CodePoints::ascii_printable_cached();
    /// // Both instances share the same underlying data
    /// ```
    pub fn ascii_printable_cached() -> &'static CodePoints {
        static ASCII_PRINTABLE: OnceLock<CodePoints> = OnceLock::new();
        ASCII_PRINTABLE.get_or_init(|| Self::ascii_printable())
    }

    /// Creates a new CodePoints instance with CRLF characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::crlf();
    /// assert!(cp.contains("\n"));
    /// assert!(cp.contains("\r"));
    /// assert!(!cp.contains("a"));
    /// ```
    pub fn crlf() -> Self {
        Self::new(ascii::CRLF_CHARS.to_vec())
    }

    /// Returns a cached instance of CRLF characters CodePoints.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::crlf_cached();
    /// let cp2 = CodePoints::crlf_cached();
    /// // Both instances share the same underlying data
    /// ```
    pub fn crlf_cached() -> &'static CodePoints {
        static CRLF: OnceLock<CodePoints> = OnceLock::new();
        CRLF.get_or_init(|| Self::crlf())
    }

    /// Creates a new CodePoints instance with all ASCII characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_all();
    /// assert!(cp.contains("Hello"));
    /// assert!(cp.contains("\n"));
    /// assert!(!cp.contains("あ"));
    /// ```
    pub fn ascii_all() -> Self {
        Self::new(ascii::ALL_ASCII.to_vec())
    }

    /// Returns a cached instance of all ASCII characters CodePoints.
    ///
    /// This method uses static caching to avoid repeated allocation.
    /// Subsequent calls return the same cached instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp1 = CodePoints::ascii_all_cached();
    /// let cp2 = CodePoints::ascii_all_cached();
    /// // Both instances share the same underlying data
    /// ```
    pub fn ascii_all_cached() -> &'static CodePoints {
        static ASCII_ALL: OnceLock<CodePoints> = OnceLock::new();
        ASCII_ALL.get_or_init(|| Self::ascii_all())
    }

    /// Returns `true` if this collection is a subset of another `CodePoints` collection.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `CodePoints` instance
    ///
    /// # Returns
    ///
    /// `true` if all code points in this collection are also in `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let cp2 = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
    /// assert!(cp1.is_subset_of(&cp2));
    /// ```
    pub fn is_subset_of(&self, other: &CodePoints) -> bool {
        self.codepoints.is_subset(&other.codepoints)
    }

    /// Returns `true` if this collection is a superset of another `CodePoints` collection.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `CodePoints` instance
    ///
    /// # Returns
    ///
    /// `true` if all code points in `other` are also in this collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp1 = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
    /// let cp2 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert!(cp1.is_superset_of(&cp2));
    /// ```
    pub fn is_superset_of(&self, other: &CodePoints) -> bool {
        self.codepoints.is_superset(&other.codepoints)
    }

    /// Returns the symmetric difference of this code point collection with another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another `CodePoints` instance
    ///
    /// # Returns
    ///
    /// A new `CodePoints` instance containing code points that are in either collection but not in both.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    /// let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let diff = cp1.symmetric_difference(&cp2);
    /// assert!(diff.contains("あ"));
    /// assert!(diff.contains("う"));
    /// assert!(!diff.contains("い"));
    /// ```
    pub fn symmetric_difference(&self, other: &CodePoints) -> CodePoints {
        let diff = self
            .codepoints
            .symmetric_difference(&other.codepoints)
            .cloned()
            .collect();
        CodePoints::new(diff)
    }

    /// Checks if the given string contains only code points that are valid in ANY of the provided code point collections.
    ///
    /// This is equivalent to the Java method `containsAllInAnyCodePoints`.
    /// Returns `true` if all characters in the string are included in at least one of the code point collections.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check
    /// * `codepoints_list` - A slice of `CodePoints` instances to check against
    ///
    /// # Returns
    ///
    /// `true` if all code points in the given string are included in any of the code points list,
    /// `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let hiragana = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let katakana = CodePoints::new(vec![0x30A2, 0x30A4]); // ア, イ
    /// let mixed_text = "あア"; // Contains both hiragana and katakana
    ///
    /// // Each character is valid in at least one collection
    /// assert!(CodePoints::contains_all_in_any("あア", &[hiragana, katakana]));
    /// ```
    pub fn contains_all_in_any(s: &str, codepoints_list: &[CodePoints]) -> bool {
        use std::collections::HashMap;

        if codepoints_list.is_empty() {
            return false;
        }

        let mut excluded_counts: HashMap<u32, usize> = HashMap::new();

        for codepoints in codepoints_list {
            let excluded = codepoints.all_excluded(s);
            if excluded.is_empty() {
                // If any CodePoints collection accepts all characters, return true immediately
                return true;
            }

            for codepoint in excluded {
                // Count how many CodePoints collections exclude each character
                *excluded_counts.entry(codepoint).or_insert(0) += 1;
            }
        }

        // Check if any character is excluded by all collections
        for (_, count) in excluded_counts {
            if count == codepoints_list.len() {
                // This character is excluded by all collections
                return false;
            }
        }

        // All characters are accepted by at least one collection
        true
    }
}

impl fmt::Display for CodePoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodePoints({} items)", self.codepoints.len())
    }
}

impl From<Vec<u32>> for CodePoints {
    fn from(codepoints: Vec<u32>) -> Self {
        Self::new(codepoints)
    }
}

impl From<&str> for CodePoints {
    fn from(s: &str) -> Self {
        Self::from_string(s)
    }
}

impl std::hash::Hash for CodePoints {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Sort the code points to ensure consistent hashing
        let mut sorted_codepoints: Vec<&u32> = self.codepoints.iter().collect();
        sorted_codepoints.sort();
        sorted_codepoints.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cp = CodePoints::new(vec![0x3041, 0x3042]); // あ, い
        assert_eq!(cp.len(), 2);
    }

    #[test]
    fn test_from_string() {
        let cp = CodePoints::from_string("あい");
        assert_eq!(cp.len(), 2);
        assert!(cp.contains("あ"));
        assert!(cp.contains("い"));
    }

    #[test]
    fn test_contains() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert!(cp.contains("あ"));
        assert!(cp.contains("い"));
        assert!(cp.contains("あい"));
        assert!(!cp.contains("う"));
        assert!(!cp.contains("あいう"));
    }

    #[test]
    fn test_contains_null_and_empty() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い

        // Test empty string (should be valid)
        assert!(cp.contains(""));

        // Test with space character (not in our set, should be invalid)
        assert!(!cp.contains(" ")); // Space character not in set
    }

    #[test]
    fn test_contains_surrogate_pairs() {
        // Test with surrogate pair characters (like emoji)
        let surrogate_char = "𠀋"; // U+2000B, a surrogate pair
        let cp = CodePoints::new(vec![0x2000B, 0x3042, 0x3044]); // surrogate + あ, い

        assert!(cp.contains(surrogate_char));
        assert!(cp.contains(&format!("{}あい", surrogate_char)));
        assert!(!cp.contains(&format!("{}あいか", surrogate_char))); // か not in set
    }

    #[test]
    fn test_contains_mixed_characters() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046, 0x3048, 0x304A, 0x2000B]); // あ,い,う,え,お + surrogate

        let test_str = format!("{}あいうあ", "𠀋"); // surrogate + あいうあ
        assert!(cp.contains(&test_str));

        let invalid_str = format!("{}あいうか", "𠀋"); // surrogate + あいうか (か not in set)
        assert!(!cp.contains(&invalid_str));
    }

    #[test]
    fn test_first_excluded() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert_eq!(cp.first_excluded("あい"), None);
        assert_eq!(cp.first_excluded("あいう"), Some(0x3046)); // う
    }

    #[test]
    fn test_first_excluded_with_surrogate_pairs() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x2000B]); // あ, い, surrogate

        let test_str = format!("{}あい", "𠀋");
        assert_eq!(cp.first_excluded(&test_str), None);

        let invalid_str = format!("{}あいう", "𠀋");
        assert_eq!(cp.first_excluded(&invalid_str), Some(0x3046)); // う
    }

    #[test]
    fn test_all_excluded() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let excluded = cp.all_excluded("あいうえ");
        assert_eq!(excluded, vec![0x3046, 0x3048]); // う, え
    }

    #[test]
    fn test_all_excluded_with_surrogate_pairs() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x2000B]); // あ, い, surrogate

        let test_str = format!("{}あいう", "𠀋");
        let excluded = cp.all_excluded(&test_str);
        assert_eq!(excluded, vec![0x3046]); // う

        // Test with multiple invalid characters including surrogate pairs
        let test_str2 = format!("{}あいうきかくか{}", "𠀋", "𠂟"); // き,か,く not in set, 2nd surrogate not in set
        let excluded2 = cp.all_excluded(&test_str2);
        // all_excluded guarantees order, so no need to sort
        assert_eq!(excluded2, vec![0x3046, 0x304D, 0x304B, 0x304F, 0x2009F]); // う,き,か,く,2nd surrogate
    }

    #[test]
    fn test_union() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
        let union = cp1.union(&cp2);
        assert_eq!(union.len(), 3);
        assert!(union.contains("あいう"));
    }

    #[test]
    fn test_intersection() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
        let intersection = cp1.intersection(&cp2);
        assert_eq!(intersection.len(), 1);
        assert!(intersection.contains("い"));
        assert!(!intersection.contains("あ"));
        assert!(!intersection.contains("う"));
    }

    #[test]
    fn test_difference() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
        let difference = cp1.difference(&cp2);
        assert_eq!(difference.len(), 1);
        assert!(difference.contains("あ"));
        assert!(!difference.contains("い"));
    }

    #[test]
    fn test_ascii_control() {
        let cp = CodePoints::ascii_control();
        assert!(cp.contains("\n"));
        assert!(cp.contains("\r"));
        assert!(cp.contains("\t"));
        assert!(!cp.contains("a"));
        assert!(!cp.contains("あ"));
    }

    #[test]
    fn test_ascii_printable() {
        let cp = CodePoints::ascii_printable();
        assert!(cp.contains("Hello"));
        assert!(cp.contains("123"));
        assert!(cp.contains("!@#$%"));
        assert!(!cp.contains("\n"));
        assert!(!cp.contains("あ"));

        // Test specific characters from Java tests
        assert!(cp.contains("Hello~"));
        assert!(cp.contains("\\100"));
        assert!(!cp.contains("Hello‾")); // Overline character
        assert!(!cp.contains("¥100")); // Yen symbol
    }

    #[test]
    fn test_crlf() {
        let cp = CodePoints::crlf();
        assert!(cp.contains("\n"));
        assert!(cp.contains("\r"));
        assert!(!cp.contains("a"));
        assert!(!cp.contains("\t"));
    }

    #[test]
    fn test_ascii_all() {
        let cp = CodePoints::ascii_all();
        assert!(cp.contains("Hello"));
        assert!(cp.contains("\n"));
        assert!(cp.contains("\r"));
        assert!(cp.contains("123"));
        assert!(!cp.contains("あ"));
    }

    #[test]
    fn test_first_excluded_with_position() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert_eq!(cp.first_excluded_with_position("あい"), None);
        // う at position 2
        assert_eq!(cp.first_excluded_with_position("あいう"), Some((0x3046, 2)));
    }

    #[test]
    fn test_is_subset_of() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        assert!(cp1.is_subset_of(&cp2));
        assert!(!cp2.is_subset_of(&cp1));
    }

    #[test]
    fn test_symmetric_difference() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
        let diff = cp1.symmetric_difference(&cp2);
        assert_eq!(diff.len(), 2);
        assert!(diff.contains("あ"));
        assert!(diff.contains("う"));
        assert!(!diff.contains("い"));
    }

    #[test]
    fn test_equals_and_hashcode() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp3 = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う

        assert_eq!(cp1, cp2);
        assert_ne!(cp1, cp3);

        // Hash codes should be equal for equal objects
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        cp1.hash(&mut hasher1);
        cp2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_from_string_with_duplicates() {
        let cp = CodePoints::from_string("あいあい"); // Duplicate characters
        assert_eq!(cp.len(), 2); // Should deduplicate
        assert!(cp.contains("あ"));
        assert!(cp.contains("い"));
    }

    #[test]
    fn test_empty_codepoints() {
        let cp = CodePoints::new(vec![]);
        assert!(cp.is_empty());
        assert_eq!(cp.len(), 0);
        assert!(cp.contains("")); // Empty string should be valid
        assert!(!cp.contains("a")); // Any non-empty string should be invalid
    }

    #[test]
    fn test_intersection_with_empty_sets() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![]); // Empty set

        let intersection = cp1.intersection(&cp2);
        assert!(intersection.is_empty());

        let intersection2 = cp2.intersection(&cp1);
        assert!(intersection2.is_empty());
    }

    #[test]
    fn test_union_with_empty_sets() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![]); // Empty set

        let union = cp1.union(&cp2);
        assert_eq!(union.len(), 2);
        assert!(union.contains("あい"));

        let union2 = cp2.union(&cp1);
        assert_eq!(union2.len(), 2);
        assert!(union2.contains("あい"));
    }

    #[test]
    fn test_difference_with_empty_sets() {
        let cp1 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        let cp2 = CodePoints::new(vec![]); // Empty set

        let difference = cp1.difference(&cp2);
        assert_eq!(difference.len(), 2);
        assert!(difference.contains("あい"));

        let difference2 = cp2.difference(&cp1);
        assert!(difference2.is_empty());
    }

    #[test]
    fn test_contains_surrogate_pairs_not_allowed() {
        // Test that surrogate pairs are not allowed when not in the set
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        let surrogate_char = "𠀋"; // U+2000B

        let test_str = format!("{}あいうあ{}", surrogate_char, surrogate_char);
        assert!(!cp.contains(&test_str));
    }

    #[test]
    fn test_first_excluded_with_surrogate_pairs_not_allowed() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        let surrogate_char = "𠀋"; // U+2000B

        let test_str = format!("{}あいうかき", surrogate_char);
        assert_eq!(cp.first_excluded(&test_str), Some(0x2000B)); // First excluded is surrogate
    }

    #[test]
    fn test_all_excluded_with_multiple_surrogate_pairs() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        let surrogate1 = "𠀋"; // U+2000B
        let surrogate2 = "𠂟"; // U+2009F

        let test_str = format!("{}あいうきかくか{}", surrogate1, surrogate2);
        let excluded = cp.all_excluded(&test_str);
        assert_eq!(excluded, vec![0x2000B, 0x304D, 0x304B, 0x304F, 0x2009F]); // surrogate1, き, か, く, surrogate2
    }

    #[test]
    fn test_first_excluded_null_and_empty() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い

        // Test empty string (should return None)
        assert_eq!(cp.first_excluded(""), None);
    }

    #[test]
    fn test_all_excluded_null_and_empty() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い

        // Test empty string (should return empty vector)
        assert_eq!(cp.all_excluded(""), vec![] as Vec<u32>);
    }

    #[test]
    fn test_contains_all_in_any() {
        let hiragana = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        let katakana = CodePoints::new(vec![0x30A2, 0x30A4, 0x30A6]); // ア, イ, ウ
        let ascii = CodePoints::ascii_printable();

        // Test with empty list
        assert!(!CodePoints::contains_all_in_any("test", &[]));

        // Test where one collection accepts all characters
        assert!(CodePoints::contains_all_in_any("あい", &[hiragana.clone()]));
        assert!(CodePoints::contains_all_in_any("アイ", &[katakana.clone()]));

        // Test mixed characters that are valid in different collections
        let mixed_collections = [hiragana.clone(), katakana.clone()];
        assert!(CodePoints::contains_all_in_any("あア", &mixed_collections)); // あ in hiragana, ア in katakana
        assert!(CodePoints::contains_all_in_any("いイ", &mixed_collections)); // い in hiragana, イ in katakana

        // Test with characters not in any collection
        assert!(!CodePoints::contains_all_in_any("xyz", &mixed_collections)); // Latin chars not in either

        // Test with some valid, some invalid characters
        assert!(!CodePoints::contains_all_in_any("あアx", &mixed_collections)); // x not in either collection

        // Test with three collections
        let three_collections = [hiragana, katakana, ascii];
        assert!(CodePoints::contains_all_in_any("あアA", &three_collections)); // Each char in different collection
        assert!(CodePoints::contains_all_in_any("Hello", &three_collections)); // All in ASCII
        assert!(!CodePoints::contains_all_in_any("あアAπ", &three_collections)); // π not in any collection

        // Test empty string (should be valid for any non-empty collection list)
        assert!(CodePoints::contains_all_in_any("", &three_collections));
    }

    #[test]
    fn test_contains_all_in_any_edge_cases() {
        let cp1 = CodePoints::new(vec![0x3042]); // あ
        let cp2 = CodePoints::new(vec![0x3044]); // い

        // Character that appears in multiple collections
        let cp3 = CodePoints::new(vec![0x3042, 0x3046]); // あ, う
        let collections = [cp1, cp2, cp3];

        assert!(CodePoints::contains_all_in_any("あ", &collections)); // あ in cp1 and cp3
        assert!(CodePoints::contains_all_in_any("い", &collections)); // い in cp2
        assert!(CodePoints::contains_all_in_any("う", &collections)); // う in cp3
        assert!(!CodePoints::contains_all_in_any("え", &collections)); // え not in any
    }

    #[test]
    fn test_ascii_cached_methods() {
        // Test that cached methods return the same instance
        let control1 = CodePoints::ascii_control_cached();
        let control2 = CodePoints::ascii_control_cached();
        assert!(std::ptr::eq(control1, control2));

        let printable1 = CodePoints::ascii_printable_cached();
        let printable2 = CodePoints::ascii_printable_cached();
        assert!(std::ptr::eq(printable1, printable2));

        let crlf1 = CodePoints::crlf_cached();
        let crlf2 = CodePoints::crlf_cached();
        assert!(std::ptr::eq(crlf1, crlf2));

        let all1 = CodePoints::ascii_all_cached();
        let all2 = CodePoints::ascii_all_cached();
        assert!(std::ptr::eq(all1, all2));

        // Test functionality is the same as non-cached versions
        assert_eq!(control1, &CodePoints::ascii_control());
        assert_eq!(printable1, &CodePoints::ascii_printable());
        assert_eq!(crlf1, &CodePoints::crlf());
        assert_eq!(all1, &CodePoints::ascii_all());
    }
}
