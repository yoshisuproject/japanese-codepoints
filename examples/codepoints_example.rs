use japanese_codepoints::CodePoints;

fn main() {
    // Create a set containing only "あ" and "い"
    let cp = CodePoints::new(vec![0x3042, 0x3044]); // あ, い

    println!("Contains 'あ': {}", cp.contains("あ"));
    println!("Contains 'あい': {}", cp.contains("あい"));
    println!("Contains 'う': {}", cp.contains("う"));

    // Check the first illegal character
    println!(
        "First excluded in 'あいう': {:?}",
        cp.first_excluded("あいう")
    );

    // Get all illegal characters
    println!(
        "All excluded in 'あいうえ': {:?}",
        cp.all_excluded("あいうえ")
    );

    // Set operations
    let cp2 = CodePoints::new(vec![0x3044, 0x3046]); // い, う
    let union = cp.union(&cp2);
    println!("Union contains 'あいう': {}", union.contains("あいう"));
    let intersection = cp.intersection(&cp2);
    println!(
        "Intersection contains 'い': {}",
        intersection.contains("い")
    );
    let difference = cp.difference(&cp2);
    println!("Difference contains 'あ': {}", difference.contains("あ"));
}
