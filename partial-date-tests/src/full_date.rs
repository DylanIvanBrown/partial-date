// Tests for full date extraction using different component orders.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Day → Month → Year order (formerly DDMMYYYY / DDMMYY)
// -------------------------------------------------------------------------

#[rstest]
#[case("25/12/2024", 25, 12, 2024)]
#[case("01/01/2000", 1, 1, 2000)]
#[case("31/01/1999", 31, 1, 1999)]
#[case("15/06/2025", 15, 6, 2025)]
#[case("28/02/2024", 28, 2, 2024)]
fn dmy_valid_slash(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Day → Month → Year with dash separator.
#[rstest]
#[case("25-12-2024", 25, 12, 2024)]
#[case("01-06-1999", 1, 6, 1999)]
fn dmy_valid_dash(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Day → Month → Year with dot separator.
#[rstest]
#[case("25.12.2024", 25, 12, 2024)]
#[case("01.06.1999", 1, 6, 1999)]
fn dmy_valid_dot(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Day → Month → Year with space separator.
#[rstest]
#[case("25 12 2024", 25, 12, 2024)]
#[case("01 06 1999", 1, 6, 1999)]
fn dmy_valid_space(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Day → Month → Year with two-digit year (sliding window expansion).
#[rstest]
#[case("25/12/24", 25, 12, 2024)]
#[case("01/01/00", 1, 1, 2000)]
#[case("31/12/99", 31, 12, 1999)]
#[case("15/06/25", 15, 6, 2025)]
fn dmy_two_digit_year(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Month → Day → Year order (formerly MMDDYYYY / MMDDYY)
// -------------------------------------------------------------------------

#[rstest]
#[case("12/25/2024", 25, 12, 2024)]
#[case("01/01/2000", 1, 1, 2000)]
#[case("06/15/2025", 15, 6, 2025)]
#[case("02/28/2024", 28, 2, 2024)]
fn mdy_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_mdy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Month → Day → Year with two-digit year (sliding window expansion).
#[rstest]
#[case("12/25/24", 25, 12, 2024)]
#[case("01/01/00", 1, 1, 2000)]
#[case("06/15/25", 15, 6, 2025)]
fn mdy_two_digit_year(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_mdy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Year → Month → Day order (formerly YYYYMMDD / YYMMDD)
// -------------------------------------------------------------------------

#[rstest]
#[case("2024/12/25", 25, 12, 2024)]
#[case("2000/01/01", 1, 1, 2000)]
#[case("2025/06/15", 15, 6, 2025)]
fn ymd_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_ymd())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Year → Month → Day with two-digit year (sliding window expansion).
#[rstest]
#[case("24/12/25", 25, 12, 2024)]
#[case("00/01/01", 1, 1, 2000)]
#[case("99/12/31", 31, 12, 1999)]
fn ymd_two_digit_year(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_ymd())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Year → Day → Month order (formerly YYYYDDMM / YYDDMM)
// -------------------------------------------------------------------------

#[rstest]
#[case("2024/25/12", 25, 12, 2024)]
#[case("2000/01/01", 1, 1, 2000)]
#[case("2025/15/06", 15, 6, 2025)]
fn ydm_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_ydm())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Year → Day → Month with two-digit year (sliding window expansion).
#[rstest]
#[case("24/25/12", 25, 12, 2024)]
#[case("00/01/01", 1, 1, 2000)]
#[case("99/31/12", 31, 12, 1999)]
fn ydm_two_digit_year(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_ydm())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Month → Year → Day order (formerly MMYYDD / MMYYYYDD)
// -------------------------------------------------------------------------

#[rstest]
#[case("12/24/25", 25, 12, 2024)]
#[case("01/00/01", 1, 1, 2000)]
fn myd_two_digit_year(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_myd())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

#[rstest]
#[case("12/2024/25", 25, 12, 2024)]
#[case("01/2000/01", 1, 1, 2000)]
fn myd_valid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_myd())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Ambiguous cases: component order determines interpretation
// -------------------------------------------------------------------------

/// The same input string produces different day/month assignments depending
/// on component order.
#[rstest]
#[case("01/06/24", order_dmy(), 1, 6, 2024)]
#[case("01/06/24", order_mdy(), 6, 1, 2024)]
#[case("05/03/2022", order_dmy(), 5, 3, 2022)]
#[case("05/03/2022", order_mdy(), 3, 5, 2022)]
fn ambiguous_date_order_determines_interpretation(
    #[case] utterance: &str,
    #[case] order: ComponentOrder,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order)));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Unambiguous cases: component order doesn't matter
// -------------------------------------------------------------------------

/// "31/12/19" — 31 can only be a day; unambiguous regardless of order.
#[rstest]
#[case(order_dmy())]
#[case(order_mdy())]
fn unambiguous_day_too_large_for_month(#[case] order: ComponentOrder) {
    let result = extract(input_with_config("31/12/19", config_with_order(order)));

    assert_eq!(result.day.value, Extracted::Found(31));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.year.value, Extracted::Found(2019));
}

/// "25/06/2024" — 25 can only be a day (>12); unambiguous regardless of order.
#[rstest]
#[case(order_dmy())]
#[case(order_mdy())]
fn unambiguous_first_value_too_large_for_month(#[case] order: ComponentOrder) {
    let result = extract(input_with_config("25/06/2024", config_with_order(order)));

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
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert!(result.day.value.is_not_found());
}

/// Invalid month values — the month should not be found.
#[rstest]
#[case("01/13/2024")]
#[case("15/00/2024")]
fn full_date_invalid_month(#[case] utterance: &str) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert!(result.month.number.is_not_found());
}

// -------------------------------------------------------------------------
// No separator (concatenated) dates
// -------------------------------------------------------------------------

/// Dates without separators, like "25122024" in Day → Month → Year order.
/// The `no_separator` flag must be enabled for concatenated date parsing.
#[rstest]
#[case("25122024", order_dmy(), 25, 12, 2024)]
#[case("12252024", order_mdy(), 25, 12, 2024)]
#[case("20241225", order_ymd(), 25, 12, 2024)]
fn full_date_no_separator(
    #[case] utterance: &str,
    #[case] order: ComponentOrder,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: order,
        no_separator: true,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

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
    let result = extract(input(utterance));

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
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// "25th of December 2024" — day with ordinal, month name, year.
#[test]
fn full_date_ordinal_day_with_month_name() {
    let result = extract(input("25th of December 2024"));

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.month.name, Extracted::Found(MonthName::December));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

// -------------------------------------------------------------------------
// Full date with noise words and non-standard phrasing
// -------------------------------------------------------------------------

/// Full dates expressed in natural spoken/written language with surrounding
/// noise text. Day, month, and year should all be extracted.
#[rstest]
#[case("I was born in 2014 October 19", 19, 10, MonthName::October, 2014)]
#[case("October 26 2012", 26, 10, MonthName::October, 2012)]
#[case("31 July  2014", 31, 7, MonthName::July, 2014)]
#[case("30 December 2014", 30, 12, MonthName::December, 2014)]
#[case("25th  October  2025", 25, 10, MonthName::October, 2025)]
#[case("January27 2013", 27, 1, MonthName::January, 2013)]
fn full_date_natural_language_with_noise(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_month_name: MonthName,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_month_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Full date with mixed separators within a single string.
#[rstest]
#[case("19th  October,2015", 19, 10, MonthName::October, 2015)]
#[case("18/6. 2013", 18, 6, MonthName::June, 2013)]
fn full_date_mixed_separators(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_month_name: MonthName,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_month_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Two-digit year embedded in natural language: "8 October 11" →
/// day=8, month=October, year=2011 (sliding window: 11 → 2011).
#[test]
fn full_date_natural_language_two_digit_year() {
    let result = extract(input("8 October 11"));

    assert_eq!(result.day.value, Extracted::Found(8));
    assert_eq!(result.month.number, Extracted::Found(10));
    assert_eq!(result.month.name, Extracted::Found(MonthName::October));
    assert_eq!(result.year.value, Extracted::Found(2011));
}

/// Noise characters interspersed in the utterance: "8 October >type 11" →
/// same as "8 October 11", noise ">type" is ignored.
#[test]
fn full_date_natural_language_with_noise_characters() {
    let result = extract(input("8 October >type 11"));

    assert_eq!(result.day.value, Extracted::Found(8));
    assert_eq!(result.month.number, Extracted::Found(10));
    assert_eq!(result.month.name, Extracted::Found(MonthName::October));
    assert_eq!(result.year.value, Extracted::Found(2011));
}

/// "31 December 2o14" — OCR-style typo where the digit `0` is replaced by
/// letter `o`. Year should be NotFound; day and month are still extracted.
#[test]
fn full_date_year_with_ocr_typo_not_found() {
    let result = extract(input("31 December  2o14"));

    assert_eq!(result.day.value, Extracted::Found(31));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.month.name, Extracted::Found(MonthName::December));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Edge cases
// -------------------------------------------------------------------------

/// Empty input returns NotFound for all.
#[test]
fn full_date_empty_input() {
    let result = extract(input_with_config("", config_with_order(order_dmy())));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}
