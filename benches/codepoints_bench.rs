//! Criterion Benchmarks for japanese-codepoints
//!
//! Run all benchmarks:
//! ```bash
//! cargo bench --all-features
//! ```
//!
//! Run specific benchmark group:
//! ```bash
//! cargo bench --all-features -- core_ops
//! ```
//!
//! Generate HTML report:
//! ```bash
//! cargo bench --all-features -- --html target/criterion
//! ```

use std::hint::black_box;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
    SamplingMode, Throughput,
};
use japanese_codepoints::{contains_all_in_any, CodePoints};

// ============================================================================
// Core Operations Group
// ============================================================================

fn group_core_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("core_ops");

    let cp = CodePoints::new(vec![0x3041, 0x3042, 0x3043, 0x3044, 0x3045]);

    // Throughput: measure per-character performance
    group.throughput(Throughput::Bytes(15)); // "あいうえおかきくけこ" is 30 bytes

    bench_with_throughput(&mut group, "contains/valid", || {
        cp.contains(black_box("あいうえおかきくけこ"))
    });

    bench_with_throughput(&mut group, "contains/invalid_early", || {
        cp.contains(black_box("かきくけこあいうえお"))
    });

    bench_with_throughput(&mut group, "contains/invalid_late", || {
        cp.contains(black_box("あいうえおかきくけこさ"))
    });

    group.finish();
}

fn group_set_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_ops");

    let cp1 = CodePoints::new((0x3041..=0x3060).collect::<Vec<_>>());
    let cp2 = CodePoints::new((0x3050..=0x3080).collect::<Vec<_>>());

    group.bench_function("union", |b| b.iter(|| cp1.union(black_box(&cp2))));
    group.bench_function("intersection", |b| {
        b.iter(|| cp1.intersection(black_box(&cp2)))
    });
    group.bench_function("difference", |b| b.iter(|| cp1.difference(black_box(&cp2))));
    group.bench_function("symmetric_difference", |b| {
        b.iter(|| cp1.symmetric_difference(black_box(&cp2)))
    });
    group.bench_function("is_subset/true", |b| {
        let subset = CodePoints::new(vec![0x3041, 0x3042]);
        b.iter(|| subset.is_subset_of(black_box(&cp1)))
    });
    group.bench_function("is_subset/false", |b| {
        let not_subset = CodePoints::new(vec![0x3041, 0x9999]);
        b.iter(|| not_subset.is_subset_of(black_box(&cp1)))
    });

    group.finish();
}

fn group_exclusion_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("exclusion");

    let cp = CodePoints::new((0x3041..=0x3093).collect::<Vec<_>>()); // All hiragana

    // first_excluded variations
    group.bench_function("first_excluded/none", |b| {
        b.iter(|| cp.first_excluded(black_box("あいう")))
    });
    group.bench_function("first_excluded/early", |b| {
        b.iter(|| cp.first_excluded(black_box("かあいう")))
    });
    group.bench_function("first_excluded/late", |b| {
        b.iter(|| cp.first_excluded(black_box("あいうか")))
    });

    // first_excluded_with_position
    group.bench_function("first_excluded_with_position/none", |b| {
        b.iter(|| cp.first_excluded_with_position(black_box("あいう")))
    });
    group.bench_function("first_excluded_with_position/late", |b| {
        b.iter(|| cp.first_excluded_with_position(black_box("あいうえおか")))
    });

    // all_excluded
    group.bench_function("all_excluded/none", |b| {
        b.iter(|| cp.all_excluded(black_box("あいう")))
    });
    group.bench_function("all_excluded/single", |b| {
        b.iter(|| cp.all_excluded(black_box("あいうえ")))
    });
    group.bench_function("all_excluded/multiple", |b| {
        b.iter(|| cp.all_excluded(black_box("あえいおうか")))
    });

    group.finish();
}

