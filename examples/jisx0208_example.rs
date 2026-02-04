//! JIS X 0208 character set example (non-kanji).
//!
//! Includes: Hiragana, Katakana, fullwidth Latin, Greek, Cyrillic,
//! special characters, and box drawing characters.
//!
//! Run: `cargo run --example jisx0208_example --features codepoints-jisx0208`

use japanese_codepoints::jisx0208::{
    BoxDrawingChars, CyrillicLetters, GreekLetters, Hiragana, JisX0208, Katakana, LatinLetters,
    SpecialChars,
};

fn main() {
    // --- Hiragana ---

    let hiragana = Hiragana::cached();

    assert!(hiragana.contains("あいうえお"));
    assert!(hiragana.contains("がぎぐげご")); // voiced
    assert!(hiragana.contains("ぱぴぷぺぽ")); // semi-voiced
    assert!(!hiragana.contains("アイウエオ")); // katakana
    assert!(!hiragana.contains("ABC"));

    println!("Hiragana: {} chars", hiragana.codepoints().len());

    // --- Katakana ---

    let katakana = Katakana::cached();

    assert!(katakana.contains("アイウエオ"));
    assert!(katakana.contains("ガギグゲゴ"));
    assert!(!katakana.contains("あいうえお"));

    println!("Katakana: {} chars", katakana.codepoints().len());

    // --- Fullwidth Latin ---

    let latin = LatinLetters::cached();

    assert!(latin.contains("ＡＢＣＤＥ"));
    assert!(latin.contains("ａｂｃｄｅ"));
    assert!(latin.contains("１２３４５"));
    assert!(!latin.contains("ABCDE")); // halfwidth

    println!("Latin: {} chars", latin.codepoints().len());

    // --- Greek ---

    let greek = GreekLetters::cached();

    assert!(greek.contains("ΑΒΓΔΕ"));
    assert!(greek.contains("αβγδε"));
    assert!(!greek.contains("ABC"));

    println!("Greek: {} chars", greek.codepoints().len());

    // --- Cyrillic ---

    let cyrillic = CyrillicLetters::cached();

    assert!(cyrillic.contains("АБВГД"));
    assert!(cyrillic.contains("абвгд"));
    assert!(!cyrillic.contains("ABC"));

    println!("Cyrillic: {} chars", cyrillic.codepoints().len());

    // --- Special characters ---

    let special = SpecialChars::cached();

    assert!(special.contains("、。"));
    assert!(special.contains("☆★"));
    assert!(special.contains("〒※"));

    println!("Special: {} chars", special.codepoints().len());

    // --- Box drawing ---

    let box_draw = BoxDrawingChars::cached();

    assert!(box_draw.contains("─│┌┐└┘"));
    assert!(box_draw.contains("━┃┏┓┗┛"));

    println!("Box drawing: {} chars", box_draw.codepoints().len());

    // --- Full JIS X 0208 (all combined) ---

    let full = JisX0208::cached();

    assert!(full.contains("あいうアイウ"));
    assert!(full.contains("ＡＢＣあいう"));
    assert!(full.contains("ΑΒΓαβγ"));
    assert!(full.contains("АБВабв"));
    assert!(full.contains("─│┌┐"));

    // NOT included
    assert!(!full.contains("漢字")); // kanji in separate module
    assert!(!full.contains("ｱｲｳ")); // halfwidth katakana (JIS X 0201)
    assert!(!full.contains("ABC")); // halfwidth Latin (ASCII)

    println!("Full JIS X 0208: {} chars", full.codepoints().len());

    // --- Validation ---

    assert!(full.validate("あいうアイウ").is_ok());

    let err = full.validate("あいう漢字").unwrap_err();
    assert_eq!(err.code_point, '漢' as u32);
    println!("Validation error: {}", err);

    let err = full.validate("Hello").unwrap_err();
    assert_eq!(err.code_point, 'H' as u32);
    println!("Validation error: {}", err);

    println!("All tests passed!");
}
