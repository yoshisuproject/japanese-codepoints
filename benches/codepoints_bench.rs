use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use japanese_codepoints::CodePoints;

fn bench_contains(c: &mut Criterion) {
    let cp = CodePoints::new(vec![0x3041, 0x3042, 0x3043, 0x3044, 0x3045]); // あ, い, う, え, お

    c.bench_function("contains_valid", |b| {
        b.iter(|| cp.contains(black_box("あいうえお")))
    });

    c.bench_function("contains_invalid", |b| {
        b.iter(|| cp.contains(black_box("あいうえおか")))
    });

    c.bench_function("contains_mixed", |b| {
        b.iter(|| cp.contains(black_box("あいうえおかきくけこ")))
    });
}

fn bench_set_operations(c: &mut Criterion) {
    let cp1 = CodePoints::new(vec![0x3041, 0x3042, 0x3043]); // あ, い, う
    let cp2 = CodePoints::new(vec![0x3042, 0x3043, 0x3044]); // い, う, え

    c.bench_function("union", |b| b.iter(|| cp1.union(black_box(&cp2))));

    c.bench_function("intersection", |b| {
        b.iter(|| cp1.intersection(black_box(&cp2)))
    });

    c.bench_function("difference", |b| b.iter(|| cp1.difference(black_box(&cp2))));
}

fn bench_first_excluded(c: &mut Criterion) {
    let cp = CodePoints::new(vec![0x3041, 0x3042, 0x3043]); // あ, い, う

    c.bench_function("first_excluded_none", |b| {
        b.iter(|| cp.first_excluded(black_box("あいう")))
    });

    c.bench_function("first_excluded_early", |b| {
        b.iter(|| cp.first_excluded(black_box("あえいう")))
    });

    c.bench_function("first_excluded_late", |b| {
        b.iter(|| cp.first_excluded(black_box("あいうえお")))
    });
}

fn bench_all_excluded(c: &mut Criterion) {
    let cp = CodePoints::new(vec![0x3041, 0x3042, 0x3043]); // あ, い, う

    c.bench_function("all_excluded_none", |b| {
        b.iter(|| cp.all_excluded(black_box("あいう")))
    });

    c.bench_function("all_excluded_some", |b| {
        b.iter(|| cp.all_excluded(black_box("あいうえおかきくけこ")))
    });
}

fn bench_from_string(c: &mut Criterion) {
    c.bench_function("from_string_short", |b| {
        b.iter(|| CodePoints::from_string(black_box("あいうえお")))
    });

    c.bench_function("from_string_long", |b| {
        b.iter(|| CodePoints::from_string(black_box("あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをん")))
    });
}

fn bench_ascii_coverage(c: &mut Criterion) {
    let ascii = CodePoints::ascii_printable();

    c.bench_function("ascii_contains_short", |b| {
        b.iter(|| ascii.contains(black_box("Hello")))
    });

    c.bench_function("ascii_contains_long", |b| {
        b.iter(|| {
            ascii.contains(black_box(
                "Hello World! This is a test string with ASCII characters.",
            ))
        })
    });

    c.bench_function("ascii_contains_mixed", |b| {
        b.iter(|| ascii.contains(black_box("Hello 世界! This has mixed characters.")))
    });

    c.bench_function("ascii_first_excluded", |b| {
        b.iter(|| ascii.first_excluded(black_box("Hello 世界!")))
    });
}

// New caching performance benchmarks
fn bench_caching_performance(c: &mut Criterion) {
    // ASCII caching benchmarks
    c.bench_function("ascii_printable_creation", |b| {
        b.iter(|| CodePoints::ascii_printable())
    });

    c.bench_function("ascii_printable_cached_access", |b| {
        b.iter(|| CodePoints::ascii_printable_cached())
    });

    c.bench_function("ascii_control_creation", |b| {
        b.iter(|| CodePoints::ascii_control())
    });

    c.bench_function("ascii_control_cached_access", |b| {
        b.iter(|| CodePoints::ascii_control_cached())
    });

    c.bench_function("crlf_creation", |b| b.iter(|| CodePoints::crlf()));

    c.bench_function("crlf_cached_access", |b| {
        b.iter(|| CodePoints::crlf_cached())
    });

    c.bench_function("ascii_all_creation", |b| b.iter(|| CodePoints::ascii_all()));

    c.bench_function("ascii_all_cached_access", |b| {
        b.iter(|| CodePoints::ascii_all_cached())
    });
}

