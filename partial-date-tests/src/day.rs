// Tests for day value extraction.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Day-only extraction: only the day is expected, only the day is returned
// -------------------------------------------------------------------------

/// When only a day is expected and a bare number 1–31 is provided,
/// it should be extracted as Found with the correct value.
#[rstest]
#[case("1", 1)]
#[case("2", 2)]
#[case("10", 10)]
#[case("15", 15)]
#[case("28", 28)]
#[case("29", 29)]
#[case("30", 30)]
#[case("31", 31)]
fn day_only_bare_number(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Day values outside the valid range (0, 32+) should not be found.
#[rstest]
#[case("0")]
#[case("32")]
#[case("99")]
#[case("100")]
fn day_only_invalid_values_not_found(#[case] utterance: &str) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert!(result.day.value.is_not_found());
}

/// When the day is expected and the input has surrounding text, the day
/// should still be extracted.
#[rstest]
#[case("the 5", 5)]
#[case("on the 12", 12)]
#[case("day 1", 1)]
#[case("it was the 25", 25)]
fn day_only_with_surrounding_text(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

// -------------------------------------------------------------------------
// Day with ordinal suffixes: "1st", "2nd", "3rd", "12th" etc.
// -------------------------------------------------------------------------

#[rstest]
#[case("1st", 1)]
#[case("2nd", 2)]
#[case("3rd", 3)]
#[case("4th", 4)]
#[case("11th", 11)]
#[case("12th", 12)]
#[case("13th", 13)]
#[case("21st", 21)]
#[case("22nd", 22)]
#[case("23rd", 23)]
#[case("31st", 31)]
fn day_with_ordinal_suffix(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Ordinal suffixes in surrounding text.
#[rstest]
#[case("the 1st of the month", 1)]
#[case("on the 15th", 15)]
#[case("happened on 22nd", 22)]
#[case("the 3rd day", 3)]
fn day_ordinal_in_context(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

// -------------------------------------------------------------------------
// Day + month provided even when only day is expected:
// If input clearly contains both, we should still extract the day correctly.
// -------------------------------------------------------------------------

/// When day is expected and input contains "12 June", the day should be
/// correctly identified as 12 even though a month name is present.
#[rstest]
#[case("12 June", 12)]
#[case("1 January", 1)]
#[case("31 December", 31)]
#[case("15 March", 15)]
fn day_extracted_when_month_name_also_present(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

// -------------------------------------------------------------------------
// Day with default value configured
// -------------------------------------------------------------------------

/// When day is not found and a default is configured, return Defaulted.
#[test]
fn day_not_found_returns_defaulted_when_configured() {
    let config = Config {
        day: DayConfig {
            expected: IsExpected::Yes,
            default: Some(1),
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        year: YearConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config("no day here", config);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Defaulted(1));
}

/// When day is not found and no default is configured, return NotFound.
#[test]
fn day_not_found_returns_not_found_when_no_default() {
    let input = input_with_config("no day here", day_only_config());
    let result = extract(input);

    assert!(result.day.value.is_not_found());
}

// -------------------------------------------------------------------------
// Day with custom min/max validation
// -------------------------------------------------------------------------

/// Custom min/max on the day config: value within range should be found.
#[rstest]
#[case("10", 10)]
#[case("15", 15)]
#[case("20", 20)]
fn day_custom_min_max_within_range(#[case] utterance: &str, #[case] expected_day: u8) {
    let config = Config {
        day: DayConfig {
            min: 10,
            max: 20,
            expected: IsExpected::Yes,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Custom min/max on the day config: values outside range should not be found.
#[rstest]
#[case("5")]
#[case("9")]
#[case("21")]
#[case("25")]
fn day_custom_min_max_outside_range(#[case] utterance: &str) {
    let config = Config {
        day: DayConfig {
            min: 10,
            max: 20,
            expected: IsExpected::Yes,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.day.value.is_not_found());
}

// -------------------------------------------------------------------------
// Edge cases
// -------------------------------------------------------------------------

/// Empty or whitespace-only input should return NotFound.
#[rstest]
#[case("")]
#[case("   ")]
#[case("\t")]
#[case("\n")]
fn day_blank_input(#[case] utterance: &str) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert!(result.day.value.is_not_found());
}

/// Leading zeros should be handled: "01" -> day 1.
#[rstest]
#[case("01", 1)]
#[case("09", 9)]
fn day_leading_zeros(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Case insensitivity for ordinal suffixes.
#[rstest]
#[case("1ST", 1)]
#[case("2ND", 2)]
#[case("3RD", 3)]
#[case("4TH", 4)]
fn day_ordinal_case_insensitive(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

// -------------------------------------------------------------------------
// Stronger edge cases: boundary days, ordinal variations
// -------------------------------------------------------------------------

/// All ordinal variants for boundary days (1, 2, 3, 21, 22, 23, 31).
#[rstest]
#[case("1st", 1)]
#[case("1ST", 1)]
#[case("1St", 1)]
#[case("2nd", 2)]
#[case("2ND", 2)]
#[case("2Nd", 2)]
#[case("3rd", 3)]
#[case("3RD", 3)]
#[case("3Rd", 3)]
#[case("21st", 21)]
#[case("21ST", 21)]
#[case("22nd", 22)]
#[case("22ND", 22)]
#[case("23rd", 23)]
#[case("23RD", 23)]
#[case("31st", 31)]
#[case("31ST", 31)]
fn day_all_ordinal_case_variants(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Mixed ordinal forms (unusual but valid): "1th", "2th", etc.
#[rstest]
#[case("11th", 11)]
#[case("12th", 12)]
#[case("13th", 13)]
#[case("4th", 4)]
#[case("5th", 5)]
fn day_ordinal_mixed_forms(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Day boundary values: min (1) and max (31).
#[rstest]
#[case("1", 1)]
#[case("31", 31)]
#[case("01", 1)]
#[case("31st", 31)]
#[case("1st", 1)]
fn day_boundary_min_max(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Day with excessive spacing around the number.
#[rstest]
#[case("  15  ", 15)]
#[case("   1st   ", 1)]
#[case("  31st  ", 31)]
fn day_excessive_spacing(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Day within natural language text with different structures.
#[rstest]
#[case("the 1 of each month", 1)]
#[case("every 15th", 15)]
#[case("by the 31st of December", 31)]
#[case("on day 22", 22)]
#[case("appointment at 5 in the morning", 5)]
fn day_in_various_natural_language_contexts(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Day with punctuation immediately following.
#[rstest]
#[case("15.", 15)]
#[case("31,", 31)]
#[case("1st.", 1)]
#[case("22nd,", 22)]
#[case("3rd!", 3)]
fn day_with_trailing_punctuation(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Day numbers with surrounding alphanumeric context.
#[rstest]
#[case("day15meeting", 15)]
#[case("the31st-day", 31)]
#[case("22nd-floor", 22)]
#[case("1st-place", 1)]
fn day_surrounded_by_alphanumeric(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// All valid days (1–31) to ensure comprehensive coverage.
#[rstest]
#[case("1", 1)]
#[case("2", 2)]
#[case("3", 3)]
#[case("4", 4)]
#[case("5", 5)]
#[case("6", 6)]
#[case("7", 7)]
#[case("8", 8)]
#[case("9", 9)]
#[case("10", 10)]
#[case("11", 11)]
#[case("12", 12)]
#[case("13", 13)]
#[case("14", 14)]
#[case("15", 15)]
#[case("16", 16)]
#[case("17", 17)]
#[case("18", 18)]
#[case("19", 19)]
#[case("20", 20)]
#[case("21", 21)]
#[case("22", 22)]
#[case("23", 23)]
#[case("24", 24)]
#[case("25", 25)]
#[case("26", 26)]
#[case("27", 27)]
#[case("28", 28)]
#[case("29", 29)]
#[case("30", 30)]
#[case("31", 31)]
fn day_all_valid_days_1_to_31(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}
