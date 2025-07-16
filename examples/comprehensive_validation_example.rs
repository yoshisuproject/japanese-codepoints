//! Comprehensive validation example demonstrating all validation features
//! 
//! This example covers:
//! 1. Basic validation macros
//! 2. Advanced validation with multiple patterns
//! 3. Feature-specific Japanese character validation
//! 4. Static caching for performance
//! 5. Multi-CodePoints validation
//! 6. Performance comparisons

use japanese_codepoints::{validate_codepoints, CodePoints};

// Feature-gated imports for Japanese character sets
#[cfg(feature = "codepoints-jisx0208")]
use japanese_codepoints::{
    validate_hiragana, validate_katakana, validate_japanese_kana, validate_japanese_mixed,
    jisx0208::{Hiragana, Katakana}
};

#[cfg(feature = "codepoints-jisx0201")]
use japanese_codepoints::{validate_jisx0201_katakana, validate_jisx0201_latin};

// Helper macro for Japanese validation tests - must be defined before use
macro_rules! test_japanese_macro {
    ($macro_name:ident, $text:expr, $description:expr) => {
        match $macro_name!($text) {
            Ok(()) => println!("  ✅ '{}' ({}): Valid", $text, $description),
            Err(_) => println!("  ❌ '{}' ({}): Invalid", $text, $description),
        }
    };
}

fn main() {
    println!("🚀 Japanese CodePoints - Comprehensive Validation Example");
    println!("=========================================================\n");
    
    // 1. Basic validation
    demo_basic_validation();
    
    // 2. Advanced validation patterns
    demo_advanced_validation();
    
    // 3. Feature-specific Japanese validation
    #[cfg(feature = "codepoints-jisx0208")]
    demo_japanese_validation();
    
    #[cfg(feature = "codepoints-jisx0201")]
    demo_jisx0201_validation();
    
    // 4. Caching functionality
    demo_caching_functionality();
    
    // 5. Multi-CodePoints validation
    demo_multi_codepoints_validation();
    
    // 6. Performance comparison
    demo_performance_comparison();
    
    println!("\n✅ All validation examples completed successfully!");
}

fn demo_basic_validation() {
    println!("📋 1. Basic Validation");
    println!("====================");
    
    let ascii = CodePoints::ascii_printable();
    let custom_chars = CodePoints::new(vec![0x3042, 0x3044]); // あ, い
    
    println!("ASCII printable character validation:");
    test_validation("hello", &ascii, "Basic ASCII text");
    test_validation("hello\n", &ascii, "ASCII with newline (should fail)");
    
    println!("\nCustom character set validation:");
    test_validation_with_macro("あい", custom_chars.clone(), "Valid Japanese chars");
    test_validation_with_macro("あいう", custom_chars, "Invalid Japanese chars");
    
    println!();
}

fn demo_advanced_validation() {
    println!("🔧 2. Advanced Validation Patterns");
    println!("=================================");
    
    let ascii = CodePoints::ascii_printable();
    
    // Pattern 1: Custom error messages
    println!("Custom error messages:");
    match japanese_codepoints::validate_codepoints_advanced!("hello", ascii.clone(), "Only ASCII printable allowed") {
        Ok(()) => println!("  ✅ 'hello': Custom validation passed"),
        Err(e) => println!("  ❌ 'hello': {}", e),
    }
    
    match japanese_codepoints::validate_codepoints_advanced!("hello\n", ascii.clone(), "Only ASCII printable allowed") {
        Ok(()) => println!("  ✅ 'hello\\n': Custom validation passed"),
        Err(e) => println!("  ❌ 'hello\\n': {}", e),
    }
    
    // Pattern 2: Detailed validation with position
    println!("\nDetailed validation with character position:");
    match japanese_codepoints::validate_codepoints_advanced!("hello\0world", detailed ascii.clone()) {
        Ok(()) => println!("  ✅ 'hello\\0world': Detailed validation passed"),
        Err(e) => println!("  ❌ 'hello\\0world': {}", e),
    }
    
    // Pattern 3: Predefined shortcuts
    println!("\nPredefined character set shortcuts:");
    match japanese_codepoints::validate_codepoints_advanced!("hello", ascii_printable) {
        Ok(()) => println!("  ✅ 'hello': ASCII printable shortcut passed"),
        Err(e) => println!("  ❌ 'hello': {}", e),
    }
    
    match japanese_codepoints::validate_codepoints_advanced!("\t\n", ascii_control) {
        Ok(()) => println!("  ✅ '\\t\\n': ASCII control shortcut passed"),
        Err(e) => println!("  ❌ '\\t\\n': {}", e),
    }
    
    println!();
}

