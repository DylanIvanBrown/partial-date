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

/// "15/00/2024" with DMY: 00 is not a valid month (below minimum of 1), so
/// month is NotFound. No other token can fill the month slot either (15 > 12,
/// 2024 is 4-digit). Result: day=15, month=NotFound, year=2024.
#[test]
fn full_date_invalid_month_zero() {
    let result = extract(input_with_config(
        "15/00/2024",
        config_with_order(order_dmy()),
    ));

    assert_eq!(result.day.value, Extracted::Found(15));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(2024));
}

/// "01/13/2024" with DMY: 13 is in the month position but is not a valid month.
/// The algorithm identifies that only token 01 can be a month (the sole value ≤ 12),
/// and reassigns: day=13, month=1, year=2024.
#[test]
fn full_date_invalid_month_position_recovered() {
    let result = extract(input_with_config(
        "01/13/2024",
        config_with_order(order_dmy()),
    ));

    assert_eq!(result.day.value, Extracted::Found(13));
    assert_eq!(result.month.number, Extracted::Found(1));
    assert_eq!(result.year.value, Extracted::Found(2024));
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
/// letter `o`. With `letter_o_substitution` enabled (the default), the
/// tokeniser recognises that `"2o14"` consists entirely of digits and the
/// letter O, substitutes O→0 before boundary splitting, and produces
/// `Numeric(2014, 4)`. The full date is therefore correctly resolved as
/// day=31, month=December, year=2014.
#[test]
fn full_date_year_with_ocr_typo_resolved() {
    let result = extract(input("31 December  2o14"));

    assert_eq!(result.day.value, Extracted::Found(31));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.month.name, Extracted::Found(MonthName::December));
    assert_eq!(result.year.value, Extracted::Found(2014));
}

/// With `letter_o_substitution` disabled, "31 December 2o14" cannot recover
/// the year from the OCR-corrupted token. The tokeniser splits "2o14" on the
/// digit-to-alpha boundary, producing Numeric(2,1) and discarding "o14" (not
/// a month name or number). With month anchored to December, the remaining
/// numerics (31,2) and (2,1) are assigned to the Day and Year slots.
/// Only (31,2) can be a Year (1-digit values are rejected as years); (31,2)
/// also fits Day. The algorithm places the only year-capable token as
/// Year=2031 and the remaining token as Day=2.
#[test]
fn full_date_year_with_ocr_typo_disabled_substitution() {
    let config = Config {
        letter_o_substitution: false,
        ..Default::default()
    };
    let result = extract(input_with_config("31 December  2o14", config));

    assert_eq!(result.day.value, Extracted::Found(2));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.month.name, Extracted::Found(MonthName::December));
    assert_eq!(result.year.value, Extracted::Found(2031));
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

// -------------------------------------------------------------------------
// Stronger edge cases: messy input, unusual formats
// -------------------------------------------------------------------------

/// Multiple extra spaces around date components should still parse correctly.
#[rstest]
#[case("25   /   12   /   2024", 25, 12, 2024)]
#[case("  1  /  1  /  2000  ", 1, 1, 2000)]
fn full_date_excessive_spacing(
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

/// Mixed separators within a single date (slashes, dashes, dots, spaces).
#[rstest]
#[case("25/12-2024", 25, 12, 2024)]
#[case("1.6 2025", 1, 6, 2025)]
#[case("31-12/99", 31, 12, 1999)]
fn full_date_mixed_separator_styles(
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

/// Ordinal day markers (st, nd, rd, th) with numeric month and year.
#[rstest]
#[case("25th 12 2024", 25, 12, 2024)]
#[case("1st 1 2000", 1, 1, 2000)]
#[case("31st 12 1999", 31, 12, 1999)]
fn full_date_ordinal_day_with_numeric_components(
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

/// Fuzzy month names (misspellings) with numeric day/year should still extract all.
#[rstest]
#[case("25 Decmber 2024", 25, 12, MonthName::December, 2024)]
#[case("1 Januray 2000", 1, 1, MonthName::January, 2000)]
#[case("31 Ocotber 1999", 31, 10, MonthName::October, 1999)]
#[case("15 Augst 2020", 15, 8, MonthName::August, 2020)]
fn full_date_fuzzy_month_name_with_numeric(
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

/// 3-letter abbreviated month with ordinal day.
#[rstest]
#[case("15th Dec 2024", 15, 12, MonthName::December, 2024)]
#[case("1st Jan 2000", 1, 1, MonthName::January, 2000)]
#[case("31st Oct 1999", 31, 10, MonthName::October, 1999)]
fn full_date_abbreviated_month_with_ordinal_day(
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

/// Natural language dates with embedded commas and extra text: "On 25, December 2024".
#[rstest]
#[case("On 25, December 2024", 25, 12, MonthName::December, 2024)]
#[case("Born 1 , January 1990", 1, 1, MonthName::January, 1990)]
fn full_date_with_embedded_commas_and_text(
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

/// Boundary dates: start of year, end of year, leap year.
#[rstest]
#[case("01/01/2024", 1, 1, 2024)]
#[case("31/12/2024", 31, 12, 2024)]
#[case("29/02/2024", 29, 2, 2024)] // Leap year
#[case("31/01/2000", 31, 1, 2000)]
fn full_date_boundary_dates(
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

/// Two-digit year at the boundary of sliding window (00, 49, 50, 99).
#[rstest]
#[case("15/06/00", 15, 6, 2000)]
#[case("15/06/49", 15, 6, 2049)]
#[case("15/06/50", 15, 6, 1950)]
#[case("15/06/99", 15, 6, 1999)]
fn full_date_two_digit_year_window_boundaries(
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

/// Year boundary cases (historical and far future).
#[rstest]
#[case("15/6/1800", 15, 6, 1800)]
#[case("15/6/2999", 15, 6, 2999)]
fn full_date_year_historical_and_future(
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

/// Case variations in abbreviated and full month names with natural language.
#[rstest]
#[case("25 DECEMBER 2024", 25, 12, MonthName::December, 2024)]
#[case("1 jAnUaRy 2000", 1, 1, MonthName::January, 2000)]
#[case("31 ocToBer 1999", 31, 10, MonthName::October, 1999)]
fn full_date_case_insensitive_month_names(
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

/// Unambiguous numeric dates with leading zeros (1-9 single digits with leading zero).
#[rstest]
#[case("01/02/2024", 1, 2, 2024)]
#[case("07/08/1999", 7, 8, 1999)]
#[case("09/09/2050", 9, 9, 2050)]
fn full_date_leading_zeros(
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

/// Dates with surrounding natural language noise: "Remind me on 25/12/2024 please".
#[rstest]
#[case("Remind me on 25/12/2024 please", 25, 12, 2024)]
#[case("The date is 01-06-2000 and then", 1, 6, 2000)]
#[case("Please note: 31.12.1999 is historic", 31, 12, 1999)]
fn full_date_surrounded_by_natural_language_noise(
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