#[cfg(feature = "codepoints-jisx0201")]
fn bench_jisx0201_coverage(c: &mut Criterion) {
    use japanese_codepoints::jisx0201::{Katakana, LatinLetters};

    // Test both creation and cached access
    c.bench_function("jisx0201_katakana_creation", |b| b.iter(|| Katakana::new()));

    c.bench_function("jisx0201_katakana_cached_access", |b| {
        b.iter(|| Katakana::cached())
    });

    c.bench_function("jisx0201_latin_creation", |b| {
        b.iter(|| LatinLetters::new())
    });

    c.bench_function("jisx0201_latin_cached_access", |b| {
        b.iter(|| LatinLetters::cached())
    });

    // Use cached instances for actual functionality tests
    let katakana = Katakana::cached();
    let latin = LatinLetters::cached();

    c.bench_function("jisx0201_katakana_contains", |b| {
        b.iter(|| katakana.contains(black_box("ｱｲｳｴｵ")))
    });

    c.bench_function("jisx0201_latin_contains", |b| {
        b.iter(|| latin.contains(black_box("ABCDE¥")))
    });

    c.bench_function("jisx0201_katakana_mixed", |b| {
        b.iter(|| katakana.contains(black_box("ｱｲｳｴｵあいうえお")))
    });

    c.bench_function("jisx0201_first_excluded", |b| {
        b.iter(|| katakana.codepoints().first_excluded(black_box("ｱｲｳｴｵあ")))
    });
}

#[cfg(not(feature = "codepoints-jisx0201"))]
fn bench_jisx0201_coverage(_c: &mut Criterion) {}

#[cfg(feature = "codepoints-jisx0208")]
fn bench_jisx0208_coverage(c: &mut Criterion) {
    use japanese_codepoints::jisx0208::{Hiragana, Katakana};

    // Test creation vs cached access performance
    c.bench_function("jisx0208_hiragana_creation", |b| b.iter(|| Hiragana::new()));

    c.bench_function("jisx0208_hiragana_cached_access", |b| {
        b.iter(|| Hiragana::cached())
    });

    c.bench_function("jisx0208_katakana_creation", |b| b.iter(|| Katakana::new()));

    c.bench_function("jisx0208_katakana_cached_access", |b| {
        b.iter(|| Katakana::cached())
    });

    // Use cached instances for functionality tests
    let hiragana = Hiragana::cached();
    let katakana = Katakana::cached();

    c.bench_function("jisx0208_hiragana_contains", |b| {
        b.iter(|| hiragana.contains(black_box("あいうえおかきくけこ")))
    });

    c.bench_function("jisx0208_katakana_contains", |b| {
        b.iter(|| katakana.contains(black_box("アイウエオカキクケコ")))
    });

    c.bench_function("jisx0208_mixed_validation", |b| {
        b.iter(|| {
            let text = "あいうえおアイウエオ";
            hiragana.contains(text) && katakana.contains(text)
        })
    });

    // Test multi-codepoints validation
    c.bench_function("jisx0208_multi_codepoints_validation", |b| {
        b.iter(|| {
            let text = "あいうえおアイウエオ";
            let collections = [hiragana.codepoints().clone(), katakana.codepoints().clone()];
            CodePoints::contains_all_in_any(text, &collections)
        })
    });
}

#[cfg(not(feature = "codepoints-jisx0208"))]
fn bench_jisx0208_coverage(_c: &mut Criterion) {}

#[cfg(feature = "codepoints-jisx0208kanji")]
fn bench_jisx0208kanji_coverage(c: &mut Criterion) {
    use japanese_codepoints::JisX0208Kanji;

    let kanji = JisX0208Kanji::new();

    c.bench_function("jisx0208kanji_contains", |b| {
        b.iter(|| kanji.contains(black_box("日本国")))
    });

    c.bench_function("jisx0208kanji_long", |b| {
        b.iter(|| kanji.contains(black_box("日本国東京都新宿区西新宿二丁目八番一号")))
    });
}

#[cfg(not(feature = "codepoints-jisx0208kanji"))]
fn bench_jisx0208kanji_coverage(_c: &mut Criterion) {}

#[cfg(feature = "codepoints-jisx0213kanji")]
fn bench_jisx0213_coverage(c: &mut Criterion) {
    use japanese_codepoints::JisX0213Kanji;

    let kanji = JisX0213Kanji::new();

    c.bench_function("jisx0213_kanji_contains", |b| {
        b.iter(|| kanji.contains(black_box("日本国")))
    });

    c.bench_function("jisx0213_kanji_long", |b| {
        b.iter(|| kanji.contains(black_box("日本国東京都新宿区西新宿二丁目八番一号")))
    });

    c.bench_function("jisx0213_kanji_with_extended", |b| {
        b.iter(|| kanji.contains(black_box("日本国𠮟咤")))
    });
}

#[cfg(not(feature = "codepoints-jisx0213kanji"))]
fn bench_jisx0213_coverage(_c: &mut Criterion) {}

