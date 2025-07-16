use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    generate_ascii_merged(&out_dir);

    generate_jisx0201_merged(&out_dir);

    generate_jisx0208_merged(&out_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/data/ascii.rs");
    println!("cargo:rerun-if-changed=src/data/jisx0201.rs");
    println!("cargo:rerun-if-changed=src/data/jisx0208.rs");
}

fn generate_ascii_merged(out_dir: &str) {
    let dest_path = Path::new(out_dir).join("ascii_all.rs");
    let ascii_content = fs::read_to_string("src/data/ascii.rs").unwrap();
    let control_chars = parse_array(&ascii_content, "CONTROL_CHARS");
    let printable_chars = parse_array(&ascii_content, "PRINTABLE_CHARS");
    let crlf_chars = parse_array(&ascii_content, "CRLF_CHARS");
    let mut all_ascii = Vec::new();
    all_ascii.extend_from_slice(&control_chars);
    all_ascii.extend_from_slice(&printable_chars);
    all_ascii.extend_from_slice(&crlf_chars);
    let code = format!("&{all_ascii:?}");
    fs::write(&dest_path, code).unwrap();
}

fn generate_jisx0201_merged(out_dir: &str) {
    let dest_path = Path::new(out_dir).join("jisx0201_all.rs");
    let content = fs::read_to_string("src/data/jisx0201.rs").unwrap();
    let latin_letters = parse_array(&content, "LATIN_LETTERS");
    let katakana = parse_array(&content, "KATAKANA");
    let mut all_jisx0201 = Vec::new();
    all_jisx0201.extend_from_slice(&latin_letters);
    all_jisx0201.extend_from_slice(&katakana);
    let code = format!("&{all_jisx0201:?}");
    fs::write(&dest_path, code).unwrap();
}

fn generate_jisx0208_merged(out_dir: &str) {
    let dest_path = Path::new(out_dir).join("jisx0208_all.rs");
    let content = fs::read_to_string("src/data/jisx0208.rs").unwrap();
    let hiragana = parse_array(&content, "HIRAGANA");
    let katakana = parse_array(&content, "KATAKANA");
    let latin_letters = parse_array(&content, "LATIN_LETTERS");
    let greek_letters = parse_array(&content, "GREEK_LETTERS");
    let cyrillic_letters = parse_array(&content, "CYRILLIC_LETTERS");
    let special_chars = parse_array(&content, "SPECIAL_CHARS");
    let box_drawing_chars = parse_array(&content, "BOX_DRAWING_CHARS");
    let mut all_jisx0208 = Vec::new();
    all_jisx0208.extend_from_slice(&hiragana);
    all_jisx0208.extend_from_slice(&katakana);
    all_jisx0208.extend_from_slice(&latin_letters);
    all_jisx0208.extend_from_slice(&greek_letters);
    all_jisx0208.extend_from_slice(&cyrillic_letters);
    all_jisx0208.extend_from_slice(&special_chars);
    all_jisx0208.extend_from_slice(&box_drawing_chars);
    let code = format!("&{all_jisx0208:?}");
    fs::write(&dest_path, code).unwrap();
}

fn parse_array(content: &str, array_name: &str) -> Vec<u32> {
    let pattern = format!("pub const {array_name}: &[u32] = &[");
    if let Some(start) = content.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = content[start..].find("];") {
            let array_content = &content[start..start + end];
            let mut result = Vec::new();
            for line in array_content.lines() {
                // start with 0x
                if let Some(idx) = line.find("0x") {
                    let hex: String = line[idx..]
                        .chars()
                        .take_while(|c| c.is_ascii_hexdigit() || *c == 'x')
                        .collect();
                    if hex.len() > 2 {
                        if let Ok(value) = u32::from_str_radix(&hex[2..], 16) {
                            result.push(value);
                        }
                    }
                }
            }
            return result;
        }
    }
    Vec::new()
}