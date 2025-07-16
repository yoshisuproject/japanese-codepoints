//! JIS X 0201 character code points
//!
//! This module contains JIS X 0201 Latin letters and Katakana (halfwidth kana) characters.

/// JIS X 0201 Latin letters
///
/// Includes ASCII printable characters plus some special characters:
/// - 0x0020-0x007E: Standard ASCII printable characters
/// - 0x00A5: Yen sign (¥)
/// - 0x203E: Overline (‾)
pub const LATIN_LETTERS: &[u32] = &[
    0x0020, //
    0x0021, // !
    0x0022, // "
    0x0023, // #
    0x0024, // $
    0x0025, // %
    0x0026, // &
    0x0027, // '
    0x0028, // (
    0x0029, // )
    0x002A, // *
    0x002B, // +
    0x002C, // ,
    0x002D, // -
    0x002E, // .
    0x002F, // /
    0x0030, // 0
    0x0031, // 1
    0x0032, // 2
    0x0033, // 3
    0x0034, // 4
    0x0035, // 5
    0x0036, // 6
    0x0037, // 7
    0x0038, // 8
    0x0039, // 9
    0x003A, // :
    0x003B, // ;
    0x003C, // <
    0x003D, // =
    0x003E, // >
    0x003F, // ?
    0x0040, // @
    0x0041, // A
    0x0042, // B
    0x0043, // C
    0x0044, // D
    0x0045, // E
    0x0046, // F
    0x0047, // G
    0x0048, // H
    0x0049, // I
    0x004A, // J
    0x004B, // K
    0x004C, // L
    0x004D, // M
    0x004E, // N
    0x004F, // O
    0x0050, // P
    0x0051, // Q
    0x0052, // R
    0x0053, // S
    0x0054, // T
    0x0055, // U
    0x0056, // V
    0x0057, // W
    0x0058, // X
    0x0059, // Y
    0x005A, // Z
    0x005B, // [
    0x00A5, // ¥
    0x005D, // ]
    0x005E, // ^
    0x005F, // _
    0x0060, // `
    0x0061, // a
    0x0062, // b
    0x0063, // c
    0x0064, // d
    0x0065, // e
    0x0066, // f
    0x0067, // g
    0x0068, // h
    0x0069, // i
    0x006A, // j
    0x006B, // k
    0x006C, // l
    0x006D, // m
    0x006E, // n
    0x006F, // o
    0x0070, // p
    0x0071, // q
    0x0072, // r
    0x0073, // s
    0x0074, // t
    0x0075, // u
    0x0076, // v
    0x0077, // w
    0x0078, // x
    0x0079, // y
    0x007A, // z
    0x007B, // {
    0x007C, // |
    0x007D, // }
    0x203E, // ‾
];

/// JIS X 0201 Katakana (halfwidth kana)
///
/// Includes all halfwidth katakana characters from 0xFF61 to 0xFF9F:
/// - 0xFF61-0xFF65: Special characters (｡｢｣､･)
/// - 0xFF66-0xFF9F: Halfwidth katakana characters
pub const KATAKANA: &[u32] = &[
    0xFF61, // ｡
    0xFF62, // ｢
    0xFF63, // ｣
    0xFF64, // ､
    0xFF65, // ･
    0xFF66, // ｦ
    0xFF67, // ｧ
    0xFF68, // ｨ
    0xFF69, // ｩ
    0xFF6A, // ｪ
    0xFF6B, // ｫ
    0xFF6C, // ｬ
    0xFF6D, // ｭ
    0xFF6E, // ｮ
    0xFF6F, // ｯ
    0xFF70, // ｰ
    0xFF71, // ｱ
    0xFF72, // ｲ
    0xFF73, // ｳ
    0xFF74, // ｴ
    0xFF75, // ｵ
    0xFF76, // ｶ
    0xFF77, // ｷ
    0xFF78, // ｸ
    0xFF79, // ｹ
    0xFF7A, // ｺ
    0xFF7B, // ｻ
    0xFF7C, // ｼ
    0xFF7D, // ｽ
    0xFF7E, // ｾ
    0xFF7F, // ｿ
    0xFF80, // ﾀ
    0xFF81, // ﾁ
    0xFF82, // ﾂ
    0xFF83, // ﾃ
    0xFF84, // ﾄ
    0xFF85, // ﾅ
    0xFF86, // ﾆ
    0xFF87, // ﾇ
    0xFF88, // ﾈ
    0xFF89, // ﾉ
    0xFF8A, // ﾊ
    0xFF8B, // ﾋ
    0xFF8C, // ﾌ
    0xFF8D, // ﾍ
    0xFF8E, // ﾎ
    0xFF8F, // ﾏ
    0xFF90, // ﾐ
    0xFF91, // ﾑ
    0xFF92, // ﾒ
    0xFF93, // ﾓ
    0xFF94, // ﾔ
    0xFF95, // ﾕ
    0xFF96, // ﾖ
    0xFF97, // ﾗ
    0xFF98, // ﾘ
    0xFF99, // ﾙ
    0xFF9A, // ﾚ
    0xFF9B, // ﾛ
    0xFF9C, // ﾜ
    0xFF9D, // ﾝ
    0xFF9E, // ﾞ
    0xFF9F, // ﾟ
];

/// All JIS X 0201 characters (Latin letters + Katakana)
pub const ALL_JISX0201: &[u32] = include!(concat!(env!("OUT_DIR"), "/jisx0201_all.rs"));
