//! JIS X 0208 Example
//!
//! This example demonstrates the JIS X 0208 character set functionality.
//! Run with: cargo run --example jisx0208_example --features codepoints-jisx0208

use japanese_codepoints::jisx0208::{
    BoxDrawingChars, CyrillicLetters, GreekLetters, Hiragana, JisX0208, Katakana, LatinLetters,
    SpecialChars,
};

fn main() {
    println!("=== JIS X 0208 Character Set Example ===\n");

    // Test Hiragana
    println!("1. Hiragana (ひらがな) Test:");
    let hiragana = Hiragana::new();

    let hiragana_texts = vec![
        "あいうえお",
        "かきくけこ",
        "さしすせそ",
        "たちつてと",
        "なにぬねの",
    ];

    for text in &hiragana_texts {
        let result = if hiragana.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Katakana
    println!("\n2. Katakana (カタカナ) Test:");
    let katakana = Katakana::new();

    let katakana_texts = vec![
        "アイウエオ",
        "カキクケコ",
        "サシスセソ",
        "タチツテト",
        "ナニヌネノ",
    ];

    for text in &katakana_texts {
        let result = if katakana.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Latin Letters
    println!("\n3. Latin Letters (fullwidth) Test:");
    let latin = LatinLetters::new();

    let latin_texts = vec![
        "ＡＢＣＤＥ",
        "ａｂｃｄｅ",
        "１２３４５",
        "ＦＧＨＩＪ",
        "ｆｇｈｉｊ",
    ];

    for text in &latin_texts {
        let result = if latin.contains(text) { "✓" } else { "✗" };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Greek Letters
    println!("\n4. Greek Letters Test:");
    let greek = GreekLetters::new();

    let greek_texts = vec!["ΑΒΓΔΕ", "αβγδε", "ΖΗΘΙΚ", "ζηθικ"];

    for text in &greek_texts {
        let result = if greek.contains(text) { "✓" } else { "✗" };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Cyrillic Letters
    println!("\n5. Cyrillic Letters Test:");
    let cyrillic = CyrillicLetters::new();

    let cyrillic_texts = vec!["АБВГД", "абвгд", "ЕЁЖЗИ", "еёжзи"];

    for text in &cyrillic_texts {
        let result = if cyrillic.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Special Characters
    println!("\n6. Special Characters Test:");
    let special = SpecialChars::new();

    let special_texts = vec!["、。", "☆★", "→←", "①②③", "〒※"];

    for text in &special_texts {
        let result = if special.contains(text) { "✓" } else { "✗" };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Box Drawing Characters
    println!("\n7. Box Drawing Characters Test:");
    let box_drawing = BoxDrawingChars::new();

    let box_texts = vec!["─│┌┐", "└┘├┤", "┬┴┼", "━┃┏┓"];

    for text in &box_texts {
        let result = if box_drawing.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test mixed text (should work with complete set)
    println!("\n8. Mixed Text Test (Complete JIS X 0208):");
    let jisx0208 = JisX0208::new();

    let mixed_texts = vec![
        "あいうえおアイウエオ",
        "ＡＢＣあいう",
        "ΑΒΓあいう",
        "АБВあいう",
        "、。☆★あいう",
        "─│┌┐あいう",
    ];

    for text in &mixed_texts {
        let result = if jisx0208.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test invalid characters
    println!("\n9. Invalid Characters Test:");
    let invalid_texts = vec![
        "漢字",           // Kanji not included in JIS X 0208 (non-kanji)
        "ｱｲｳｴｵ",          // Halfwidth katakana (JIS X 0201)
        "Hello",          // Halfwidth Latin
        "あいうえお漢字", // Mixed with kanji
    ];

    for text in &invalid_texts {
        let result = if jisx0208.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Show character counts
    println!("\n10. Character Counts:");
    println!("  Hiragana: {} characters", hiragana.codepoints().len());
    println!("  Katakana: {} characters", katakana.codepoints().len());
    println!("  Latin Letters: {} characters", latin.codepoints().len());
    println!("  Greek Letters: {} characters", greek.codepoints().len());
    println!(
        "  Cyrillic Letters: {} characters",
        cyrillic.codepoints().len()
    );
    println!(
        "  Special Characters: {} characters",
        special.codepoints().len()
    );
    println!(
        "  Box Drawing Characters: {} characters",
        box_drawing.codepoints().len()
    );
    println!(
        "  Total JIS X 0208: {} characters",
        jisx0208.codepoints().len()
    );

    // Show some sample code points
    println!("\n11. Sample Code Points:");
    println!(
        "  Hiragana (first 5): {:?}",
        &hiragana.codepoints().iter().take(5).collect::<Vec<_>>()
    );
    println!(
        "  Katakana (first 5): {:?}",
        &katakana.codepoints().iter().take(5).collect::<Vec<_>>()
    );
    println!(
        "  Latin (first 5): {:?}",
        &latin.codepoints().iter().take(5).collect::<Vec<_>>()
    );
}
