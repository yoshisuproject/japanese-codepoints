use japanese_codepoints::CodePoints;

fn main() {
    let ascii_control = CodePoints::ascii_control();
    let ascii_printable = CodePoints::ascii_printable();
    let ascii_all = CodePoints::ascii_all();

    println!(
        "ASCII Control contains '\\n': {}",
        ascii_control.contains("\n")
    );
    println!(
        "ASCII Printable contains 'Hello!': {}",
        ascii_printable.contains("Hello!")
    );
    println!(
        "ASCII All contains 'Hello\\n': {}",
        ascii_all.contains("Hello\n")
    );

    // Check mixed string
    println!(
        "ASCII Printable contains 'Helloあ': {}",
        ascii_printable.contains("Helloあ")
    );
    println!(
        "ASCII All contains 'Helloあ': {}",
        ascii_all.contains("Helloあ")
    );
}