fn group_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction");

    group.bench_function("from_vec/small", |b| {
        b.iter(|| CodePoints::new(black_box(vec![0x3041, 0x3042, 0x3043])))
    });
    group.bench_function("from_vec/large", |b| {
        let data: Vec<_> = (0x3041..=0x3093).collect();
        b.iter(|| CodePoints::new(black_box(data.clone())))
    });

    group.bench_function("from_slice", |b| {
        let slice: &[u32] = &[0x3041, 0x3042, 0x3043];
        b.iter(|| CodePoints::from_slice(black_box(slice)))
    });

    group.bench_function("from_string/short", |b| {
        b.iter(|| CodePoints::from_string(black_box("あいうえお")))
    });
    group.bench_function("from_string/long", |b| {
        b.iter(|| {
            CodePoints::from_string(black_box(
                "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめも",
            ))
        })
    });

    group.finish();
}

// ============================================================================
// ASCII Group
// ============================================================================

fn group_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("ascii");

    let printable = CodePoints::ascii_printable_cached();
    let control = CodePoints::ascii_control_cached();
    let all = CodePoints::ascii_all_cached();

    // Throughput based benchmarks
    let short_text = "Hello World";
    let long_text = "Hello World! This is a benchmark test with ASCII characters. ".repeat(10);
    let mixed_text = "Hello 世界! Test テスト";

    group.throughput(Throughput::Bytes(short_text.len() as u64));
    group.bench_function("printable/short", |b| {
        b.iter(|| printable.contains(black_box(short_text)))
    });

    group.throughput(Throughput::Bytes(long_text.len() as u64));
    group.bench_function("printable/long", |b| {
        b.iter(|| printable.contains(black_box(&long_text)))
    });

    group.throughput(Throughput::Bytes(mixed_text.len() as u64));
    group.bench_function("printable/mixed", |b| {
        b.iter(|| printable.contains(black_box(mixed_text)))
    });

    // First excluded
    group.bench_function("printable/first_excluded/valid", |b| {
        b.iter(|| printable.first_excluded(black_box("Hello World")))
    });
    group.bench_function("printable/first_excluded/invalid", |b| {
        b.iter(|| printable.first_excluded(black_box("Hello\nWorld")))
    });

    // Control chars
    group.bench_function("control/contains", |b| {
        b.iter(|| control.contains(black_box("\n\r\t")))
    });

    // ASCII all
    group.bench_function("all/contains", |b| {
        b.iter(|| all.contains(black_box("Hello\nWorld\t!")))
    });

    group.finish();
}

// ============================================================================
// Caching Group
// ============================================================================

fn group_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("caching");
    group.sampling_mode(SamplingMode::Flat); // More accurate for fast operations

    // Compare create vs cached
    group.bench_function("printable/create", |b| {
        b.iter(|| CodePoints::ascii_printable())
    });
    group.bench_function("printable/cached", |b| {
        b.iter(|| CodePoints::ascii_printable_cached())
    });

    group.bench_function("control/create", |b| b.iter(|| CodePoints::ascii_control()));
    group.bench_function("control/cached", |b| {
        b.iter(|| CodePoints::ascii_control_cached())
    });

    group.bench_function("all/create", |b| b.iter(|| CodePoints::ascii_all()));
    group.bench_function("all/cached", |b| b.iter(|| CodePoints::ascii_all_cached()));

    group.bench_function("crlf/create", |b| b.iter(|| CodePoints::crlf()));
    group.bench_function("crlf/cached", |b| b.iter(|| CodePoints::crlf_cached()));

    group.finish();
}

// ============================================================================
// Validation Group
// ============================================================================

fn group_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");

    let ascii = CodePoints::ascii_printable_cached();

    group.bench_function("validate/ok", |b| {
        b.iter(|| ascii.validate(black_box("Hello World")))
    });
    group.bench_function("validate/err_early", |b| {
        b.iter(|| ascii.validate(black_box("\nHello World")))
    });
    group.bench_function("validate/err_late", |b| {
        b.iter(|| ascii.validate(black_box("Hello World\n")))
    });

    // Multi-set validation
    let alpha = CodePoints::new((0x41..=0x5A).chain(0x61..=0x7A).collect::<Vec<_>>());
    let digits = CodePoints::new((0x30..=0x39).collect::<Vec<_>>());

    group.bench_function("validate_multi/ok", |b| {
        b.iter(|| contains_all_in_any(black_box("Hello123"), &[&alpha, &digits]))
    });
    group.bench_function("validate_multi/err", |b| {
        b.iter(|| contains_all_in_any(black_box("Hello 123"), &[&alpha, &digits]))
    });

    group.finish();
}

