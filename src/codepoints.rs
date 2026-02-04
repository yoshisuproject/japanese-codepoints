//! Core code-point collection type and multi-set membership helper.
//!
//! [`CodePoints`] is the central data structure: an immutable set of Unicode
//! scalar values that can efficiently test membership for individual
//! characters or entire strings.
//!
//! The free function [`contains_all_in_any`] extends membership testing to
//! multiple sets at once — useful when a string may legally contain characters
//! from several scripts simultaneously.

use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

use crate::data::ascii;

// ── main type ─────────────────────────────────────────────────────────────────

/// An immutable collection of Unicode code points.
///
/// The primary use-case is character-set validation: given a policy (e.g.
/// "only JIS X 0208 hiragana"), quickly determine whether a string conforms.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::CodePoints;
///
/// let allowed = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
/// assert!(allowed.contains("あい"));
/// assert!(!allowed.contains("う"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodePoints {
    codepoints: HashSet<u32>,
}

// ── constructors ──────────────────────────────────────────────────────────────

impl CodePoints {
    /// Creates a `CodePoints` from a `Vec` of code-point values.
    ///
    /// Duplicate values are silently de-duplicated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3042, 0x3044]);
    /// assert_eq!(cp.len(), 2);
    /// ```
    pub fn new(codepoints: Vec<u32>) -> Self {
        Self {
            codepoints: codepoints.into_iter().collect(),
        }
    }

    /// Creates a `CodePoints` from a slice of code-point values.
    ///
    /// This is the preferred constructor when the source data is a static or
    /// borrowed `&[u32]` because it avoids an intermediate `Vec` allocation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// const HIRAGANA_AI: &[u32] = &[0x3042, 0x3044];
    /// let cp = CodePoints::from_slice(HIRAGANA_AI);
    /// assert!(cp.contains("あい"));
    /// ```
    pub fn from_slice(slice: &[u32]) -> Self {
        Self {
            codepoints: slice.iter().copied().collect(),
        }
    }

    /// Creates a `CodePoints` by extracting every unique code point from a
    /// string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::from_string("あいあ");
    /// assert_eq!(cp.len(), 2); // あ deduplicated
    /// ```
    pub fn from_string(s: &str) -> Self {
        Self {
            codepoints: s.chars().map(|c| c as u32).collect(),
        }
    }
}

// ── membership ────────────────────────────────────────────────────────────────

impl CodePoints {
    /// Returns `true` if **every** character in `text` belongs to this set.
    ///
    /// An empty string is always considered valid (vacuously true).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert!(cp.contains("あい"));
    /// assert!(!cp.contains("う"));
    /// assert!(cp.contains(""));   // empty string
    /// ```
    pub fn contains(&self, s: &str) -> bool {
        s.chars().all(|c| self.codepoints.contains(&(c as u32)))
    }

    /// Returns `true` if the single character `c` belongs to this set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042]); // あ
    /// assert!(cp.contains_char('あ'));
    /// assert!(!cp.contains_char('い'));
    /// ```
    pub fn contains_char(&self, c: char) -> bool {
        self.codepoints.contains(&(c as u32))
    }

    /// Returns the first code point in `text` that is **not** in this set,
    /// together with its zero-based character index (not byte index).
    ///
    /// Returns `None` when every character is allowed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert_eq!(cp.first_excluded_with_position("あいう"), Some((0x3046, 2)));
    /// assert_eq!(cp.first_excluded_with_position("あい"),   None);
    /// ```
    pub fn first_excluded_with_position(&self, s: &str) -> Option<(u32, usize)> {
        s.chars().enumerate().find_map(|(i, c)| {
            let cp = c as u32;
            if self.codepoints.contains(&cp) {
                None
            } else {
                Some((cp, i))
            }
        })
    }

    /// Returns the first code point in `text` that is **not** in this set.
    ///
    /// This is a convenience wrapper around [`Self::first_excluded_with_position`]
    /// that discards the position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// assert_eq!(cp.first_excluded("あいう"), Some(0x3046)); // う
    /// assert_eq!(cp.first_excluded("あい"),   None);
    /// ```
    pub fn first_excluded(&self, s: &str) -> Option<u32> {
        self.first_excluded_with_position(s).map(|(cp, _)| cp)
    }

