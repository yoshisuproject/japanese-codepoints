//! JIS X 0201 Example
//!
//! This example demonstrates the JIS X 0201 character set functionality.
//! Run with: cargo run --example jisx0201_example --features codepoints-jisx0201

use japanese_codepoints::jisx0201::{JisX0201, Katakana, LatinLetters};

fn main() {
    println!("=== JIS X 0201 Character Set Example ===\n");

    // Test Latin Letters
    println!("1. Latin Letters Test:");
    let latin = LatinLetters::new();

    let latin_texts = vec![
        "Hello World",
        "1234567890",
        "!@#$%^&*()",
        "¥", // Yen sign
        "‾", // Overline
    ];

    for text in &latin_texts {
        let result = if latin.contains(text) { "✓" } else { "✗" };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test Katakana
    println!("\n2. Katakana (Halfwidth) Test:");
    let katakana = Katakana::new();

    let katakana_texts = vec![
        "ｱｲｳｴｵ", // Basic katakana
        "ｶｷｸｹｺ", // Ka-line
        "ｻｼｽｾｿ", // Sa-line
        "｡｢｣､･", // Special characters
        "ﾞﾟ",      // Dakuten and handakuten
    ];

    for text in &katakana_texts {
        let result = if katakana.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test mixed text (should fail for individual sets)
    println!("\n3. Mixed Text Test:");
    let mixed_texts = vec!["Helloｱｲｳｴｵ", "ｱｲｳｴｵHello", "¥｡｢｣､･"];

    for text in &mixed_texts {
        let latin_result = if latin.contains(text) { "✓" } else { "✗" };
        let katakana_result = if katakana.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!(
            "  {} (Latin: {}, Katakana: {})",
            text, latin_result, katakana_result
        );
    }

    // Test complete JIS X 0201 set
    println!("\n4. Complete JIS X 0201 Test:");
    let jisx0201 = JisX0201::new();

    for text in &mixed_texts {
        let result = if jisx0201.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Test invalid characters
    println!("\n5. Invalid Characters Test:");
    let invalid_texts = vec![
        "あいうえお", // Fullwidth hiragana
        "アイウエオ", // Fullwidth katakana
        "漢字",       // Kanji
        "Hello世界",  // Mixed with Chinese
    ];

    for text in &invalid_texts {
        let result = if jisx0201.contains(text) {
            "✓"
        } else {
            "✗"
        };
        println!("  {} {} -> {}", result, text, text);
    }

    // Show code point counts
    println!("\n6. Character Counts:");
    println!("  Latin Letters: {} characters", latin.codepoints().len());
    println!("  Katakana: {} characters", katakana.codepoints().len());
    println!(
        "  Total JIS X 0201: {} characters",
        jisx0201.codepoints().len()
    );

    // Show some code points
    println!("\n7. Sample Code Points:");
    println!(
        "  Latin Letters (first 5): {:?}",
        &latin.codepoints().iter().take(5).collect::<Vec<_>>()
    );
    println!(
        "  Katakana (first 5): {:?}",
        &katakana.codepoints().iter().take(5).collect::<Vec<_>>()
    );
}
