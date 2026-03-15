// Tests for month extraction.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Month-only extraction: numeric values 1–12
// -------------------------------------------------------------------------

/// All valid month numbers 1–12 should be extracted when month is the only
/// expected value.
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
fn month_only_valid_numbers(#[case] utterance: &str, #[case] expected_month: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.day.value.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Numbers outside valid month range should not be found.
#[rstest]
#[case("0")]
#[case("13")]
#[case("99")]
fn month_only_invalid_numbers(#[case] utterance: &str) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert!(result.month.number.is_not_found());
}

/// Leading zeros should work: "01" -> month 1.
#[rstest]
#[case("01", 1)]
#[case("06", 6)]
#[case("09", 9)]
#[case("12", 12)]
fn month_leading_zeros(#[case] utterance: &str, #[case] expected_month: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_month));
}

// -------------------------------------------------------------------------
// Month from full names
// -------------------------------------------------------------------------

/// Full month names should be extracted and mapped to the correct number
/// and MonthName.
#[rstest]
#[case("January", 1, MonthName::January)]
#[case("February", 2, MonthName::February)]
#[case("March", 3, MonthName::March)]
#[case("April", 4, MonthName::April)]
#[case("May", 5, MonthName::May)]
#[case("June", 6, MonthName::June)]
#[case("July", 7, MonthName::July)]
#[case("August", 8, MonthName::August)]
#[case("September", 9, MonthName::September)]
#[case("October", 10, MonthName::October)]
#[case("November", 11, MonthName::November)]
#[case("December", 12, MonthName::December)]
fn month_full_names(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
}

/// Full month names should be case-insensitive.
#[rstest]
#[case("january", 1)]
#[case("JANUARY", 1)]
#[case("January", 1)]
#[case("jAnUaRy", 1)]
#[case("december", 12)]
#[case("DECEMBER", 12)]
fn month_full_names_case_insensitive(#[case] utterance: &str, #[case] expected_number: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
}

// -------------------------------------------------------------------------
// Month from standard 3-letter abbreviations
// -------------------------------------------------------------------------

#[rstest]
#[case("Jan", 1, MonthName::January)]
#[case("Feb", 2, MonthName::February)]
#[case("Mar", 3, MonthName::March)]
#[case("Apr", 4, MonthName::April)]
#[case("May", 5, MonthName::May)]
#[case("Jun", 6, MonthName::June)]
#[case("Jul", 7, MonthName::July)]
#[case("Aug", 8, MonthName::August)]
#[case("Sep", 9, MonthName::September)]
#[case("Oct", 10, MonthName::October)]
#[case("Nov", 11, MonthName::November)]
#[case("Dec", 12, MonthName::December)]
fn month_three_letter_abbreviations(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
}

/// Abbreviations should be case-insensitive.
#[rstest]
#[case("jan", 1)]
#[case("JAN", 1)]
#[case("Jan", 1)]
#[case("dec", 12)]
#[case("DEC", 12)]
fn month_abbreviations_case_insensitive(#[case] utterance: &str, #[case] expected_number: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
}

// -------------------------------------------------------------------------
// Month from unambiguous prefix matching
// -------------------------------------------------------------------------

/// Unambiguous prefixes longer than 3 chars should match.
/// e.g. "Janu" -> January, "Febr" -> February, "Sept" -> September
#[rstest]
#[case("Janu", 1, MonthName::January)]
#[case("Janua", 1, MonthName::January)]
#[case("Januar", 1, MonthName::January)]
#[case("Febr", 2, MonthName::February)]
#[case("Febru", 2, MonthName::February)]
#[case("Sept", 9, MonthName::September)]
#[case("Septe", 9, MonthName::September)]
#[case("Novem", 11, MonthName::November)]
#[case("Decem", 12, MonthName::December)]
fn month_unambiguous_prefix(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
}

// -------------------------------------------------------------------------
// Month from fuzzy / misspelled names (Levenshtein distance)
// -------------------------------------------------------------------------

/// Common misspellings should be resolved to the correct month.
#[rstest]
#[case("Januray", 1, MonthName::January)]
#[case("Janurary", 1, MonthName::January)]
#[case("Feburary", 2, MonthName::February)]
#[case("Febuary", 2, MonthName::February)]
#[case("Mrach", 3, MonthName::March)]
#[case("Apirl", 4, MonthName::April)]
#[case("Agust", 8, MonthName::August)]
#[case("Augst", 8, MonthName::August)]
#[case("Setpember", 9, MonthName::September)]
#[case("Septmber", 9, MonthName::September)]
#[case("Ocotber", 10, MonthName::October)]
#[case("Novmber", 11, MonthName::November)]
#[case("Decmber", 12, MonthName::December)]
fn month_misspellings(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
}

/// A string too far from any month name should not match.
#[rstest]
#[case("Xyzember")]
#[case("Foo")]
#[case("Abcde")]
fn month_too_far_misspelling_not_found(#[case] utterance: &str) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert!(result.month.number.is_not_found());
    assert!(result.month.name.is_not_found());
}

// -------------------------------------------------------------------------
// Month in surrounding text
// -------------------------------------------------------------------------

/// Month names embedded in natural language should still be extracted.
#[rstest]
#[case("it was in January", 1)]
#[case("happened during March", 3)]
#[case("the month of June", 6)]
#[case("sometime in Nov", 11)]
fn month_in_surrounding_text(#[case] utterance: &str, #[case] expected_number: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
}

// -------------------------------------------------------------------------
// Month with default value configured
// -------------------------------------------------------------------------

/// When month is not found and a default is configured, return Defaulted.
#[test]
fn month_not_found_returns_defaulted_when_configured() {
    let config = Config {
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::Yes,
            default: Some(6),
            ..Default::default()
        },
        year: YearConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config("no month here 42", config);
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Defaulted(6));
}

/// When month is not found and no default is configured, return NotFound.
#[test]
fn month_not_found_returns_not_found_when_no_default() {
    let input = input_with_config("no month here 42", month_only_config());
    let result = extract(input);

    assert!(result.month.number.is_not_found());
}

// -------------------------------------------------------------------------
// Edge cases
// -------------------------------------------------------------------------

/// Empty input returns NotFound.
#[test]
fn month_empty_input() {
    let input = input_with_config("", month_only_config());
    let result = extract(input);

    assert!(result.month.number.is_not_found());
}

/// Month abbreviation with a dot (e.g. "Jan.") should still match.
#[rstest]
#[case("Jan.", 1)]
#[case("Feb.", 2)]
#[case("Dec.", 12)]
fn month_abbreviation_with_dot(#[case] utterance: &str, #[case] expected_number: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_number));
}