    /// Returns all unique code points in `text` that are **not** in this set.
    ///
    /// The returned vector preserves **first-occurrence order**: the first
    /// excluded character encountered while scanning `text` left-to-right
    /// appears first.  Each excluded code point appears exactly once even if
    /// it occurs multiple times in the input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// // う then え, first-occurrence order
    /// assert_eq!(cp.all_excluded("あいうえ"), vec![0x3046, 0x3048]);
    /// ```
    pub fn all_excluded(&self, s: &str) -> Vec<u32> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for c in s.chars() {
            let cp = c as u32;
            if !self.codepoints.contains(&cp) && seen.insert(cp) {
                result.push(cp);
            }
        }
        result
    }
}

// ── validation ────────────────────────────────────────────────────────────────

impl CodePoints {
    /// Validates that every character in `text` belongs to this set.
    ///
    /// Returns `Ok(())` if all characters are valid.  On failure, returns an
    /// error that identifies the first offending character and its position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_printable();
    /// assert!(cp.validate("hello").is_ok());
    ///
    /// let err = cp.validate("hello\0world").unwrap_err();
    /// assert_eq!(err.code_point, 0);  // NULL
    /// assert_eq!(err.position, 5);
    /// ```
    pub fn validate(&self, text: &str) -> Result<(), crate::validation::ValidationError> {
        match self.first_excluded_with_position(text) {
            None => Ok(()),
            Some((cp, pos)) => Err(crate::validation::ValidationError::new(cp, pos)),
        }
    }
}

// ── set operations ────────────────────────────────────────────────────────────

impl CodePoints {
    /// Returns a new set that is the **union** of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let a = CodePoints::new(vec![0x3042]);          // あ
    /// let b = CodePoints::new(vec![0x3044]);          // い
    /// assert!(a.union(&b).contains("あい"));
    /// ```
    pub fn union(&self, other: &CodePoints) -> CodePoints {
        let mut codepoints = self.codepoints.clone();
        codepoints.extend(&other.codepoints);
        CodePoints { codepoints }
    }

    /// Returns a new set containing only the code points present in **both**
    /// `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let a = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let b = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let i = a.intersection(&b);
    /// assert!(i.contains("い"));
    /// assert!(!i.contains("あ"));
    /// ```
    pub fn intersection(&self, other: &CodePoints) -> CodePoints {
        CodePoints {
            codepoints: self
                .codepoints
                .intersection(&other.codepoints)
                .copied()
                .collect(),
        }
    }

    /// Returns a new set containing code points in `self` but **not** in
    /// `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let a = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let b = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let d = a.difference(&b);
    /// assert!(d.contains("あ"));
    /// assert!(!d.contains("い"));
    /// ```
    pub fn difference(&self, other: &CodePoints) -> CodePoints {
        CodePoints {
            codepoints: self
                .codepoints
                .difference(&other.codepoints)
                .copied()
                .collect(),
        }
    }

    /// Returns a new set containing code points that are in **either** `self`
    /// or `other`, but not in both (symmetric difference / XOR).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let a = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    /// let b = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    /// let s = a.symmetric_difference(&b);
    /// assert!(s.contains("あ"));
    /// assert!(s.contains("う"));
    /// assert!(!s.contains("い"));
    /// ```
    pub fn symmetric_difference(&self, other: &CodePoints) -> CodePoints {
        CodePoints {
            codepoints: self
                .codepoints
                .symmetric_difference(&other.codepoints)
                .copied()
                .collect(),
        }
    }

    /// Returns `true` if every code point in `self` is also in `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let small = CodePoints::new(vec![0x3042]);                // あ
    /// let big   = CodePoints::new(vec![0x3042, 0x3044]);        // あ, い
    /// assert!(small.is_subset_of(&big));
    /// assert!(!big.is_subset_of(&small));
    /// ```
    pub fn is_subset_of(&self, other: &CodePoints) -> bool {
        self.codepoints.is_subset(&other.codepoints)
    }

    /// Returns `true` if every code point in `other` is also in `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let big   = CodePoints::new(vec![0x3042, 0x3044]);        // あ, い
    /// let small = CodePoints::new(vec![0x3042]);                // あ
    /// assert!(big.is_superset_of(&small));
    /// ```
    pub fn is_superset_of(&self, other: &CodePoints) -> bool {
        self.codepoints.is_superset(&other.codepoints)
    }
}

