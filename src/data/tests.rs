#[cfg(test)]
mod tests {
    use crate::codepoints::CodePoints;
    use crate::data::jisx0201::{
        KATAKANA as JISX0201_KATAKANA, LATIN_LETTERS as JISX0201_LATIN_LETTERS,
    };
    use crate::data::jisx0208::{
        BOX_DRAWING_CHARS as JISX0208_BOX_DRAWING_CHARS,
        CYRILLIC_LETTERS as JISX0208_CYRILLIC_LETTERS, GREEK_LETTERS as JISX0208_GREEK_LETTERS,
        HIRAGANA as JISX0208_HIRAGANA, KATAKANA as JISX0208_KATAKANA,
        LATIN_LETTERS as JISX0208_LATIN_LETTERS, SPECIAL_CHARS as JISX0208_SPECIAL_CHARS,
    };
    use crate::data::jisx0208kanji::JISX0208_CHARS as JISX0208_KANJI;
    use crate::data::jisx0213kanji::JISX0213_KANJI;

    #[test]
    fn test_ascii_printable() {
        let cp = CodePoints::ascii_printable();
        assert!(cp.contains("Hello World 123!@#"));
        assert!(!cp.contains("Hello World\n")); // \n is a control character
        assert_eq!(cp.first_excluded("a-b-c-あ"), Some(0x3042)); // あ
    }

    #[test]
    fn test_ascii_control() {
        let cp = CodePoints::ascii_control();
        assert!(cp.contains("\n\r\t"));
        assert!(!cp.contains("a\n\r\t"));
        assert_eq!(cp.first_excluded("\n\rA\t"), Some(0x0041)); // A
    }

    #[test]
    fn test_crlf() {
        let cp = CodePoints::crlf();
        assert!(cp.contains("\r\n"));
        assert!(!cp.contains("\r\n\t"));
        assert_eq!(cp.first_excluded("\r\n\t"), Some(0x0009)); // \t
    }

    #[test]
    fn test_jisx0201_katakana() {
        let cp = CodePoints::new(JISX0201_KATAKANA.to_vec());
        assert!(cp.contains("｡｢｣､･ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝﾞﾟ"));
        assert!(!cp.contains("アイウエオ")); // Full-width katakana
        assert_eq!(cp.first_excluded("ﾊﾝｶｸA"), Some(0x0041)); // A
    }

    #[test]
    fn test_jisx0201_latin_letters() {
        let cp = CodePoints::new(JISX0201_LATIN_LETTERS.to_vec());
        assert!(cp.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(cp.contains("¥‾")); // Special characters for JIS X 0201 variant
        assert!(!cp.contains("Hello\\World")); // Backslash is different in some variants
        assert_eq!(cp.first_excluded("abc\\"), Some(0x005C)); // \
    }

    #[test]
    fn test_jisx0208_hiragana() {
        let cp = CodePoints::new(JISX0208_HIRAGANA.to_vec());
        assert!(cp.contains("あいうえお"));
        assert!(cp.contains("がぎぐげご"));
        assert!(!cp.contains("アイウエオ")); // Katakana
        assert!(!cp.contains("漢字")); // Kanji
        assert_eq!(cp.first_excluded("ひらがなA"), Some(0x0041)); // A
    }

    #[test]
    fn test_jisx0208_katakana() {
        let cp = CodePoints::new(JISX0208_KATAKANA.to_vec());
        assert!(cp.contains("アイウエオ"));
        assert!(cp.contains("ガギグゲゴ"));
        assert!(!cp.contains("あいうえお")); // Hiragana
        assert_eq!(cp.first_excluded("カタカナa"), Some(0x0061)); // a
    }

    #[test]
    fn test_jisx0208_latin_letters() {
        let cp = CodePoints::new(JISX0208_LATIN_LETTERS.to_vec());
        assert!(cp.contains("ＡＢＣＤＥＦＧ"));
        assert!(cp.contains("ａｂｃｄｅｆｇ"));
        assert!(cp.contains("０１２３４５６７８９"));
        assert!(!cp.contains("ABCDEFG")); // Half-width
        assert_eq!(cp.first_excluded("ＺＥＮＫＡＫＵ1"), Some(0x0031)); // 1
    }

    #[test]
    fn test_jisx0208_greek_letters() {
        let cp = CodePoints::new(JISX0208_GREEK_LETTERS.to_vec());
        assert!(cp.contains("ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"));
        assert!(cp.contains("αβγδεζηθικλμνξοπρστυφχψω"));
        assert!(!cp.contains("ABC"));
    }

    #[test]
    fn test_jisx0208_cyrillic_letters() {
        let cp = CodePoints::new(JISX0208_CYRILLIC_LETTERS.to_vec());
        assert!(cp.contains("АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"));
        assert!(cp.contains("абвгдеёжзийклмнопрстуфхцчшщъыьэюя"));
        assert!(!cp.contains("ABC"));
    }

    #[test]
    fn test_jisx0208_box_drawing_chars() {
        let cp = CodePoints::new(JISX0208_BOX_DRAWING_CHARS.to_vec());
        assert!(cp.contains("─│┌┐┘└├┬┤┴┼"));
        assert!(!cp.contains("-|"));
    }

    #[test]
    fn test_jisx0208_special_chars() {
        let cp = CodePoints::new(JISX0208_SPECIAL_CHARS.to_vec());
        assert!(cp.contains("、。，．・：；？！"));
        assert!(!cp.contains("abc"));
    }

    #[test]
    fn test_jisx0208_kanji() {
        let cp = CodePoints::new(JISX0208_KANJI.to_vec());
        assert!(cp.contains(
            "亜唖娃阿哀愛挨姶逢葵茜穐悪握渥旭葦芦鯵梓圧斡扱宛姐虻飴絢綾鮎或粟袷安庵按暗案闇鞍杏"
        )); // Level 1
        assert!(cp.contains("弌丐丕个丱丶丼丿乂乖乘亂亅豫亊舒弍于亞亟亠亢亰亳亶从仍仄仆仂仗")); // Level 2
        assert!(!cp.contains("a"));
        assert!(!cp.contains("あ"));
        assert!(!cp.contains("ア"));
    }

    #[test]
    fn test_jisx0213_kanji() {
        let cp = CodePoints::new(JISX0213_KANJI.to_vec());
        // Level 1
        assert!(cp.contains(
            "亜唖娃阿哀愛挨姶逢葵茜穐悪握渥旭葦芦鯵梓圧斡扱宛姐虻飴絢綾鮎或粟袷安庵按暗案闇鞍杏"
        ));
        // Level 2
        assert!(cp.contains("弌丐丕个丱丶丼丿乂乖乘亂亅豫亊舒弍于亞亟亠亢亰亳亶从仍仄仆仂仗"));
        // Test some Level 3 kanji (CJK Unified Ideographs Extension A)
        assert!(cp.contains("㐂㠯㒵"));
        // Test some Level 4 kanji (CJK Unified Ideographs Extension B)
        assert!(cp.contains("俱剝頰"));
        assert!(!cp.contains("a"));
        assert!(!cp.contains("あ"));
        assert!(!cp.contains("ア"));
    }
}
