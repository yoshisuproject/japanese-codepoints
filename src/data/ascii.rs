//! ASCII character code points
//!
//! This module contains ASCII control characters and printable characters.

/// ASCII control characters (0x0000-0x001F, 0x007F)
pub const CONTROL_CHARS: &[u32] = &[
    0x0000, // NULL
    0x0001, // START OF HEADING
    0x0002, // START OF TEXT
    0x0003, // END OF TEXT
    0x0004, // END OF TRANSMISSION
    0x0005, // ENQUIRY
    0x0006, // ACKNOWLEDGE
    0x0007, // BELL
    0x0008, // BACKSPACE
    0x0009, // HORIZONTAL TABULATION
    0x000A, // LINE FEED
    0x000B, // VERTICAL TABULATION
    0x000C, // FORM FEED
    0x000D, // CARRIAGE RETURN
    0x000E, // SHIFT OUT
    0x000F, // SHIFT IN
    0x0010, // DATA LINK ESCAPE
    0x0011, // DEVICE CONTROL ONE
    0x0012, // DEVICE CONTROL TWO
    0x0013, // DEVICE CONTROL THREE
    0x0014, // DEVICE CONTROL FOUR
    0x0015, // NEGATIVE ACKNOWLEDGE
    0x0016, // SYNCHRONOUS IDLE
    0x0017, // END OF TRANSMISSION BLOCK
    0x0018, // CANCEL
    0x0019, // END OF MEDIUM
    0x001A, // SUBSTITUTE
    0x001B, // ESCAPE
    0x001C, // FILE SEPARATOR
    0x001D, // GROUP SEPARATOR
    0x001E, // RECORD SEPARATOR
    0x001F, // UNIT SEPARATOR
    0x007F, // DELETE
];

/// ASCII printable characters (0x0020-0x007E)
pub const PRINTABLE_CHARS: &[u32] = &[
    0x0020, // SPACE
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
    0x005C, // \
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
    0x007E, // ~
];

/// CRLF characters
pub const CRLF_CHARS: &[u32] = &[
    0x000A, // LINE FEED
    0x000D, // CARRIAGE RETURN
];

/// All ASCII characters (control + printable + CRLF)
pub const ALL_ASCII: &[u32] = include!(concat!(env!("OUT_DIR"), "/ascii_all.rs"));
