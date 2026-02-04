//! JIS X 0208 Kanji example.
//!
//! Contains 6,355 kanji:
//! - Level 1: 2,965 chars (sorted by reading)
//! - Level 2: 3,390 chars (sorted by radical)
//!
//! Run: `cargo run --example jisx0208kanji_example --features codepoints-jisx0208kanji`

use japanese_codepoints::jisx0208kanji::JisX0208Kanji;

fn main() {
    let kanji = JisX0208Kanji::cached();

    // --- Basic checks ---

    assert_eq!(kanji.codepoints().len(), 6355);
    println!("Total: {} kanji", kanji.codepoints().len());

    // --- Level 1 kanji (common) ---

    assert!(kanji.contains("亜")); // first in Level 1
    assert!(kanji.contains("愛"));
    assert!(kanji.contains("安"));
    assert!(kanji.contains("一"));
    assert!(kanji.contains("日本語"));
    assert!(kanji.contains("漢字"));

    println!("Level 1 kanji OK");

    // --- Level 2 kanji (less common) ---

    assert!(kanji.contains("堯"));
    assert!(kanji.contains("槇"));
    assert!(kanji.contains("遙"));
    assert!(kanji.contains("瑤"));
    assert!(kanji.contains("凜"));
    assert!(kanji.contains("熙"));

    println!("Level 2 kanji OK");

    // --- Non-kanji rejected ---

    assert!(!kanji.contains("ABC"));
    assert!(!kanji.contains("あいう")); // hiragana
    assert!(!kanji.contains("アイウ")); // katakana
    assert!(!kanji.contains("123"));

    println!("Non-kanji rejection OK");

    // --- Mixed strings ---

    assert!(kanji.contains("亜愛安以伊位一乙王黄"));
    assert!(!kanji.contains("亜ABC愛"));
    assert!(!kanji.contains("漢字あいう"));

    println!("Mixed string tests OK");

    // --- Validation ---

    assert!(kanji.validate("亜愛安").is_ok());
    assert!(kanji.validate("日本語").is_ok());

    let err = kanji.validate("亜ABC愛").unwrap_err();
    assert_eq!(err.code_point, 'A' as u32);
    assert_eq!(err.position, 1);
    println!("Validation error: {}", err);

    let err = kanji.validate("漢字あいう").unwrap_err();
    assert_eq!(err.code_point, 'あ' as u32);
    println!("Validation error: {}", err);

    println!("All tests passed!");
}
