//! Data modules for character code points
//!
//! This module contains the actual character data organized by standards.

pub mod ascii;
pub mod jisx0201;
pub mod jisx0208;
pub mod jisx0208kanji;
pub mod jisx0213kanji;

#[cfg(test)]
mod tests;