// ============================================================================
// JIS X 0201 Group
// ============================================================================

#[cfg(feature = "codepoints-jisx0201")]
fn group_jisx0201(c: &mut Criterion) {
    use japanese_codepoints::jisx0201::{JisX0201, Katakana, LatinLetters};

    let mut group = c.benchmark_group("jisx0201");

    // Creation vs cached
    group.bench_function("latin/create", |b| b.iter(|| LatinLetters::new()));
    group.bench_function("latin/cached", |b| b.iter(|| LatinLetters::cached()));
    group.bench_function("katakana/create", |b| b.iter(|| Katakana::new()));
    group.bench_function("katakana/cached", |b| b.iter(|| Katakana::cached()));
    group.bench_function("combined/create", |b| b.iter(|| JisX0201::new()));
    group.bench_function("combined/cached", |b| b.iter(|| JisX0201::cached()));

    let latin = LatinLetters::cached();
    let katakana = Katakana::cached();
    let combined = JisX0201::cached();

    // Contains
    group.bench_function("latin/contains", |b| {
        b.iter(|| latin.contains(black_box("Hello¥‾")))
    });
    group.bench_function("katakana/contains", |b| {
        b.iter(|| katakana.contains(black_box("ｱｲｳｴｵ")))
    });
    group.bench_function("combined/contains", |b| {
        b.iter(|| combined.contains(black_box("Hello¥ｱｲｳ")))
    });

    // Validation
    group.bench_function("latin/validate", |b| {
        b.iter(|| latin.validate(black_box("Hello¥")))
    });
    group.bench_function("katakana/validate", |b| {
        b.iter(|| katakana.validate(black_box("ｱｲｳｴｵ")))
    });

    group.finish();
}

#[cfg(not(feature = "codepoints-jisx0201"))]
fn group_jisx0201(_c: &mut Criterion) {}

// ============================================================================
// JIS X 0208 Group
// ============================================================================

#[cfg(feature = "codepoints-jisx0208")]
fn group_jisx0208(c: &mut Criterion) {
    use japanese_codepoints::jisx0208::{
        BoxDrawingChars, CyrillicLetters, GreekLetters, Hiragana, JisX0208, Katakana, LatinLetters,
        SpecialChars,
    };

    let mut group = c.benchmark_group("jisx0208");

    // Creation vs cached for main types
    group.bench_function("hiragana/create", |b| b.iter(|| Hiragana::new()));
    group.bench_function("hiragana/cached", |b| b.iter(|| Hiragana::cached()));
    group.bench_function("katakana/create", |b| b.iter(|| Katakana::new()));
    group.bench_function("katakana/cached", |b| b.iter(|| Katakana::cached()));
    group.bench_function("latin/create", |b| b.iter(|| LatinLetters::new()));
    group.bench_function("latin/cached", |b| b.iter(|| LatinLetters::cached()));
    group.bench_function("greek/create", |b| b.iter(|| GreekLetters::new()));
    group.bench_function("greek/cached", |b| b.iter(|| GreekLetters::cached()));
    group.bench_function("cyrillic/create", |b| b.iter(|| CyrillicLetters::new()));
    group.bench_function("cyrillic/cached", |b| b.iter(|| CyrillicLetters::cached()));
    group.bench_function("special/create", |b| b.iter(|| SpecialChars::new()));
    group.bench_function("special/cached", |b| b.iter(|| SpecialChars::cached()));
    group.bench_function("box_drawing/create", |b| b.iter(|| BoxDrawingChars::new()));
    group.bench_function("box_drawing/cached", |b| {
        b.iter(|| BoxDrawingChars::cached())
    });
    group.bench_function("combined/create", |b| b.iter(|| JisX0208::new()));
    group.bench_function("combined/cached", |b| b.iter(|| JisX0208::cached()));

    let hiragana = Hiragana::cached();
    let katakana = Katakana::cached();
    let greek = GreekLetters::cached();
    let cyrillic = CyrillicLetters::cached();
    let combined = JisX0208::cached();

    // Contains with different text lengths
    group.bench_function("hiragana/contains_short", |b| {
        b.iter(|| hiragana.contains(black_box("あいう")))
    });
    group.bench_function("hiragana/contains_long", |b| {
        b.iter(|| hiragana.contains(black_box("あいうえおかきくけこさしすせそ")))
    });
    group.bench_function("katakana/contains", |b| {
        b.iter(|| katakana.contains(black_box("アイウエオ")))
    });
    group.bench_function("greek/contains", |b| {
        b.iter(|| greek.contains(black_box("ΑΒΓΔΕαβγδε")))
    });
    group.bench_function("cyrillic/contains", |b| {
        b.iter(|| cyrillic.contains(black_box("АБВГДабвгд")))
    });
    group.bench_function("combined/contains", |b| {
        b.iter(|| combined.contains(black_box("あいうアイウＡＢＣ")))
    });

    // Multi-set validation
    group.bench_function("multi/hiragana_katakana", |b| {
        b.iter(|| {
            contains_all_in_any(
                black_box("あいうえおアイウエオ"),
                &[hiragana.codepoints(), katakana.codepoints()],
            )
        })
    });

    group.finish();
}