// ── size / iteration ──────────────────────────────────────────────────────────

impl CodePoints {
    /// Returns the number of code points in this set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]);
    /// assert_eq!(cp.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.codepoints.len()
    }

    /// Returns `true` if the set contains no code points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// assert!(CodePoints::new(vec![]).is_empty());
    /// assert!(!CodePoints::new(vec![0x41]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.codepoints.is_empty()
    }

    /// Returns an iterator over the code points in this set.
    ///
    /// > **Note:** iteration order is **not** guaranteed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::new(vec![0x3042, 0x3044]);
    /// assert_eq!(cp.iter().count(), 2);
    /// ```
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, u32> {
        self.codepoints.iter()
    }
}

// ── ASCII factory methods ─────────────────────────────────────────────────────

impl CodePoints {
    /// Creates a new set containing all ASCII **control** characters
    /// (U+0000–U+001F and U+007F).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_control();
    /// assert!(cp.contains("\n\r\t"));
    /// assert!(!cp.contains("a"));
    /// ```
    pub fn ascii_control() -> Self {
        Self::from_slice(ascii::CONTROL_CHARS)
    }

    /// Returns a cached static reference to the ASCII control character set.
    ///
    /// Equivalent to [`Self::ascii_control`] but allocated only once via
    /// [`OnceLock`].
    pub fn ascii_control_cached() -> &'static CodePoints {
        static INSTANCE: OnceLock<CodePoints> = OnceLock::new();
        INSTANCE.get_or_init(Self::ascii_control)
    }

    /// Creates a new set containing all ASCII **printable** characters
    /// (U+0020–U+007E).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_printable();
    /// assert!(cp.contains("Hello 123!"));
    /// assert!(!cp.contains("あ"));
    /// ```
    pub fn ascii_printable() -> Self {
        Self::from_slice(ascii::PRINTABLE_CHARS)
    }

    /// Returns a cached static reference to the ASCII printable character set.
    pub fn ascii_printable_cached() -> &'static CodePoints {
        static INSTANCE: OnceLock<CodePoints> = OnceLock::new();
        INSTANCE.get_or_init(Self::ascii_printable)
    }

    /// Creates a new set containing only CR (U+000D) and LF (U+000A).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::crlf();
    /// assert!(cp.contains("\r\n"));
    /// assert!(!cp.contains("\t"));
    /// ```
    pub fn crlf() -> Self {
        Self::from_slice(ascii::CRLF_CHARS)
    }

    /// Returns a cached static reference to the CRLF character set.
    pub fn crlf_cached() -> &'static CodePoints {
        static INSTANCE: OnceLock<CodePoints> = OnceLock::new();
        INSTANCE.get_or_init(Self::crlf)
    }

    /// Creates a new set containing **all** 128 ASCII characters
    /// (control + printable).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use japanese_codepoints::CodePoints;
    ///
    /// let cp = CodePoints::ascii_all();
    /// assert!(cp.contains("Hello\n"));
    /// assert!(!cp.contains("あ"));
    /// ```
    pub fn ascii_all() -> Self {
        let mut cps = HashSet::new();
        cps.extend(ascii::CONTROL_CHARS.iter());
        cps.extend(ascii::PRINTABLE_CHARS.iter());
        // CRLF is a subset of CONTROL_CHARS; extend on a HashSet is idempotent.
        Self { codepoints: cps }
    }

    /// Returns a cached static reference to the full ASCII character set.
    pub fn ascii_all_cached() -> &'static CodePoints {
        static INSTANCE: OnceLock<CodePoints> = OnceLock::new();
        INSTANCE.get_or_init(Self::ascii_all)
    }
}

// ── trait implementations ────────────────────────────────────────────────────

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
        // Sort for deterministic hashing regardless of HashSet iteration order.
        let mut sorted: Vec<&u32> = self.codepoints.iter().collect();
        sorted.sort_unstable();
        sorted.hash(state);
    }
}

// ── multi-set membership ──────────────────────────────────────────────────────

