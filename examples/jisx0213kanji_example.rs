use japanese_codepoints::jisx0213kanji::JisX0213Kanji;

fn main() {
    println!("=== JIS X 0213 Kanji Character Set Example ===\n");

    // Create JIS X 0213 kanji character set instance
    let kanji = JisX0213Kanji::new();

    println!("Total kanji characters: {}", kanji.codepoints_vec().len());
    println!("Expected count: 10050");
    println!();

    // Test some common kanji
    let test_kanji = vec![
        "亜", "愛", "安", "以", "伊", "位", "一", "乙", "王", "黄", "漢", "字", "日", "本", "語",
        "学", "習", "読", "書", "写",
    ];

    println!("Testing common kanji:");
    for kanji_char in test_kanji {
        let contains = kanji.contains(kanji_char);
        println!("  {}: {}", kanji_char, if contains { "✓" } else { "✗" });
    }
    println!();

    // Test some Level 3 and 4 kanji
    let test_level3_4_kanji = vec!["堯", "槇", "遙", "瑤", "凜", "熙"];

    println!("Testing Level 3 and 4 kanji:");
    for kanji_char in test_level3_4_kanji {
        let contains = kanji.contains(kanji_char);
        println!("  {}: {}", kanji_char, if contains { "✓" } else { "✗" });
    }
    println!();

    // Test mixed strings
    let test_strings = vec![
        "亜愛安以伊位一乙王黄",
        "漢字日本語",
        "堯槇遙瑤凜熙",
        "亜A愛", // Contains non-kanji
        "123",   // Numbers
        "",      // Empty string
    ];

    println!("Testing mixed strings:");
    for s in test_strings {
        let contains = kanji.contains(s);
        println!("  \"{}\": {}", s, if contains { "✓" } else { "✗" });
    }
    println!();

    // Performance test
    println!("Performance test:");
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        kanji.contains("亜愛安以伊位一乙王黄漢字日本語");
    }
    let duration = start.elapsed();
    println!("  1000 contains calls took: {:?}", duration);
    println!("  Average per call: {:?}", duration / 1000);
}