#[cfg(not(feature = "codepoints-jisx0208"))]
fn group_jisx0208(_c: &mut Criterion) {}

// ============================================================================
// Kanji Groups
// ============================================================================

#[cfg(feature = "codepoints-jisx0208kanji")]
fn group_jisx0208kanji(c: &mut Criterion) {
    use japanese_codepoints::JisX0208Kanji;

    let mut group = c.benchmark_group("jisx0208kanji");

    let kanji = JisX0208Kanji::cached();

    // Creation vs cached
    group.bench_function("create", |b| b.iter(|| JisX0208Kanji::new()));
    group.bench_function("cached", |b| b.iter(|| JisX0208Kanji::cached()));

    // Contains with different lengths
    group.bench_function("contains/3chars", |b| {
        b.iter(|| kanji.contains(black_box("日本国")))
    });
    group.bench_function("contains/10chars", |b| {
        b.iter(|| kanji.contains(black_box("日本国東京都新宿区")))
    });
    group.bench_function("contains/20chars", |b| {
        b.iter(|| kanji.contains(black_box("日本国東京都新宿区西新宿二丁目八番一号")))
    });

    // First excluded (worst case: all valid)
    group.bench_function("first_excluded/all_valid", |b| {
        b.iter(|| kanji.codepoints().first_excluded(black_box("日本国東京都")))
    });
    group.bench_function("first_excluded/early_fail", |b| {
        b.iter(|| {
            kanji
                .codepoints()
                .first_excluded(black_box("A日本国東京都"))
        })
    });

    // Validation
    group.bench_function("validate/ok", |b| {
        b.iter(|| kanji.validate(black_box("日本国東京都")))
    });
    group.bench_function("validate/err", |b| {
        b.iter(|| kanji.validate(black_box("日本国A東京都")))
    });

    group.finish();
}

#[cfg(not(feature = "codepoints-jisx0208kanji"))]
fn group_jisx0208kanji(_c: &mut Criterion) {}

#[cfg(feature = "codepoints-jisx0213kanji")]
fn group_jisx0213kanji(c: &mut Criterion) {
    use japanese_codepoints::JisX0213Kanji;

    let mut group = c.benchmark_group("jisx0213kanji");

    let kanji = JisX0213Kanji::cached();

    // Creation vs cached
    group.bench_function("create", |b| b.iter(|| JisX0213Kanji::new()));
    group.bench_function("cached", |b| b.iter(|| JisX0213Kanji::cached()));

    // Contains
    group.bench_function("contains/3chars", |b| {
        b.iter(|| kanji.contains(black_box("日本国")))
    });
    group.bench_function("contains/10chars", |b| {
        b.iter(|| kanji.contains(black_box("日本国東京都新宿区")))
    });
    group.bench_function("contains/20chars", |b| {
        b.iter(|| kanji.contains(black_box("日本国東京都新宿区西新宿二丁目八番一号")))
    });

    // Compare JIS X 0213 vs JIS X 0208 kanji (if available)
    #[cfg(feature = "codepoints-jisx0208kanji")]
    {
        use japanese_codepoints::JisX0208Kanji;

        let jisx0208 = JisX0208Kanji::cached();

        group.bench_function("compare/jisx0208", |b| {
            b.iter(|| jisx0208.contains(black_box("日本国東京都新宿区")))
        });
        group.bench_function("compare/jisx0213", |b| {
            b.iter(|| kanji.contains(black_box("日本国東京都新宿区")))
        });
    }

    group.finish();
}

