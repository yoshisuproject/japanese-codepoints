//! Comprehensive validation example.
//!
//! Demonstrates all validation features:
//! - `validate()` method
//! - Validation macros
//! - Multi-set validation
//! - Static caching
//!
//! Run: `cargo run --example comprehensive_validation_example --features full`

use japanese_codepoints::{contains_all_in_any, CodePoints};

fn main() {
    test_basic_validation();
    test_macro_validation();

    #[cfg(feature = "codepoints-jisx0208")]
    test_japanese_validation();

    #[cfg(feature = "codepoints-jisx0201")]
    test_jisx0201_validation();

    test_caching();
    test_multi_set_validation();

    println!("All tests passed!");
}

fn test_basic_validation() {
    let ascii = CodePoints::ascii_printable_cached();

    assert!(ascii.validate("hello").is_ok());

    let err = ascii.validate("hello\n").unwrap_err();
    assert_eq!(err.code_point, '\n' as u32);
    assert_eq!(err.position, 5);

    let err = ascii.validate("hello\0world").unwrap_err();
    assert_eq!(err.code_point, 0);
    assert_eq!(err.position, 5);

    println!("Basic validation OK");
}

fn test_macro_validation() {
    let ascii = CodePoints::ascii_printable_cached();

    // validate_codepoints!
    assert!(japanese_codepoints::validate_codepoints!("hello", ascii).is_ok());
    assert!(japanese_codepoints::validate_codepoints!("hello\n", ascii).is_err());

    // validate_codepoints_advanced! with custom message
    let result = japanese_codepoints::validate_codepoints_advanced!(
        "hello\n",
        ascii,
        "Only printable ASCII"
    );
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Only printable ASCII"));

    // validate_codepoints_advanced! with detailed error
    let result = japanese_codepoints::validate_codepoints_advanced!("hello\0", detailed ascii);
    let err = result.unwrap_err();
    assert_eq!(err.code_point, 0);
    assert_eq!(err.position, 5);

    println!("Macro validation OK");
}

#[cfg(feature = "codepoints-jisx0208")]
fn test_japanese_validation() {
    use japanese_codepoints::jisx0208::{Hiragana, Katakana};

    // Hiragana
    assert!(japanese_codepoints::validate_hiragana!("あいうえお").is_ok());
    assert!(japanese_codepoints::validate_hiragana!("Hello").is_err());

    // Katakana
    assert!(japanese_codepoints::validate_katakana!("アイウエオ").is_ok());
    assert!(japanese_codepoints::validate_katakana!("あいうえお").is_err());

    // Kana (hiragana or katakana)
    assert!(japanese_codepoints::validate_japanese_kana!("あいアイ").is_ok());
    assert!(japanese_codepoints::validate_japanese_kana!("Hello").is_err());

    // Japanese mixed (hiragana + katakana + ASCII)
    assert!(japanese_codepoints::validate_japanese_mixed!("こんにちはHello").is_ok());
    assert!(japanese_codepoints::validate_japanese_mixed!("こんにちは漢字").is_err());

    // Direct validation
    let hiragana = Hiragana::cached();
    let katakana = Katakana::cached();

    assert!(hiragana.validate("あいうえお").is_ok());
    assert!(katakana.validate("アイウエオ").is_ok());

    println!("Japanese validation OK");
}

#[cfg(feature = "codepoints-jisx0201")]
fn test_jisx0201_validation() {
    // Halfwidth katakana
    assert!(japanese_codepoints::validate_jisx0201_katakana!("ｱｲｳｴｵ").is_ok());
    assert!(japanese_codepoints::validate_jisx0201_katakana!("アイウエオ").is_err());

    // JIS X 0201 Latin
    assert!(japanese_codepoints::validate_jisx0201_latin!("Hello¥").is_ok());
    assert!(japanese_codepoints::validate_jisx0201_latin!("こんにちは").is_err());

    println!("JIS X 0201 validation OK");
}

fn test_caching() {
    // Same pointer returned
    let p1 = CodePoints::ascii_printable_cached();
    let p2 = CodePoints::ascii_printable_cached();
    assert!(std::ptr::eq(p1, p2));

    #[cfg(feature = "codepoints-jisx0208")]
    {
        use japanese_codepoints::jisx0208::{Hiragana, Katakana};

        let h1 = Hiragana::cached();
        let h2 = Hiragana::cached();
        assert!(std::ptr::eq(h1, h2));

        let k1 = Katakana::cached();
        let k2 = Katakana::cached();
        assert!(std::ptr::eq(k1, k2));
    }

    println!("Caching OK");
}

fn test_multi_set_validation() {
    use japanese_codepoints::validation::validate_all_in_any;

    let alpha = CodePoints::new((b'A'..=b'Z').chain(b'a'..=b'z').map(u32::from).collect());
    let digits = CodePoints::new((b'0'..=b'9').map(u32::from).collect());

    // contains_all_in_any
    assert!(contains_all_in_any("Hello123", &[&alpha, &digits]));
    assert!(contains_all_in_any("Hello", &[&alpha, &digits]));
    assert!(contains_all_in_any("123", &[&alpha, &digits]));
    assert!(!contains_all_in_any("Hello 123", &[&alpha, &digits])); // space

    // validate_all_in_any
    assert!(validate_all_in_any("Hello123", &[&alpha, &digits]).is_ok());

    let err = validate_all_in_any("Hello 123", &[&alpha, &digits]).unwrap_err();
    assert_eq!(err.code_point, ' ' as u32);
    assert_eq!(err.position, 5);

    #[cfg(feature = "codepoints-jisx0208")]
    {
        use japanese_codepoints::jisx0208::{Hiragana, Katakana};

        let hiragana = Hiragana::cached();
        let katakana = Katakana::cached();
        let ascii = CodePoints::ascii_printable_cached();

        let sets: &[&CodePoints] = &[hiragana.codepoints(), katakana.codepoints(), ascii];

        assert!(contains_all_in_any("こんにちは", sets));
        assert!(contains_all_in_any("コンニチハ", sets));
        assert!(contains_all_in_any("Hello", sets));
        assert!(contains_all_in_any("こんにちはHello", sets));
        assert!(!contains_all_in_any("こんにちは漢字", sets)); // kanji
    }

    println!("Multi-set validation OK");
}
