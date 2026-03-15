// Tests for full date extraction using different formats.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// DDMMYYYY format
// -------------------------------------------------------------------------

#[rstest]
#[case("25/12/2024", 25, 12, 2024)]
#[case("01/01/2000", 1, 1, 2000)]
#[case("31/01/1999", 31, 1, 1999)]
#[case("15/06/2025", 15, 6, 2025)]
#[case("28/02/2024", 28, 2, 2024)]
fn ddmmyyyy_valid_slash(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// DDMMYYYY with dash separator.
#[rstest]
#[case("25-12-2024", 25, 12, 2024)]
#[case("01-06-1999", 1, 6, 1999)]
fn ddmmyyyy_valid_dash(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// DDMMYYYY with dot separator.
#[rstest]
#[case("25.12.2024", 25, 12, 2024)]
#[case("01.06.1999", 1, 6, 1999)]
fn ddmmyyyy_valid_dot(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// DDMMYYYY with space separator.
#[rstest]
#[case("25 12 2024", 25, 12, 2024)]
#[case("01 06 1999", 1, 6, 1999)]
fn ddmmyyyy_valid_space(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// DDMMYY format
// -------------------------------------------------------------------------

#[rstest]
#[case("25/12/24", 25, 12, 2024)]
#[case("01/01/00", 1, 1, 2000)]
#[case("31/12/99", 31, 12, 1999)]
#[case("15/06/25", 15, 6, 2025)]
fn ddmmyy_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// MMDDYYYY format
// -------------------------------------------------------------------------

#[rstest]
#[case("12/25/2024", 25, 12, 2024)]
#[case("01/01/2000", 1, 1, 2000)]
#[case("06/15/2025", 15, 6, 2025)]
#[case("02/28/2024", 28, 2, 2024)]
fn mmddyyyy_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::MMDDYYYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// MMDDYY format
// -------------------------------------------------------------------------

#[rstest]
#[case("12/25/24", 25, 12, 2024)]
#[case("01/01/00", 1, 1, 2000)]
#[case("06/15/25", 15, 6, 2025)]
fn mmddyy_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::MMDDYY));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// YYYYMMDD format
// -------------------------------------------------------------------------

#[rstest]
#[case("2024/12/25", 25, 12, 2024)]
#[case("2000/01/01", 1, 1, 2000)]
#[case("2025/06/15", 15, 6, 2025)]
fn yyyymmdd_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::YYYYMMDD));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// YYYYDDMM format
// -------------------------------------------------------------------------

#[rstest]
#[case("2024/25/12", 25, 12, 2024)]
#[case("2000/01/01", 1, 1, 2000)]
#[case("2025/15/06", 15, 6, 2025)]
fn yyyyddmm_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::YYYYDDMM));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// YYMMDD format
// -------------------------------------------------------------------------

#[rstest]
#[case("24/12/25", 25, 12, 2024)]
#[case("00/01/01", 1, 1, 2000)]
#[case("99/12/31", 31, 12, 1999)]
fn yymmdd_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::YYMMDD));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// YYDDMM format
// -------------------------------------------------------------------------

#[rstest]
#[case("24/25/12", 25, 12, 2024)]
#[case("00/01/01", 1, 1, 2000)]
#[case("99/31/12", 31, 12, 1999)]
fn yyddmm_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::YYDDMM));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// MMYYDD format
// -------------------------------------------------------------------------

#[rstest]
#[case("12/24/25", 25, 12, 2024)]
#[case("01/00/01", 1, 1, 2000)]
fn mmyydd_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::MMYYDD));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// MMYYYYDD format
// -------------------------------------------------------------------------

#[rstest]
#[case("12/2024/25", 25, 12, 2024)]
#[case("01/2000/01", 1, 1, 2000)]
fn mmyyyydd_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(Format::MMYYYYDD));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Ambiguous cases: the format determines interpretation
// -------------------------------------------------------------------------

