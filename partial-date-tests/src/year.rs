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

/// A custom sliding window with a different pivot point should shift the mapping.
#[rstest]
#[case("00", 2000)]
#[case("24", 2024)]
#[case("69", 2069)]
#[case("70", 1970)]
#[case("85", 1985)]
#[case("99", 1999)]
fn year_two_digit_custom_sliding_window(#[case] utterance: &str, #[case] expected_year: i32) {
    let wr = WindowRange::new(
        Range {
            min: 2000,
            max: 2070,
        },
        Range {
            min: 1970,
            max: 2000,
        },
    )
    .unwrap();
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::SlidingWindow(wr),
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
// Two-digit year expansion: Always2000s
// -------------------------------------------------------------------------

/// When TwoDigitYearExpansion::Always2000s is configured, all two-digit
/// years map to 2000–2099.
#[rstest]
#[case("00", 2000)]
#[case("24", 2024)]
#[case("50", 2050)]
#[case("99", 2099)]
fn year_two_digit_always_2000s(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always2000s,
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

/// Three-digit numbers should not be treated as years when year is expected.
#[rstest]
#[case("123")]
#[case("999")]
fn year_three_digit_not_valid(#[case] utterance: &str) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert!(result.year.value.is_not_found());
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