fn bench_coverage_comparison(c: &mut Criterion) {
    // Use cached instances for better performance
    let ascii = CodePoints::ascii_printable_cached();

    // Simplified test cases - focus on most important scenarios
    let test_cases = [
        ("ascii_only", "Hello World"),
        ("japanese_hiragana", "あいうえお"),
        ("japanese_kanji", "日本国"),
        ("mixed_content", "Hello あいうえお"),
    ];

    for (name, text) in test_cases.iter() {
        c.bench_function(&format!("ascii_coverage_{}", name), |b| {
            b.iter(|| ascii.contains(black_box(text)))
        });
    }

    #[cfg(feature = "codepoints-jisx0208")]
    {
        use japanese_codepoints::jisx0208::Hiragana;
        let hiragana = Hiragana::cached(); // Use cached version

        for (name, text) in test_cases.iter() {
            c.bench_function(&format!("hiragana_coverage_{}", name), |b| {
                b.iter(|| hiragana.contains(black_box(text)))
            });
        }
    }

    // Only test kanji with relevant text
    #[cfg(feature = "codepoints-jisx0208kanji")]
    {
        use japanese_codepoints::JisX0208Kanji;
        let kanji = JisX0208Kanji::new();

        let kanji_test_cases = [("kanji_simple", "日本国"), ("kanji_mixed", "Hello 日本国")];

        for (name, text) in kanji_test_cases.iter() {
            c.bench_function(&format!("kanji_coverage_{}", name), |b| {
                b.iter(|| kanji.contains(black_box(text)))
            });
        }
    }
}

fn bench_large_text_performance(c: &mut Criterion) {
    // Test with different sizes for more comprehensive benchmarking
    let small_text = "日本国東京都".repeat(10); // ~50 chars
    let medium_text = "日本国東京都".repeat(50); // ~250 chars
    let large_text = "日本国東京都".repeat(200); // ~1000 chars

    // Test ASCII performance with large text
    let ascii = CodePoints::ascii_printable_cached();
    let ascii_large = "Hello World! This is a performance test. ".repeat(100);

    c.bench_function("large_ascii_text_contains", |b| {
        b.iter(|| ascii.contains(black_box(&ascii_large)))
    });

    #[cfg(feature = "codepoints-jisx0208kanji")]
    {
        use japanese_codepoints::JisX0208Kanji;
        let kanji = JisX0208Kanji::new();

        c.bench_function("small_kanji_text_contains", |b| {
            b.iter(|| kanji.contains(black_box(&small_text)))
        });

        c.bench_function("medium_kanji_text_contains", |b| {
            b.iter(|| kanji.contains(black_box(&medium_text)))
        });

        c.bench_function("large_kanji_text_contains", |b| {
            b.iter(|| kanji.contains(black_box(&large_text)))
        });

        c.bench_function("large_kanji_first_excluded", |b| {
            b.iter(|| kanji.all.first_excluded(black_box(&large_text)))
        });
    }
}

// New validation macro benchmarks
fn bench_validation_macros(c: &mut Criterion) {
    // Basic validation macro performance
    let ascii = CodePoints::ascii_printable();

    c.bench_function("validate_codepoints_macro", |b| {
        b.iter(|| {
            japanese_codepoints::validate_codepoints!(black_box("Hello World"), ascii.clone())
        })
    });

    c.bench_function("validate_codepoints_advanced_macro", |b| {
        b.iter(|| {
            japanese_codepoints::validate_codepoints_advanced!(
                black_box("Hello World"),
                ascii.clone(),
                "Custom error"
            )
        })
    });

    // Test predefined shortcuts
    c.bench_function("validate_ascii_printable_shortcut", |b| {
        b.iter(|| {
            japanese_codepoints::validate_codepoints_advanced!(
                black_box("Hello World"),
                ascii_printable
            )
        })
    });

    #[cfg(feature = "codepoints-jisx0208")]
    {
        c.bench_function("validate_hiragana_macro", |b| {
            b.iter(|| japanese_codepoints::validate_hiragana!(black_box("あいうえお")))
        });

        c.bench_function("validate_katakana_macro", |b| {
            b.iter(|| japanese_codepoints::validate_katakana!(black_box("アイウエオ")))
        });

        c.bench_function("validate_japanese_mixed_macro", |b| {
            b.iter(|| japanese_codepoints::validate_japanese_mixed!(black_box("こんにちはHello")))
        });
    }
}

criterion_group!(
    benches,
    bench_contains,
    bench_set_operations,
    bench_first_excluded,
    bench_all_excluded,
    bench_from_string,
    bench_ascii_coverage,
    bench_caching_performance,
    bench_validation_macros,
    bench_jisx0201_coverage,
    bench_jisx0208_coverage,
    bench_jisx0208kanji_coverage,
    bench_jisx0213_coverage,
    bench_coverage_comparison,
    bench_large_text_performance,
);
criterion_main!(benches);
