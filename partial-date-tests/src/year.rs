// Tests for year extraction.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Year-only extraction: four-digit years
// -------------------------------------------------------------------------

/// Common four-digit years should be extracted as Found.
#[rstest]
#[case("2024", 2024)]
#[case("2025", 2025)]
#[case("2000", 2000)]
#[case("1999", 1999)]
#[case("1900", 1900)]
#[case("2100", 2100)]
#[case("0001", 1)]
fn year_only_four_digit(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
}

/// Common four-digit years with the letter O instead of a 0 should be extracted as Found.
#[rstest]
#[case("2o24", 2024)]
#[case("2O25", 2025)]
#[case("2O00", 2000)]
#[case("19oo", 1900)]
#[case("21OO", 2100)]
#[case("ooo1", 1)]
fn year_only_four_digit_includes_o(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
}

/// Year in surrounding text.
#[rstest]
#[case("the year 2024", 2024)]
#[case("in 1999", 1999)]
#[case("happened in 2001", 2001)]
fn year_in_surrounding_text(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Two-digit year expansion: sliding window (default)
// -------------------------------------------------------------------------

/// Two-digit years with the default sliding window expansion:
/// 00–49 -> 2000–2049, 50–99 -> 1950–1999.
#[rstest]
#[case("00", 2000)]
#[case("01", 2001)]
#[case("24", 2024)]
#[case("25", 2025)]
#[case("49", 2049)]
#[case("50", 1950)]
#[case("75", 1975)]
#[case("99", 1999)]
fn year_two_digit_sliding_window(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Two-digit year expansion: custom sliding window (00–69 → 2000–2069, 70–99 → 1970–1999)
// -------------------------------------------------------------------------

/// A custom sliding window with a non-default pivot shifts the mapping.
/// earliest_year: 1970, pivot: 70 → 00–69 map to 2000–2069, 70–99 map to 1970–1999.
#[rstest]
#[case("00", 2000)]
#[case("24", 2024)]
#[case("69", 2069)]
#[case("70", 1970)]
#[case("85", 1985)]
#[case("99", 1999)]
fn year_two_digit_custom_sliding_window(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::SlidingWindow {
                earliest_year: 1970,
                pivot: SlidingWindowPivot::new(70).unwrap(),
            },
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Industrial Revolution era sliding window: earliest_year 1750, pivot 50.
/// 00–49 → 1800–1849, 50–99 → 1750–1799. min: 1760, max: 1840 rejects out-of-era values.
#[rstest]
#[case("00", Some(1800))]
#[case("34", Some(1834))]
#[case("40", Some(1840))]
#[case("41", None)] // 1841 > max 1840 → rejected
#[case("49", None)] // 1849 > max 1840 → rejected
#[case("50", None)] // 1750 < min 1760 → rejected
#[case("59", None)] // 1759 < min 1760 → rejected
#[case("60", Some(1760))] // 1750 + (60 - 50) = 1760 ✓
#[case("88", Some(1788))] // 1750 + (88 - 50) = 1788 ✓
#[case("99", Some(1799))] // 1750 + (99 - 50) = 1799, within min:1760 max:1840 ✓
fn year_two_digit_industrial_revolution_window(
    #[case] utterance: &str,
    #[case] expected_year: Option<i32>,
) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            min: 1760,
            max: 1840,
            two_digit_expansion: TwoDigitYearExpansion::SlidingWindow {
                earliest_year: 1750,
                pivot: SlidingWindowPivot::new(50).unwrap(),
            },
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    match expected_year {
        Some(year) => assert_eq!(result.year.value, Extracted::Found(year)),
        None => assert!(result.year.value.is_not_found()),
    }
}

/// When earliest_year is entirely outside YearConfig::min/max, every two-digit
/// input should return NotFound — the window produces no valid years.
#[rstest]
#[case("00")]
#[case("49")]
#[case("50")]
#[case("99")]
fn year_two_digit_sliding_window_earliest_year_outside_range(#[case] utterance: &str) {
    // Window: earliest_year 1500, pivot 50 → produces 1500–1599.
    // YearConfig::min/max: 1760–1840 → no overlap, all values rejected.
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            min: 1760,
            max: 1840,
            two_digit_expansion: TwoDigitYearExpansion::SlidingWindow {
                earliest_year: 1500,
                pivot: SlidingWindowPivot::new(50).unwrap(),
            },
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Two-digit year expansion: Always(Century(2000))
// -------------------------------------------------------------------------

/// When TwoDigitYearExpansion::Always(Century(2000)) is configured, all
/// two-digit years map to 2000–2099.
#[rstest]
#[case("00", 2000)]
#[case("24", 2024)]
#[case("50", 2050)]
#[case("99", 2099)]
fn year_two_digit_always_2000s(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always(Century::new(2000).unwrap()),
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Two-digit year expansion: Literal
// -------------------------------------------------------------------------

/// When TwoDigitYearExpansion::Literal is configured, two-digit years
/// are returned as-is (e.g. 24 stays 24, not 2024).
#[rstest]
#[case("00", 0)]
#[case("24", 24)]
#[case("50", 50)]
#[case("99", 99)]
fn year_two_digit_literal(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Literal,
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Year with default value configured
// -------------------------------------------------------------------------

/// When year is not found and a default is configured, return Defaulted.
#[test]
fn year_not_found_returns_defaulted_when_configured() {
    let config = Config {
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        year: YearConfig {
            expected: IsExpected::Yes,
            default: Some(2025),
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config("no year here", config);
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Defaulted(2025));
}

/// When year is not found and no default is configured, return NotFound.
#[test]
fn year_not_found_returns_not_found_when_no_default() {
    let input = input_with_config("no year here", year_only_config());
    let result = extract(input);

    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Year with custom min/max validation
// -------------------------------------------------------------------------

/// Years within the configured min/max range should be found.
#[rstest]
#[case("2000", 2000)]
#[case("2025", 2025)]
#[case("2030", 2030)]
fn year_custom_min_max_within_range(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            min: 2000,
            max: 2030,
            expected: IsExpected::Yes,
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Years outside the configured min/max range should not be found.
#[rstest]
#[case("1999")]
#[case("1900")]
#[case("2031")]
#[case("2050")]
fn year_custom_min_max_outside_range(#[case] utterance: &str) {
    let config = Config {
        year: YearConfig {
            min: 2000,
            max: 2030,
            expected: IsExpected::Yes,
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Edge cases
// -------------------------------------------------------------------------

/// Empty input returns NotFound.
#[test]
fn year_empty_input() {
    let input = input_with_config("", year_only_config());
    let result = extract(input);

    assert!(result.year.value.is_not_found());
}

/// Three-digit numbers (100–999) are treated as literal years.
/// This supports word-number-replaced inputs like "one hundred" → "100" → year=100.
#[rstest]
#[case("123", 123)]
#[case("999", 999)]
fn year_three_digit_is_valid(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Five-digit numbers should not be treated as years.
#[test]
fn year_five_digit_not_valid() {
    let input = input_with_config("10000", year_only_config());
    let result = extract(input);

    assert!(result.year.value.is_not_found());
}

/// Year at boundaries of the default range.
#[rstest]
#[case("0000", 0)]
#[case("3000", 3000)]
fn year_boundary_values(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Year just outside the default range should not be found.
#[test]
fn year_outside_default_range() {
    let input = input_with_config("3001", year_only_config());
    let result = extract(input);

    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Stronger edge cases: all 2-digit window boundaries, leading zeros, etc.
// -------------------------------------------------------------------------

/// All boundary years for the default sliding window (00–49 → 2000–2049, 50–99 → 1950–1999).
#[rstest]
#[case("00", 2000)]
#[case("01", 2001)]
#[case("24", 2024)]
#[case("48", 2048)]
#[case("49", 2049)]
#[case("50", 1950)]
#[case("51", 1951)]
#[case("75", 1975)]
#[case("98", 1998)]
#[case("99", 1999)]
fn year_two_digit_all_window_boundaries(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// 4-digit years with leading zeros should parse correctly.
#[rstest]
#[case("0001", 1)]
#[case("0010", 10)]
#[case("0100", 100)]
#[case("0500", 500)]
#[case("1000", 1000)]
fn year_four_digit_with_leading_zeros(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Year with surrounding natural language text.
#[rstest]
#[case("in 2024", 2024)]
#[case("the year 1999", 1999)]
#[case("year 2025", 2025)]
#[case("happened in year 2000", 2000)]
#[case("back in 1975", 1975)]
fn year_in_natural_language_context(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Year with excessive spacing around it.
#[rstest]
#[case("  2024  ", 2024)]
#[case("   1999   ", 1999)]
fn year_excessive_spacing(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Year with trailing punctuation (period and comma).
#[rstest]
#[case("2024.", 2024)]
#[case("1999,", 1999)]
fn year_with_trailing_punctuation(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Historical years (before modern era).
#[rstest]
#[case("1776", 1776)]
#[case("1066", 1066)]
#[case("0500", 500)]
fn year_historical_years(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Future years far ahead.
#[rstest]
#[case("2100", 2100)]
#[case("2500", 2500)]
#[case("2999", 2999)]
#[case("3000", 3000)]
fn year_future_years(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Two-digit years with Always(Century(2000)) expansion: all values 00–99 map
/// to 2000–2099.
#[rstest]
#[case("00", 2000)]
#[case("25", 2025)]
#[case("50", 2050)]
#[case("75", 2075)]
#[case("99", 2099)]
fn year_two_digit_always_2000s_all_values(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always(Century::new(2000).unwrap()),
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Two-digit year expansion: Always(Century) — non-2000s centuries
// -------------------------------------------------------------------------

/// Always(Century(1800)): all two-digit values map to 1800–1899.
#[rstest]
#[case("00", 1800)]
#[case("34", 1834)]
#[case("50", 1850)]
#[case("88", 1888)]
#[case("99", 1899)]
fn year_two_digit_always_1800s(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always(Century::new(1800).unwrap()),
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Always(Century) with a YearConfig::min/max that is entirely outside the
/// century: every two-digit input should return NotFound.
#[rstest]
#[case("00")]
#[case("50")]
#[case("99")]
fn year_two_digit_always_century_outside_range(#[case] utterance: &str) {
    // Century(1800) → produces 1800–1899, but min/max is 1760–1840.
    // 1841–1899 are rejected; 1800–1840 would be accepted but that's 41 values.
    // This test checks a century entirely outside the range: Century(1700)
    // produces 1700–1799, which is fully below min 1800.
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always(Century::new(1700).unwrap()),
            min: 1800,
            max: 1899,
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.year.value.is_not_found());
}

/// Two-digit years with Literal expansion: values stay as-is.
#[rstest]
#[case("00", 0)]
#[case("01", 1)]
#[case("24", 24)]
#[case("50", 50)]
#[case("99", 99)]
fn year_two_digit_literal_all_values(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Literal,
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}
