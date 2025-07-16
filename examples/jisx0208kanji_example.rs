use japanese_codepoints::jisx0208kanji::JisX0208Kanji;

fn main() {
    println!("=== JIS X 0208 Kanji Character Set Example ===\n");

    // Create JIS X 0208 kanji character set instance
    let kanji = JisX0208Kanji::new();

    println!("Total kanji characters: {}", kanji.codepoints_vec().len());
    println!("Expected count: 6355");
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

    // Test some Level 2 kanji
    let test_level2_kanji = vec!["堯", "槇", "遙", "瑤", "凜", "熙"];

    println!("Testing Level 2 kanji:");
    for kanji_char in test_level2_kanji {
        let contains = kanji.contains(kanji_char);
        println!("  {}: {}", kanji_char, if contains { "✓" } else { "✗" });
    }
    println!();

    // Other tests
    println!("Testing consist_of effect:");
    println!(
        "  '亜愛安以伊位一乙王黄': {}",
        kanji.contains("亜愛安以伊位一乙王黄")
    );
    println!("  'ABC123': {}", kanji.contains("ABC123"));
    println!("  '亜ABC愛': {}", kanji.contains("亜ABC愛"));
    println!();

    // Print first few codepoints
    let codepoints = kanji.codepoints_vec();
    println!("First 10 codepoints: {:?}", &codepoints[..10]);
}
