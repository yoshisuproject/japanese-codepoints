/// Basic macro for validating that a string contains only allowed code points
///
/// This macro checks if all characters in the given string are present in the specified
/// CodePoints collection.
///
/// # Arguments
///
/// * `$value` - The string to validate
/// * `$codepoints` - A CodePoints instance or expression that returns one
///
/// # Returns
///
/// `Ok(())` if all characters are valid, `Err(String)` with an error message otherwise.
///
/// # Examples
///
/// ```rust
/// use japanese_codepoints::{validate_codepoints, CodePoints};
///
/// let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
///
/// // Valid string
/// let result = validate_codepoints!("あい", cp.clone());
/// assert!(result.is_ok());
///
/// // Invalid string
/// let result = validate_codepoints!("あいう", cp);
/// assert!(result.is_err());
/// ```
#[macro_export]
macro_rules! validate_codepoints {
    ($value:expr, $codepoints:expr) => {{
        let cp = $codepoints;

        if !cp.contains($value) {
            Err(format!("Value contains invalid code points"))
        } else {
            Ok(())
        }
    }};
}

/// Advanced macro for validating strings against multiple character sets with various options
///
/// This macro provides multiple validation patterns:
/// 1. Single character set validation with custom error messages
/// 2. Multiple character set validation (any_of pattern)
/// 3. Detailed validation with position information
/// 4. Predefined character set shortcuts
///
/// # Patterns
///
/// ## Single character set with custom message
/// ```rust
/// use japanese_codepoints::{validate_codepoints_advanced, CodePoints};
/// 
/// let cp = CodePoints::ascii_printable();
/// let result = validate_codepoints_advanced!("hello", cp, "Only ASCII characters allowed");
/// assert!(result.is_ok());
/// ```
///
/// ## Multiple character sets (any_of pattern)
/// ```rust
/// use japanese_codepoints::{validate_codepoints_advanced, CodePoints};
/// 
/// let ascii_printable = CodePoints::ascii_printable();
/// let ascii_control = CodePoints::ascii_control();
/// let collections = [ascii_printable, ascii_control];
/// if CodePoints::contains_all_in_any("test\n", &collections) {
///     println!("Valid");
/// }
/// ```
///
/// ## Detailed validation with position
/// ```rust
/// use japanese_codepoints::{validate_codepoints_advanced, CodePoints};
/// 
/// let cp = CodePoints::ascii_printable();
/// let result = validate_codepoints_advanced!("hello\0world", detailed cp);
/// // Returns Err with position information
/// ```
///
/// ## Predefined character set shortcuts
/// ```rust
/// use japanese_codepoints::{validate_codepoints_advanced, CodePoints};
/// 
/// let ascii = CodePoints::ascii_printable();
/// let result = validate_codepoints_advanced!("hello", ascii);
/// assert!(result.is_ok());
/// ```
///
/// ## Combined character sets example
/// ```rust
/// use japanese_codepoints::{CodePoints};
/// 
/// # #[cfg(feature = "codepoints-jisx0208")]
/// {
///     use japanese_codepoints::validate_japanese_mixed;
///     let result = validate_japanese_mixed!("こんにちはHello");
///     assert!(result.is_ok());
/// }
/// ```
#[macro_export]
macro_rules! validate_codepoints_advanced {
    // Pattern 1: Single character set with custom error message
    ($value:expr, $codepoints:expr, $error_msg:expr) => {{
        let cp = $codepoints;
        let val = $value;
        
        if !cp.contains(val) {
            Err($error_msg.to_string())
        } else {
            Ok(())
        }
    }};
    
    // Pattern 2: Multiple character sets validation (any_of pattern)
    ($value:expr, any_of [$($codepoints:expr),+ $(,)?]) => {{
        let val = $value;
        let collections = [$($codepoints),+];
        
        if $crate::CodePoints::contains_all_in_any(val, &collections) {
            Ok(())
        } else {
            if let Some((invalid_char, position)) = collections[0].first_excluded_with_position(val) {
                Err(format!(
                    "Invalid character '{}' (U+{:04X}) at position {} - not allowed in any of the specified character sets",
                    char::from_u32(invalid_char).unwrap_or('�'),
                    invalid_char,
                    position
                ))
            } else {
                Err("Value contains invalid code points".to_string())
            }
        }
    }};
    
    // Pattern 3: Detailed validation with position information
    ($value:expr, detailed $codepoints:expr) => {{
        let cp = $codepoints;
        let val = $value;
        
        if cp.contains(val) {
            Ok(())
        } else {
            if let Some((invalid_char, position)) = cp.first_excluded_with_position(val) {
                Err(format!(
                    "Invalid character '{}' (U+{:04X}) at position {}",
                    char::from_u32(invalid_char).unwrap_or('�'),
                    invalid_char,
                    position
                ))
            } else {
                Err("Value contains invalid code points".to_string())
            }
        }
    }};
    
    // Pattern 4: Predefined character set shortcuts
    ($value:expr, ascii_control) => {{
        let cp = $crate::CodePoints::ascii_control_cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-ASCII control characters", $value))
        } else {
            Ok(())
        }
    }};
    
    ($value:expr, ascii_printable) => {{
        let cp = $crate::CodePoints::ascii_printable_cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-ASCII printable characters", $value))
        } else {
            Ok(())
        }
    }};
    
    ($value:expr, crlf) => {{
        let cp = $crate::CodePoints::crlf_cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-CRLF characters", $value))
        } else {
            Ok(())
        }
    }};
    
    ($value:expr, ascii_all) => {{
        let cp = $crate::CodePoints::ascii_all_cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-ASCII characters", $value))
        } else {
            Ok(())
        }
    }};
    
    
    // Fallback to original pattern for backward compatibility
    ($value:expr, $codepoints:expr) => {{
        let cp = $codepoints;
        
        if !cp.contains($value) {
            Err(format!("Value contains invalid code points"))
        } else {
            Ok(())
        }
    }};
}

