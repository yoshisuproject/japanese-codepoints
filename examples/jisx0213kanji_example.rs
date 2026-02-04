//! JIS X 0213 Kanji example.
//!
//! Contains 10,050 kanji (superset of JIS X 0208):
//! - Level 1: 2,965 chars (same as JIS X 0208)
//! - Level 2: 3,390 chars (same as JIS X 0208)
//! - Level 3: 1,259 chars (new in 0213)
//! - Level 4: 2,436 chars (new in 0213)
//!
//! Run: `cargo run --example jisx0213kanji_example --features codepoints-jisx0213kanji`

use japanese_codepoints::jisx0213kanji::JisX0213Kanji;

fn main() {
    let kanji = JisX0213Kanji::cached();

    // --- Basic checks ---

    assert_eq!(kanji.codepoints().len(), 10050);
    println!("Total: {} kanji", kanji.codepoints().len());

    // --- Level 1 & 2 (inherited from JIS X 0208) ---

    assert!(kanji.contains("亜愛安"));
    assert!(kanji.contains("日本語"));
    assert!(kanji.contains("漢字"));

    println!("Level 1 & 2 OK");

    // --- Level 3 & 4 (unique to JIS X 0213) ---

    // These characters are NOT in JIS X 0208
    assert!(kanji.contains("㐂")); // U+3402
    assert!(kanji.contains("㐆")); // U+3406
    assert!(kanji.contains("㐬")); // U+342C
    assert!(kanji.contains("龢")); // U+9FA2

    println!("Level 3 & 4 OK");

    // --- Non-kanji rejected ---

    assert!(!kanji.contains("ABC"));
    assert!(!kanji.contains("あいう"));
    assert!(!kanji.contains("アイウ"));
    assert!(!kanji.contains("123"));

    println!("Non-kanji rejection OK");

    // --- Validation ---

    assert!(kanji.validate("亜愛㐂龢").is_ok());

    let err = kanji.validate("亜x愛").unwrap_err();
    assert_eq!(err.code_point, 'x' as u32);
    assert_eq!(err.position, 1);
    println!("Validation error: {}", err);

    // --- Comparison with JIS X 0208 (if feature enabled) ---

    #[cfg(feature = "codepoints-jisx0208kanji")]
    {
        use japanese_codepoints::jisx0208kanji::JisX0208Kanji;

        let kanji_0208 = JisX0208Kanji::cached();

        let diff = kanji.codepoints().len() - kanji_0208.codepoints().len();
        assert_eq!(diff, 3695);
        println!("JIS X 0213 has {} more kanji than JIS X 0208", diff);

        // 㐂 is in 0213 but NOT in 0208
        assert!(!kanji_0208.contains("㐂"));
        assert!(kanji.contains("㐂"));
        println!(
            "㐂: in 0208={}, in 0213={}",
            kanji_0208.contains("㐂"),
            kanji.contains("㐂")
        );
    }

    println!("All tests passed!");
}
