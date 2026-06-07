//! Data modules for character code points
//!
//! This module contains the actual character data organized by standards.

pub mod ascii;

#[cfg(feature = "codepoints-jisx0201")]
pub mod jisx0201;

#[cfg(feature = "codepoints-jisx0208")]
pub mod jisx0208;

#[cfg(feature = "codepoints-jisx0208kanji")]
pub mod jisx0208kanji;

#[cfg(feature = "codepoints-jisx0213kanji")]
pub mod jisx0213kanji;

#[cfg(test)]
mod tests;
