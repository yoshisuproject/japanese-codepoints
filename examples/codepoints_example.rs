//! Core CodePoints API example.
//!
//! Run: `cargo run --example codepoints_example`

use japanese_codepoints::{contains_all_in_any, validation::validate_all_in_any, CodePoints};

fn main() {
    // --- Construction ---

    let from_vec = CodePoints::new(vec![0x3042, 0x3044, 0x3046]); // あ い う
    let from_slice = CodePoints::from_slice(&[0x3042, 0x3044]);
    let from_str = CodePoints::from_string("あいう");

    assert_eq!(from_vec.len(), 3);
    assert_eq!(from_slice.len(), 2);
    assert_eq!(from_str.len(), 3);

    println!("Construction OK");

    // --- Membership ---

    let cp = CodePoints::from_slice(&[0x3042, 0x3044]); // あ い

    assert!(cp.contains("あ"));
    assert!(cp.contains("あい"));
    assert!(!cp.contains("う"));
    assert!(cp.contains("")); // empty is always valid

    assert!(cp.contains_char('あ'));
    assert!(!cp.contains_char('う'));

    println!("Membership OK");

    // --- Exclusion queries ---

    assert_eq!(cp.first_excluded("あいう"), Some(0x3046)); // う
    assert_eq!(cp.first_excluded_with_position("あいう"), Some((0x3046, 2)));

    let excluded = cp.all_excluded("あいうえお");
    assert_eq!(excluded, vec![0x3046, 0x3048, 0x304A]); // う え お

    println!("Exclusion queries OK");

    // --- Validation ---

    assert!(cp.validate("あい").is_ok());

    let err = cp.validate("あいう").unwrap_err();
    assert_eq!(err.code_point, 0x3046);
    assert_eq!(err.position, 2);
    println!("Validation error: {}", err);

    // --- Set operations ---

    let a = CodePoints::from_slice(&[0x3042, 0x3044]); // あ い
    let b = CodePoints::from_slice(&[0x3044, 0x3046]); // い う

    let union = a.union(&b);
    assert!(union.contains("あいう"));
    assert_eq!(union.len(), 3);

    let intersection = a.intersection(&b);
    assert!(intersection.contains("い"));
    assert_eq!(intersection.len(), 1);

    let diff = a.difference(&b);
    assert!(diff.contains("あ"));
    assert!(!diff.contains("い"));

    let sym_diff = a.symmetric_difference(&b);
    assert!(sym_diff.contains("あ"));
    assert!(sym_diff.contains("う"));
    assert!(!sym_diff.contains("い"));

    assert!(a.is_subset_of(&union));
    assert!(union.is_superset_of(&a));

    println!("Set operations OK");

    // --- Multi-set validation ---

    let hiragana = CodePoints::from_slice(&[0x3042, 0x3044, 0x3046]);
    let katakana = CodePoints::from_slice(&[0x30A2, 0x30A4, 0x30A6]);

    assert!(contains_all_in_any("あア", &[&hiragana, &katakana]));
    assert!(!contains_all_in_any("あx", &[&hiragana, &katakana]));

    assert!(validate_all_in_any("あア", &[&hiragana, &katakana]).is_ok());

    let err = validate_all_in_any("あxア", &[&hiragana, &katakana]).unwrap_err();
    assert_eq!(err.code_point, 'x' as u32);
    println!("Multi-set error: {}", err);

    println!("All tests passed!");
}