#[cfg(feature = "codepoints-jisx0208")]
fn demo_japanese_validation() {
    println!("🇯🇵 3. Japanese Character Validation (JIS X 0208)");
    println!("===============================================");
    
    // Hiragana validation
    println!("Hiragana validation:");
    test_japanese_macro!(validate_hiragana, "あいうえお", "Pure hiragana");
    test_japanese_macro!(validate_hiragana, "Hello", "ASCII text (should fail)");
    
    // Katakana validation
    println!("\nKatakana validation:");
    test_japanese_macro!(validate_katakana, "アイウエオ", "Pure katakana");
    test_japanese_macro!(validate_katakana, "あいうえお", "Hiragana text (should fail)");
    
    // Combined kana validation
    println!("\nCombined Hiragana/Katakana validation:");
    test_japanese_macro!(validate_japanese_kana, "あいアイ", "Mixed hiragana and katakana");
    test_japanese_macro!(validate_japanese_kana, "Hello", "ASCII text (should fail)");
    
    // Mixed Japanese and ASCII validation
    println!("\nJapanese + ASCII mixed validation:");
    test_japanese_macro!(validate_japanese_mixed, "こんにちはHello", "Japanese + ASCII");
    test_japanese_macro!(validate_japanese_mixed, "こんにちは漢字", "Contains kanji (should fail)");
    
    println!();
}

#[cfg(feature = "codepoints-jisx0201")]
fn demo_jisx0201_validation() {
    println!("📝 4. JIS X 0201 Character Validation");
    println!("===================================");
    
    // JIS X 0201 Katakana validation
    println!("JIS X 0201 Katakana validation:");
    test_japanese_macro!(validate_jisx0201_katakana, "ｱｲｳｴｵ", "Halfwidth katakana");
    test_japanese_macro!(validate_jisx0201_katakana, "アイウエオ", "Fullwidth katakana (should fail)");
    
    // JIS X 0201 Latin validation
    println!("\nJIS X 0201 Latin validation:");
    test_japanese_macro!(validate_jisx0201_latin, "Hello¥", "ASCII + Yen sign");
    test_japanese_macro!(validate_jisx0201_latin, "こんにちは", "Japanese text (should fail)");
    
    println!();
}

fn demo_caching_functionality() {
    println!("⚡ 5. Static Caching Functionality");
    println!("================================");
    
    println!("ASCII character set caching:");
    let printable1 = CodePoints::ascii_printable_cached();
    let printable2 = CodePoints::ascii_printable_cached();
    println!("  Same cached instance: {}", std::ptr::eq(printable1, printable2));
    println!("  Contains 'Hello': {}", printable1.contains("Hello"));
    
    #[cfg(feature = "codepoints-jisx0208")]
    {
        println!("\nJapanese character set caching:");
        let hiragana1 = Hiragana::cached();
        let hiragana2 = Hiragana::cached();
        println!("  Same cached hiragana: {}", std::ptr::eq(hiragana1, hiragana2));
        println!("  Contains 'あいうえお': {}", hiragana1.contains("あいうえお"));
        
        let katakana1 = Katakana::cached();
        let katakana2 = Katakana::cached();
        println!("  Same cached katakana: {}", std::ptr::eq(katakana1, katakana2));
        println!("  Contains 'アイウエオ': {}", katakana1.contains("アイウエオ"));
    }
    
    println!();
}