/// Returns `true` if **every** character in `text` belongs to **at least one**
/// of the provided character sets.
///
/// This is the idiomatic way to check text that may contain characters from
/// multiple scripts — for example Japanese hiragana mixed with ASCII
/// punctuation.
///
/// # Edge cases
///
/// * An empty `text` returns `true` (vacuously).
/// * An empty `sets` slice returns `false` for any input (including empty).
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::{CodePoints, contains_all_in_any};
///
/// let hiragana = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
/// let katakana = CodePoints::new(vec![0x30A2, 0x30A4]); // ア, イ
///
/// // Each character is valid in at least one set
/// assert!(contains_all_in_any("あア", &[&hiragana, &katakana]));
///
/// // 'x' is not in either set
/// assert!(!contains_all_in_any("あx", &[&hiragana, &katakana]));
/// ```
pub fn contains_all_in_any(text: &str, sets: &[&CodePoints]) -> bool {
    if sets.is_empty() {
        return false;
    }
    text.chars()
        .all(|c| sets.iter().any(|set| set.contains_char(c)))
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── construction ──────────────────────────────────────────────────────

    #[test]
    fn test_new_deduplicates() {
        let cp = CodePoints::new(vec![0x3042, 0x3042, 0x3044]);
        assert_eq!(cp.len(), 2);
    }

    #[test]
    fn test_from_slice() {
        let cp = CodePoints::from_slice(&[0x3042, 0x3044]);
        assert!(cp.contains("あい"));
        assert_eq!(cp.len(), 2);
    }

    #[test]
    fn test_from_string() {
        let cp = CodePoints::from_string("あいあ");
        assert_eq!(cp.len(), 2);
        assert!(cp.contains("あい"));
    }

    #[test]
    fn test_empty() {
        let cp = CodePoints::new(vec![]);
        assert!(cp.is_empty());
        assert!(cp.contains("")); // empty string is always valid
        assert!(!cp.contains("a")); // any character fails
    }

    // ── membership ────────────────────────────────────────────────────────

    #[test]
    fn test_contains_basic() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert!(cp.contains("あ"));
        assert!(cp.contains("あい"));
        assert!(!cp.contains("う"));
        assert!(!cp.contains("あいう"));
        assert!(cp.contains(""));
    }

    #[test]
    fn test_contains_char() {
        let cp = CodePoints::new(vec![0x3042]); // あ
        assert!(cp.contains_char('あ'));
        assert!(!cp.contains_char('い'));
    }

    #[test]
    fn test_contains_surrogate_pairs() {
        // U+2000B is outside the BMP; Rust represents it as a single char.
        let cp = CodePoints::new(vec![0x2000B, 0x3042, 0x3044]);
        assert!(cp.contains("𠀋あい"));
        assert!(!cp.contains("𠀋あいか")); // か not in set
    }

    #[test]
    fn test_contains_mixed_characters() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046, 0x3048, 0x304A, 0x2000B]);
        assert!(cp.contains("𠀋あいうあ"));
        assert!(!cp.contains("𠀋あいうか")); // か not in set
    }

    // ── exclusion queries ─────────────────────────────────────────────────

    #[test]
    fn test_first_excluded() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert_eq!(cp.first_excluded("あい"), None);
        assert_eq!(cp.first_excluded("あいう"), Some(0x3046)); // う
    }

    #[test]
    fn test_first_excluded_empty() {
        let cp = CodePoints::new(vec![0x3042]);
        assert_eq!(cp.first_excluded(""), None);
    }

    #[test]
    fn test_first_excluded_with_position() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert_eq!(cp.first_excluded_with_position("あいう"), Some((0x3046, 2)));
        assert_eq!(cp.first_excluded_with_position("あい"), None);
    }

    #[test]
    fn test_first_excluded_surrogate() {
        // あ, い, う
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046]);
        // 𠀋 (U+2000B) is the first excluded character
        assert_eq!(cp.first_excluded("𠀋あいう"), Some(0x2000B));
    }

    #[test]
    fn test_all_excluded_order() {
        // あ, い
        let cp = CodePoints::new(vec![0x3042, 0x3044]);
        // う appears before え; duplicate う is skipped
        assert_eq!(cp.all_excluded("あいうえ"), vec![0x3046, 0x3048]);
    }

    #[test]
    fn test_all_excluded_empty() {
        let cp = CodePoints::new(vec![0x3042]);
        assert_eq!(cp.all_excluded(""), Vec::<u32>::new());
    }

    #[test]
    fn test_all_excluded_surrogate() {
        // あ, い
        let cp = CodePoints::new(vec![0x3042, 0x3044]);
        // 𠀋 (U+2000B) then き (U+304D)
        let result = cp.all_excluded("あ𠀋いき");
        assert_eq!(result, vec![0x2000B, 0x304D]);
    }

    #[test]
    fn test_all_excluded_multiple_surrogates() {
        let cp = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        let result = cp.all_excluded("𠀋あいうきかくか𠂟");
        // 𠀋, き, か, く, 𠂟  (か deduplicated)
        assert_eq!(result, vec![0x2000B, 0x304D, 0x304B, 0x304F, 0x2009F]);
    }

    // ── validation ────────────────────────────────────────────────────────

    #[test]
    fn test_validate_ok() {
        let cp = CodePoints::ascii_printable();
        assert!(cp.validate("Hello World!").is_ok());
    }

    #[test]
    fn test_validate_err() {
        let cp = CodePoints::ascii_printable();
        let err = cp.validate("hello\0world").unwrap_err();
        assert_eq!(err.code_point, 0);
        assert_eq!(err.position, 5);
    }

    // ── set operations ────────────────────────────────────────────────────

    #[test]
    fn test_union() {
        let a = CodePoints::new(vec![0x3042, 0x3044]);
        let b = CodePoints::new(vec![0x3044, 0x3046]);
        let u = a.union(&b);
        assert_eq!(u.len(), 3);
        assert!(u.contains("あいう"));
    }

    #[test]
    fn test_intersection() {
        let a = CodePoints::new(vec![0x3042, 0x3044]);
        let b = CodePoints::new(vec![0x3044, 0x3046]);
        let i = a.intersection(&b);
        assert_eq!(i.len(), 1);
        assert!(i.contains("い"));
        assert!(!i.contains("あ"));
    }

    #[test]
    fn test_difference() {
        let a = CodePoints::new(vec![0x3042, 0x3044]);
        let b = CodePoints::new(vec![0x3044, 0x3046]);
        let d = a.difference(&b);
        assert_eq!(d.len(), 1);
        assert!(d.contains("あ"));
        assert!(!d.contains("い"));
    }

    #[test]
    fn test_symmetric_difference() {
        let a = CodePoints::new(vec![0x3042, 0x3044]);
        let b = CodePoints::new(vec![0x3044, 0x3046]);
        let s = a.symmetric_difference(&b);
        assert_eq!(s.len(), 2);
        assert!(s.contains("あ"));
        assert!(s.contains("う"));
        assert!(!s.contains("い"));
    }

    #[test]
    fn test_subset_superset() {
        let small = CodePoints::new(vec![0x3042]);
        let big = CodePoints::new(vec![0x3042, 0x3044]);
        assert!(small.is_subset_of(&big));
        assert!(big.is_superset_of(&small));
        assert!(!big.is_subset_of(&small));
        assert!(!small.is_superset_of(&big));
    }

    #[test]
    fn test_set_ops_with_empty() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]);
        let empty = CodePoints::new(vec![]);

        assert!(cp.intersection(&empty).is_empty());
        assert_eq!(cp.union(&empty).len(), 2);
        assert_eq!(cp.difference(&empty).len(), 2);
        assert!(empty.difference(&cp).is_empty());
    }

    // ── ASCII factories ───────────────────────────────────────────────────

    #[test]
    fn test_ascii_control() {
        let cp = CodePoints::ascii_control();
        assert!(cp.contains("\n\r\t"));
        assert!(!cp.contains("a"));
        assert!(!cp.contains("あ"));
    }

    #[test]
    fn test_ascii_printable() {
        let cp = CodePoints::ascii_printable();
        assert!(cp.contains("Hello 123!@#~"));
        assert!(!cp.contains("\n"));
        assert!(!cp.contains("あ"));
        // JIS X 0201 special chars NOT in plain ASCII printable
        assert!(!cp.contains("Hello‾")); // Overline
        assert!(!cp.contains("¥100")); // Yen symbol
    }

    #[test]
    fn test_crlf() {
        let cp = CodePoints::crlf();
        assert!(cp.contains("\r\n"));
        assert!(!cp.contains("\t"));
        assert!(!cp.contains("a"));
    }

    #[test]
    fn test_ascii_all() {
        let cp = CodePoints::ascii_all();
        assert!(cp.contains("Hello\n\r\t"));
        assert!(!cp.contains("あ"));
    }

    #[test]
    fn test_ascii_cached_identity() {
        // Each cached() call must return the exact same pointer.
        assert!(std::ptr::eq(
            CodePoints::ascii_control_cached(),
            CodePoints::ascii_control_cached()
        ));
        assert!(std::ptr::eq(
            CodePoints::ascii_printable_cached(),
            CodePoints::ascii_printable_cached()
        ));
        assert!(std::ptr::eq(
            CodePoints::crlf_cached(),
            CodePoints::crlf_cached()
        ));
        assert!(std::ptr::eq(
            CodePoints::ascii_all_cached(),
            CodePoints::ascii_all_cached()
        ));
    }

    #[test]
    fn test_ascii_cached_equals_uncached() {
        assert_eq!(
            *CodePoints::ascii_control_cached(),
            CodePoints::ascii_control()
        );
        assert_eq!(
            *CodePoints::ascii_printable_cached(),
            CodePoints::ascii_printable()
        );
        assert_eq!(*CodePoints::crlf_cached(), CodePoints::crlf());
        assert_eq!(*CodePoints::ascii_all_cached(), CodePoints::ascii_all());
    }

    // ── trait impls ───────────────────────────────────────────────────────

    #[test]
    fn test_display() {
        let cp = CodePoints::new(vec![0x3042, 0x3044]);
        assert_eq!(cp.to_string(), "CodePoints(2 items)");
    }

    #[test]
    fn test_from_vec() {
        let cp: CodePoints = vec![0x3042u32].into();
        assert!(cp.contains("あ"));
    }

    #[test]
    fn test_from_str() {
        let cp: CodePoints = "あい".into();
        assert_eq!(cp.len(), 2);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Two sets with same elements but potentially different insertion order.
        let a = CodePoints::new(vec![0x3042, 0x3044]);
        let b = CodePoints::new(vec![0x3044, 0x3042]);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);

        assert_eq!(a, b);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── contains_all_in_any ───────────────────────────────────────────────

    #[test]
    fn test_contains_all_in_any_basic() {
        let hira = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ, い, う
        let kata = CodePoints::new(vec![0x30A2, 0x30A4, 0x30A6]); // ア, イ, ウ

        assert!(contains_all_in_any("あア", &[&hira, &kata]));
        assert!(contains_all_in_any("あいう", &[&hira]));
        assert!(contains_all_in_any("アイウ", &[&kata]));
        assert!(!contains_all_in_any("xyz", &[&hira, &kata]));
        assert!(!contains_all_in_any("あアx", &[&hira, &kata])); // x not in either
    }

    #[test]
    fn test_contains_all_in_any_empty_text() {
        let cp = CodePoints::new(vec![0x3042]);
        // Empty text with non-empty sets → vacuously true
        assert!(contains_all_in_any("", &[&cp]));
    }

    #[test]
    fn test_contains_all_in_any_empty_sets() {
        // Empty sets → always false
        assert!(!contains_all_in_any("a", &[]));
        assert!(!contains_all_in_any("", &[]));
    }

    #[test]
    fn test_contains_all_in_any_three_sets() {
        let hira = CodePoints::new(vec![0x3042]); // あ
        let kata = CodePoints::new(vec![0x30A2]); // ア
        let ascii = CodePoints::ascii_printable();

        // Each char in a different set
        assert!(contains_all_in_any("あアA", &[&hira, &kata, &ascii]));
        // π (U+03C0) not in any
        assert!(!contains_all_in_any("あアAπ", &[&hira, &kata, &ascii]));
        // "Hello" is entirely in ascii
        assert!(contains_all_in_any("Hello", &[&hira, &kata, &ascii]));
    }

    #[test]
    fn test_contains_all_in_any_overlap() {
        // Character present in multiple sets — should still pass.
        let cp1 = CodePoints::new(vec![0x3042, 0x3046]); // あ, う
        let cp2 = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
        assert!(contains_all_in_any("あいう", &[&cp1, &cp2]));
    }
}