// Feature-specific validation macros for JIS character sets
// These are separate to avoid feature compilation issues in the main macro

/// Validates text using JIS X 0208 Hiragana characters
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_hiragana {
    ($value:expr) => {{
        let cp = $crate::jisx0208::Hiragana::cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-Hiragana characters", $value))
        } else {
            Ok(())
        }
    }};
}

/// Validates text using JIS X 0208 Katakana characters
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_katakana {
    ($value:expr) => {{
        let cp = $crate::jisx0208::Katakana::cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-Katakana characters", $value))
        } else {
            Ok(())
        }
    }};
}

/// Validates text using either Hiragana or Katakana characters
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_japanese_kana {
    ($value:expr) => {{
        let hiragana = $crate::jisx0208::Hiragana::cached();
        let katakana = $crate::jisx0208::Katakana::cached();
        let collections = [hiragana.codepoints().clone(), katakana.codepoints().clone()];
        
        if $crate::CodePoints::contains_all_in_any($value, &collections) {
            Ok(())
        } else {
            Err(format!("Value '{}' contains characters that are not Hiragana or Katakana", $value))
        }
    }};
}

/// Validates text using Hiragana, Katakana, or ASCII printable characters
#[cfg(feature = "codepoints-jisx0208")]
#[macro_export]
macro_rules! validate_japanese_mixed {
    ($value:expr) => {{
        let hiragana = $crate::jisx0208::Hiragana::cached();
        let katakana = $crate::jisx0208::Katakana::cached();
        let ascii = $crate::CodePoints::ascii_printable_cached();
        let collections = [hiragana.codepoints().clone(), katakana.codepoints().clone(), ascii.clone()];
        
        if $crate::CodePoints::contains_all_in_any($value, &collections) {
            Ok(())
        } else {
            Err(format!("Value '{}' contains invalid characters for Japanese text", $value))
        }
    }};
}

/// Validates text using JIS X 0201 Katakana characters
#[cfg(feature = "codepoints-jisx0201")]
#[macro_export]
macro_rules! validate_jisx0201_katakana {
    ($value:expr) => {{
        let cp = $crate::jisx0201::Katakana::cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-JIS X 0201 Katakana characters", $value))
        } else {
            Ok(())
        }
    }};
}

/// Validates text using JIS X 0201 Latin characters
#[cfg(feature = "codepoints-jisx0201")]
#[macro_export]
macro_rules! validate_jisx0201_latin {
    ($value:expr) => {{
        let cp = $crate::jisx0201::LatinLetters::cached();
        if !cp.contains($value) {
            Err(format!("Value '{}' contains non-JIS X 0201 Latin characters", $value))
        } else {
            Ok(())
        }
    }};
}
