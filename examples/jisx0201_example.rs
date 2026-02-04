//! JIS X 0201 character set example.
//!
//! JIS X 0201 defines:
//! - Latin letters (ASCII printable with `\` → `¥` and `~` → `‾`)
//! - Halfwidth katakana (U+FF61–U+FF9F)
//!
//! Run: `cargo run --example jisx0201_example --features codepoints-jisx0201`

use japanese_codepoints::jisx0201::{JisX0201, Katakana, LatinLetters};

fn main() {
    let latin = LatinLetters::cached();
    let katakana = Katakana::cached();
    let full = JisX0201::cached();

    // --- Latin letters ---

    assert!(latin.contains("Hello World"));
    assert!(latin.contains("¥100")); // yen sign (replaces \)
    assert!(latin.contains("‾")); // overline (replaces ~)
    assert!(!latin.contains("\\")); // backslash NOT in JIS X 0201
    assert!(!latin.contains("~")); // tilde NOT in JIS X 0201
    assert!(!latin.contains("あ"));

    println!("Latin tests passed");

    // --- Halfwidth katakana ---

    assert!(katakana.contains("ｱｲｳｴｵ"));
    assert!(katakana.contains("ｶｷｸｹｺ"));
    assert!(katakana.contains("｡｢｣､･")); // punctuation
    assert!(!katakana.contains("アイウエオ")); // fullwidth katakana
    assert!(!katakana.contains("あいうえお")); // hiragana

    println!("Katakana tests passed");

    // --- Full JIS X 0201 (Latin ∪ Katakana) ---

    assert!(full.contains("Helloｱｲｳ"));
    assert!(full.contains("¥｡｢｣ｱｲｳ"));
    assert!(!full.contains("あいうえお")); // fullwidth hiragana
    assert!(!full.contains("漢字"));

    println!("Full JIS X 0201 tests passed");

    // --- Validation ---

    assert!(full.validate("Hello¥ｱｲｳ").is_ok());

    let err = full.validate("Hello\\ｱｲｳ").unwrap_err();
    assert_eq!(err.code_point, '\\' as u32);
    assert_eq!(err.position, 5);
    println!("Validation error: {}", err);

    let err = full.validate("あいう").unwrap_err();
    assert_eq!(err.code_point, 'あ' as u32);
    println!("Validation error: {}", err);

    // --- Sizes ---

    println!("Latin: {} chars", latin.codepoints().len());
    println!("Katakana: {} chars", katakana.codepoints().len());
    println!("Full: {} chars", full.codepoints().len());

    println!("All tests passed!");
}
