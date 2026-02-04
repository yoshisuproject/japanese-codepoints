//! ASCII character set example.
//!
//! Run: `cargo run --example ascii_example`

use japanese_codepoints::CodePoints;

fn main() {
    // Cached references (recommended for production use)
    let control = CodePoints::ascii_control_cached();
    let printable = CodePoints::ascii_printable_cached();
    let all = CodePoints::ascii_all_cached();

    // --- Membership tests ---

    assert!(control.contains("\n\r\t"));
    assert!(!control.contains("A"));

    assert!(printable.contains("Hello!"));
    assert!(!printable.contains("Hello\n"));

    assert!(all.contains("Hello\n"));
    assert!(!all.contains("あ"));

    println!("Membership tests passed");

    // --- Single character checks ---

    assert!(control.contains_char('\n'));
    assert!(!control.contains_char('A'));
    assert!(printable.contains_char(' '));
    assert!(!printable.contains_char('\t'));

    println!("Single char tests passed");

    // --- Validation ---

    assert!(printable.validate("Hello").is_ok());

    let err = printable.validate("Hello\n").unwrap_err();
    assert_eq!(err.code_point, '\n' as u32);
    assert_eq!(err.position, 5);
    println!("Validation error: {}", err);

    // --- Exclusion queries ---

    assert_eq!(printable.first_excluded("Hello\n"), Some('\n' as u32));
    assert_eq!(
        printable.first_excluded_with_position("Hello\n"),
        Some(('\n' as u32, 5))
    );

    let excluded = printable.all_excluded("Hi\n世界\0");
    assert_eq!(excluded.len(), 4); // \n, 世, 界, \0
    println!("Excluded: {:?}", excluded);

    // --- Set sizes ---

    println!("Control: {} chars", control.len());
    println!("Printable: {} chars", printable.len());
    println!("All: {} chars", all.len());

    // --- Set operations ---

    let union = control.union(printable);
    assert_eq!(union.len(), all.len());

    let intersection = control.intersection(printable);
    assert!(intersection.is_empty());

    println!("All tests passed!");
}