fn demo_multi_codepoints_validation() {
    println!("🔀 6. Multi-CodePoints Validation");
    println!("================================");
    
    // Create different character sets
    let ascii = CodePoints::ascii_printable();
    let digits = CodePoints::new(vec![0x0030, 0x0031, 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039]); // 0-9
    let special = CodePoints::new(vec![0x0040, 0x0023, 0x0024]); // @, #, $
    
    let collections = [ascii, digits, special];
    
    println!("Testing various character combinations:");
    
    let test_cases = [
        ("Hello123", "ASCII letters + digits"),
        ("Hello@#$", "ASCII letters + special chars"),
        ("123@#$", "Digits + special chars"),
        ("Hello123@#$", "All three types mixed"),
        ("αβγ", "Greek letters (not in any collection)"),
        ("Hello\n123", "Contains newline (invalid)"),
    ];
    
    for (text, description) in test_cases.iter() {
        let is_valid = CodePoints::contains_all_in_any(text, &collections);
        let status = if is_valid { "✅ Valid" } else { "❌ Invalid" };
        println!("  '{}' ({}): {}", text, description, status);
    }
    
    // Advanced multi-collection validation using the function directly
    println!("\nUsing multi-collection validation:");
    let ascii_check = CodePoints::ascii_printable();
    let digit_check = CodePoints::new(vec![0x0030, 0x0031, 0x0032]); // 0, 1, 2
    let test_collections = [ascii_check, digit_check];
    
    let is_valid = CodePoints::contains_all_in_any("Hello012", &test_collections);
    if is_valid {
        println!("  ✅ 'Hello012': Multi-collection validation passed");
    } else {
        println!("  ❌ 'Hello012': Multi-collection validation failed");
    }
    
    #[cfg(feature = "codepoints-jisx0208")]
    {
        println!("\nJapanese multi-character validation:");
        let hiragana = Hiragana::cached();
        let katakana = Katakana::cached();
        let ascii_cached = CodePoints::ascii_printable_cached();
        
        let japanese_collections = [hiragana.codepoints().clone(), katakana.codepoints().clone(), ascii_cached.clone()];
        
        let japanese_test_cases = [
            ("こんにちは", "Pure hiragana"),
            ("コンニチハ", "Pure katakana"),
            ("Hello", "Pure ASCII"),
            ("こんにちはHello", "Hiragana + ASCII"),
            ("こんにちはコンニチハ", "Hiragana + Katakana"),
            ("Helloコンニチハ", "ASCII + Katakana"),
            ("こんにちはコンニチハHello", "All three types"),
            ("こんにちは漢字", "Contains kanji (invalid)"),
        ];
        
        for (text, description) in japanese_test_cases.iter() {
            let is_valid = CodePoints::contains_all_in_any(text, &japanese_collections);
            let status = if is_valid { "✅ Valid" } else { "❌ Invalid" };
            println!("  '{}' ({}): {}", text, description, status);
        }
    }
    
    println!();
}

fn demo_performance_comparison() {
    println!("🏎️  7. Performance Comparison");
    println!("============================");
    
    println!("Comparing cached vs non-cached performance (1000 iterations):");
    
    // Non-cached performance
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _cp = CodePoints::ascii_printable(); // Creates new instance each time
    }
    let non_cached_time = start.elapsed();
    
    // Cached performance
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _cp = CodePoints::ascii_printable_cached(); // Returns same cached instance
    }
    let cached_time = start.elapsed();
    
    println!("  Non-cached creation: {:?}", non_cached_time);
    println!("  Cached access: {:?}", cached_time);
    
    if cached_time.as_nanos() > 0 {
        let improvement = non_cached_time.as_nanos() as f64 / cached_time.as_nanos() as f64;
        println!("  Performance improvement: {:.1}x faster with caching", improvement);
    } else {
        println!("  Cached access is too fast to measure accurately!");
    }
    
    println!();
}

// Helper functions for cleaner test output
fn test_validation(text: &str, codepoints: &CodePoints, description: &str) {
    let result = if codepoints.contains(text) { "✅ Valid" } else { "❌ Invalid" };
    println!("  '{}' ({}): {}", text, description, result);
}

fn test_validation_with_macro(text: &str, codepoints: CodePoints, description: &str) {
    match validate_codepoints!(text, codepoints) {
        Ok(()) => println!("  ✅ '{}' ({}): Valid", text, description),
        Err(_) => println!("  ❌ '{}' ({}): Invalid", text, description),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_validation() {
        let ascii = CodePoints::ascii_printable();
        assert!(validate_codepoints!("hello", ascii.clone()).is_ok());
        assert!(validate_codepoints!("hello\n", ascii).is_err());
    }

    #[test]
    fn test_advanced_validation() {
        let ascii = CodePoints::ascii_printable();
        assert!(japanese_codepoints::validate_codepoints_advanced!("hello", ascii.clone(), "error").is_ok());
        assert!(japanese_codepoints::validate_codepoints_advanced!("hello\n", ascii, "error").is_err());
    }

    #[test]
    fn test_caching() {
        let cached1 = CodePoints::ascii_printable_cached();
        let cached2 = CodePoints::ascii_printable_cached();
        assert!(std::ptr::eq(cached1, cached2));
    }

    #[test]
    #[cfg(feature = "codepoints-jisx0208")]
    fn test_japanese_validation() {
        assert!(validate_hiragana!("あいう").is_ok());
        assert!(validate_hiragana!("Hello").is_err());
        assert!(validate_katakana!("アイウ").is_ok());
        assert!(validate_japanese_kana!("あいアイ").is_ok());
        assert!(validate_japanese_kana!("Hello").is_err());
        assert!(validate_japanese_mixed!("こんにちはHello").is_ok());
    }

    #[test]
    fn test_multi_codepoints() {
        let ascii = CodePoints::ascii_printable();
        let digits = CodePoints::new(vec![0x30, 0x31, 0x32]); // 0, 1, 2
        let collections = [ascii, digits];
        
        assert!(CodePoints::contains_all_in_any("Hello012", &collections));
        assert!(!CodePoints::contains_all_in_any("Hello789", &collections));
    }
}