#[cfg(not(feature = "codepoints-jisx0213kanji"))]
fn group_jisx0213kanji(_c: &mut Criterion) {}

// ============================================================================
// Large Text Scaling
// ============================================================================

fn group_large_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_text");

    let ascii = CodePoints::ascii_printable_cached();

    // Different text sizes
    let sizes = [
        ("100", 100),
        ("1k", 1_000),
        ("10k", 10_000),
        ("100k", 100_000),
    ];

    for (name, size) in &sizes {
        let text = "Hello World! Test ".repeat(size / 18);
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_function(&format!("ascii/{}", name), |b| {
            b.iter(|| ascii.contains(black_box(&text)))
        });
    }

    #[cfg(feature = "codepoints-jisx0208kanji")]
    {
        use japanese_codepoints::JisX0208Kanji;

        let kanji = JisX0208Kanji::cached();

        for (name, size) in &sizes {
            // Japanese characters are typically 3 bytes in UTF-8
            let char_count = size / 3;
            let text = "日本国東京都".repeat(char_count / 5);
            group.throughput(Throughput::Bytes(text.len() as u64));
            group.bench_function(&format!("kanji/{}", name), |b| {
                b.iter(|| kanji.contains(black_box(&text)))
            });
        }
    }

    group.finish();
}

// ============================================================================
// Real-world Scenarios
// ============================================================================

fn group_real_world(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world");

    // Email validation (ASCII printable)
    let email = "user.name+tag@example.com";
    let ascii = CodePoints::ascii_printable_cached();

    group.throughput(Throughput::Bytes(email.len() as u64));
    group.bench_function("email_validation", |b| {
        b.iter(|| ascii.validate(black_box(email)))
    });

    // Form field validation (Japanese name)
    #[cfg(feature = "codepoints-jisx0208")]
    {
        use japanese_codepoints::jisx0208::Hiragana;

        let hiragana = Hiragana::cached();
        let japanese_name = "たなかたろう";

        group.throughput(Throughput::Bytes(japanese_name.len() as u64));
        group.bench_function("japanese_name_hiragana", |b| {
            b.iter(|| hiragana.validate(black_box(japanese_name)))
        });
    }

    // Mixed validation (Hiragana + Katakana + ASCII)
    #[cfg(feature = "codepoints-jisx0208")]
    {
        use japanese_codepoints::jisx0208::{Hiragana, Katakana};

        let hiragana = Hiragana::cached();
        let katakana = Katakana::cached();
        let mixed_text = "こんにちはHelloアイウ";

        group.bench_function("mixed_japanese_ascii", |b| {
            b.iter(|| {
                contains_all_in_any(
                    black_box(mixed_text),
                    &[hiragana.codepoints(), katakana.codepoints(), &ascii],
                )
            })
        });
    }

    group.finish();
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to run a benchmark with throughput measurement
fn bench_with_throughput<F>(group: &mut BenchmarkGroup<WallTime>, name: &str, mut f: F)
where
    F: FnMut() -> bool,
{
    group.bench_function(name, move |b| b.iter(&mut f));
}

// ============================================================================
// Criterion Groups
// ============================================================================

criterion_group!(
    name = core_ops;
    config = Criterion::default()
        .sample_size(200)
        .warm_up_time(std::time::Duration::from_secs(1))
        .measurement_time(std::time::Duration::from_secs(3));
    targets = group_core_ops, group_set_operations, group_exclusion_queries, group_construction
);

criterion_group!(
    name = character_sets;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(3));
    targets = group_ascii, group_caching, group_validation
);

criterion_group!(
    name = japanese_sets;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(3));
    targets = group_jisx0201, group_jisx0208, group_jisx0208kanji, group_jisx0213kanji
);

criterion_group!(
    name = scaling;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(std::time::Duration::from_secs(5));
    targets = group_large_text
);

criterion_group!(
    name = scenarios;
    config = Criterion::default()
        .sample_size(200)
        .measurement_time(std::time::Duration::from_secs(3));
    targets = group_real_world
);

criterion_main!(core_ops, character_sets, japanese_sets, scaling, scenarios,);
