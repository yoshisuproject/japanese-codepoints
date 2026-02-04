# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-05

### Added

- `CodePoints::from_slice()` - zero-copy construction from static data arrays
- `CodePoints::contains_char()` - single-character membership test
- `CodePoints::validate()` - returns structured `ValidationError` with detailed information
- `validate_all_in_any()` - multi-set validation with per-character error detail
- `cached()` method added to all leaf character sets (`LatinLetters`, `GreekLetters`, `CyrillicLetters`, `SpecialChars`, `BoxDrawingChars`)
- `validate()` method added to `JisX0208Kanji` and `JisX0213Kanji`
- `Default` implementation for all character set types
- `ValidationError` struct with fields `code_point`, `position`, and `message`
- Internal `charset!` macro to eliminate duplicate struct definitions

### Changed

- **BREAKING**: `JisX0208Kanji.all` field is now private; use `codepoints()` accessor method
- **BREAKING**: `contains_all_in_any` is now a standalone function with `&[&CodePoints]` signature (previously a method)
- **BREAKING**: Validation macros now return `Result<(), ValidationError>` instead of `Result<(), String>`
- `validate_codepoints!` and `validate_codepoints_advanced!` macros simplified; removed redundant shortcut patterns
- Feature-gated validation macros (`validate_hiragana!`, etc.) now borrow cached instances
- Composite character set types now combine sub-tables via `HashSet` instead of compile-time generated files
- Criterion benchmark dependency updated from 0.7 to 0.8

### Removed

- **BREAKING**: `build.rs` compile-time code generation (fragile text-parser for `ALL_*` arrays)
- **BREAKING**: `include!()`-based constant declarations
- Stale Windows path comments in data files

### Fixed

- Fixed `iter()` lifetime warning by adding explicit `'_` annotation
- Fixed `all_excluded()` documentation to clarify that order IS preserved (first-occurrence), not random

### Improved

- Complete README rewrite with:
  - Badges (MSRV, dependency status)
  - Performance comparison table
  - Architecture diagram
  - Java vs Rust feature comparison
  - Common pitfalls section
- All 7 examples improved with better formatting and `cached()` usage demonstrations
- Benchmarks completely reorganized with Criterion groups and throughput measurement

## [0.1.0] - 2025-07-26

### Added

- Initial release
- Rust port of Java library `terasoluna-gfw-codepoints`
- Core `CodePoints` struct wrapping `HashSet<u32>`
- Validation methods: `contains()`, `first_excluded()`, `first_excluded_with_position()`, `all_excluded()`
- Set operations: `union()`, `intersection()`, `difference()`, `symmetric_difference()`
- ASCII support: control characters, printable characters, CRLF
- JIS X 0201 support: Half-width Katakana (63 chars) + Latin letters (95 chars)
- JIS X 0208 support: Hiragana, Katakana, Latin, Greek, Cyrillic, special chars, box-drawing chars
- JIS X 0208 Kanji: 6,355 characters (Level 1 & 2)
- JIS X 0213 Kanji: 10,050 characters (Level 1-4)
- Validation macros: `validate_codepoints!`, `validate_codepoints_advanced!`
- Feature-gated convenience macros: `validate_hiragana!`, `validate_katakana!`, etc.
- Static caching via `std::sync::OnceLock` for zero-cost repeated access

[0.2.0]: https://github.com/yoshisuproject/japanese-codepoints/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yoshisuproject/japanese-codepoints/releases/tag/v0.1.0