/// Ambiguous dates where the format determines interpretation.
/// The same input string produces different day/month assignments depending on format.
#[rstest]
#[case("01/06/24", Format::DDMMYY, 1, 6, 2024)]
#[case("01/06/24", Format::MMDDYY, 6, 1, 2024)]
#[case("05/03/2022", Format::DDMMYYYY, 5, 3, 2022)]
#[case("05/03/2022", Format::MMDDYYYY, 3, 5, 2022)]
fn ambiguous_date_format_determines_interpretation(
    #[case] utterance: &str,
    #[case] format: Format,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, config_with_format(format));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Unambiguous cases: format doesn't matter
// -------------------------------------------------------------------------

/// "31/12/19" — 31 can only be a day, 12 is a valid month, 19 is year.
/// Regardless of DD/MM or MM/DD format, the result should be the same
/// because 31 is too large to be a month.
#[rstest]
#[case(Format::DDMMYY)]
#[case(Format::MMDDYY)]
fn unambiguous_day_too_large_for_month(#[case] format: Format) {
    let input = input_with_config("31/12/19", config_with_format(format));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(31));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.year.value, Extracted::Found(2019));
}

/// "25/06/2024" — 25 can only be a day (>12), so format doesn't matter
/// for DD vs MM position disambiguation.
#[rstest]
#[case(Format::DDMMYYYY)]
#[case(Format::MMDDYYYY)]
fn unambiguous_first_value_too_large_for_month(#[case] format: Format) {
    let input = input_with_config("25/06/2024", config_with_format(format));
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(6));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

// -------------------------------------------------------------------------
// Invalid values in date components
// -------------------------------------------------------------------------

/// Invalid day values — the day should not be found.
#[rstest]
#[case("102/06/2024")]
#[case("00/06/2024")]
fn full_date_invalid_day(#[case] utterance: &str) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert!(result.day.value.is_not_found());
}

/// Invalid month values — the month should not be found.
#[rstest]
#[case("01/13/2024")]
#[case("15/00/2024")]
fn full_date_invalid_month(#[case] utterance: &str) {
    let input = input_with_config(utterance, config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert!(result.month.number.is_not_found());
}

// -------------------------------------------------------------------------
// No separator (concatenated) dates
// -------------------------------------------------------------------------

/// Dates without separators, like "25122024" in DDMMYYYY format.
#[rstest]
#[case("25122024", Format::DDMMYYYY, 25, 12, 2024)]
#[case("12252024", Format::MMDDYYYY, 25, 12, 2024)]
#[case("20241225", Format::YYYYMMDD, 25, 12, 2024)]
fn full_date_no_separator(
    #[case] utterance: &str,
    #[case] format: Format,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        primary_format: format,
        primary_separator: Separator::NoSeparator,
        ..Default::default()
    };
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Full date with natural language month
// -------------------------------------------------------------------------

/// "25 December 2024" — day, month name, year should all be extracted.
#[rstest]
#[case("25 December 2024", 25, 12, MonthName::December, 2024)]
#[case("1 January 2000", 1, 1, MonthName::January, 2000)]
#[case("15 Jun 2025", 15, 6, MonthName::June, 2025)]
#[case("28 Feb 2024", 28, 2, MonthName::February, 2024)]
fn full_date_natural_language(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
    #[case] expected_year: i32,
) {
    let input = input(utterance);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// "December 25, 2024" — month-first natural language.
#[rstest]
#[case("December 25, 2024", 25, 12, 2024)]
#[case("January 1, 2000", 1, 1, 2000)]
#[case("Jun 15, 2025", 15, 6, 2025)]
fn full_date_month_first_natural_language(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input(utterance);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// "25th of December 2024" — day with ordinal, month name, year.
#[test]
fn full_date_ordinal_day_with_month_name() {
    let input = input("25th of December 2024");
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.month.name, Extracted::Found(MonthName::December));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

// -------------------------------------------------------------------------
// Edge cases
// -------------------------------------------------------------------------

/// Empty input returns NotFound for all.
#[test]
fn full_date_empty_input() {
    let input = input_with_config("", config_with_format(Format::DDMMYYYY));
    let result = extract(input);

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}
