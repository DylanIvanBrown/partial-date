// Tests where a partial date is provided (fewer than 3 components present in
// the input). Covers: numeric DD/MM, DD/MM/YY, MM/DD ambiguity, natural
// language month names, ordinal days, default application, IsExpected hints,
// separators, and edge cases.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Numeric DD/MM — two components, no year
// -------------------------------------------------------------------------

/// Day-first order with two-component input: day and month extracted, year NotFound.
#[rstest]
#[case("12/06", 12, 6)]
#[case("01/01", 1, 1)]
#[case("31/12", 31, 12)]
#[case("28/02", 28, 2)]
#[case("15/06", 15, 6)]
fn partial_dmy_order_day_and_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Month-first order with two-component input: month and day swapped.
#[rstest]
#[case("12/06", 6, 12)]
#[case("06/15", 15, 6)]
#[case("01/28", 28, 1)]
fn partial_mdy_order_day_and_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_mdy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Unambiguous two-component dates — order irrelevant
// -------------------------------------------------------------------------

/// When the first value exceeds 12 it can only be a day, making the split
/// unambiguous regardless of the configured component order.
#[rstest]
#[case(order_dmy(), "31/06", 31, 6)]
#[case(order_mdy(), "31/06", 31, 6)]
#[case(order_dmy(), "29/11", 29, 11)]
#[case(order_mdy(), "29/11", 29, 11)]
#[case(order_dmy(), "25/03", 25, 3)]
#[case(order_mdy(), "25/03", 25, 3)]
#[case(order_dmy(), "30/09", 30, 9)]
#[case(order_mdy(), "30/09", 30, 9)]
fn partial_unambiguous_large_day(
    #[case] order: ComponentOrder,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order)));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Numeric DD/MM with different separators
// -------------------------------------------------------------------------

/// Partial dates should be extractable regardless of the separator used.
#[rstest]
#[case("15/06")]
#[case("15-06")]
#[case("15.06")]
#[case("15 06")]
#[case("15,06")]
fn partial_day_month_various_separators(#[case] utterance: &str) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(15));
    assert_eq!(result.month.number, Extracted::Found(6));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Numeric DD/YYYY — day and year, no month
// -------------------------------------------------------------------------

/// When only day and year are present, month should be NotFound (no default).
#[rstest]
#[case("15 2024", 15, 2024)]
#[case("01 2000", 1, 2000)]
#[case("31 1999", 31, 1999)]
fn partial_day_and_year_no_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Natural language — month name only
// -------------------------------------------------------------------------

/// All 12 month names alone: only month extracted.
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
fn partial_month_name_only(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

/// 3-letter abbreviations alone: only month extracted.
#[rstest]
#[case("Jan", 1, MonthName::January)]
#[case("Feb", 2, MonthName::February)]
#[case("Mar", 3, MonthName::March)]
#[case("Apr", 4, MonthName::April)]
#[case("Jun", 6, MonthName::June)]
#[case("Jul", 7, MonthName::July)]
#[case("Aug", 8, MonthName::August)]
#[case("Sep", 9, MonthName::September)]
#[case("Oct", 10, MonthName::October)]
#[case("Nov", 11, MonthName::November)]
#[case("Dec", 12, MonthName::December)]
fn partial_month_abbreviation_only(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Natural language — month name + year
// -------------------------------------------------------------------------

/// "Month YYYY" — month name and year extracted, day NotFound.
#[rstest]
#[case("January 2024", 1, MonthName::January, 2024)]
#[case("June 2024", 6, MonthName::June, 2024)]
#[case("December 1999", 12, MonthName::December, 1999)]
#[case("March 2000", 3, MonthName::March, 2000)]
#[case("Sep 2025", 9, MonthName::September, 2025)]
#[case("Feb 2020", 2, MonthName::February, 2020)]
fn partial_month_name_and_year(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Natural language — day + month name
// -------------------------------------------------------------------------

/// Bare number + month name: day and month extracted, year NotFound.
#[rstest]
#[case("1 January", 1, 1, MonthName::January)]
#[case("12 June", 12, 6, MonthName::June)]
#[case("15 March", 15, 3, MonthName::March)]
#[case("28 February", 28, 2, MonthName::February)]
#[case("31 December", 31, 12, MonthName::December)]
#[case("3 September", 3, 9, MonthName::September)]
#[case("20 November", 20, 11, MonthName::November)]
fn partial_bare_day_and_month_name(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

/// Ordinal day + month name: day and month extracted, year NotFound.
#[rstest]
#[case("1st January", 1, 1, MonthName::January)]
#[case("2nd February", 2, 2, MonthName::February)]
#[case("3rd March", 3, 3, MonthName::March)]
#[case("15th June", 15, 6, MonthName::June)]
#[case("21st August", 21, 8, MonthName::August)]
#[case("22nd October", 22, 10, MonthName::October)]
#[case("23rd November", 23, 11, MonthName::November)]
#[case("31st December", 31, 12, MonthName::December)]
fn partial_ordinal_day_and_month_name(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

/// "the Nth of Month" natural language form.
#[rstest]
#[case("the 1st of January", 1, 1, MonthName::January)]
#[case("the 3rd of March", 3, 3, MonthName::March)]
#[case("the 15th of June", 15, 6, MonthName::June)]
#[case("the 25th of December", 25, 12, MonthName::December)]
fn partial_the_nth_of_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Natural language — year only
// -------------------------------------------------------------------------

/// A bare four-digit year alone: only year extracted.
#[rstest]
#[case("2024", 2024)]
#[case("2000", 2000)]
#[case("1999", 1999)]
#[case("1900", 1900)]
#[case("2025", 2025)]
#[case("2010", 2010)]
fn partial_year_only(#[case] utterance: &str, #[case] expected_year: i32) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Natural language — ordinal day only
// -------------------------------------------------------------------------

/// A bare ordinal alone: only day extracted.
#[rstest]
#[case("1st", 1)]
#[case("2nd", 2)]
#[case("3rd", 3)]
#[case("15th", 15)]
#[case("16th", 16)]
#[case("18th", 18)]
#[case("21st", 21)]
#[case("31st", 31)]
fn partial_ordinal_day_only(#[case] utterance: &str, #[case] expected_day: u8) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Defaults: year default applied when year missing
// -------------------------------------------------------------------------

/// Numeric DD/MM with a configured default year: year should be Defaulted.
#[rstest]
#[case(order_dmy(), "12/06", 12, 6, 2025)]
#[case(order_dmy(), "01/01", 1, 1, 2000)]
#[case(order_dmy(), "31/12", 31, 12, 1999)]
#[case(order_mdy(), "06/12", 12, 6, 2025)]
fn partial_numeric_with_default_year(
    #[case] order: ComponentOrder,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] default_year: i32,
) {
    let config = Config {
        component_order: order,
        year: YearConfig {
            default: Some(default_year),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Defaulted(default_year));
}

/// Natural language month + day with a configured default year.
#[rstest]
#[case("12 June", 12, 6, MonthName::June, 2025)]
#[case("1st January", 1, 1, MonthName::January, 2000)]
#[case("15th March", 15, 3, MonthName::March, 2024)]
#[case("28 February", 28, 2, MonthName::February, 2020)]
#[case("31 December", 31, 12, MonthName::December, 1999)]
fn partial_natural_language_with_default_year(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
    #[case] default_year: i32,
) {
    let config = Config {
        year: YearConfig {
            default: Some(default_year),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Defaulted(default_year));
}

/// Month name alone with a configured default year: Defaulted year applied.
#[rstest]
#[case("January", 1, MonthName::January, 2025)]
#[case("June", 6, MonthName::June, 2025)]
#[case("December", 12, MonthName::December, 2000)]
#[case("Sep", 9, MonthName::September, 1999)]
fn partial_month_name_only_with_default_year(
    #[case] utterance: &str,
    #[case] expected_number: u8,
    #[case] expected_name: MonthName,
    #[case] default_year: i32,
) {
    let config = Config {
        year: YearConfig {
            default: Some(default_year),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_number));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Defaulted(default_year));
}

// -------------------------------------------------------------------------
// Defaults: month default applied when month missing
// -------------------------------------------------------------------------

/// Day and year present, no month — month should be Defaulted.
#[rstest]
#[case("15 2024", 15, 1, 2024)]
#[case("01 2000", 1, 6, 2000)]
#[case("31 1999", 31, 12, 1999)]
fn partial_day_and_year_with_default_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] default_month: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: order_dmy(),
        month: MonthConfig {
            default: Some(default_month),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Defaulted(default_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Defaults: day default applied when day missing
// -------------------------------------------------------------------------

/// Month name + year with a configured default day: day should be Defaulted.
#[rstest]
#[case("June 2024", 6, MonthName::June, 2024, 1)]
#[case("December 1999", 12, MonthName::December, 1999, 15)]
#[case("Jan 2025", 1, MonthName::January, 2025, 1)]
fn partial_month_and_year_with_default_day(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
    #[case] expected_year: i32,
    #[case] default_day: u8,
) {
    let config = Config {
        day: DayConfig {
            default: Some(default_day),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Defaulted(default_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Defaults: all three defaults applied when input is empty
// -------------------------------------------------------------------------

/// Empty input with all defaults configured — all three fields Defaulted.
#[rstest]
#[case(1, 1, 2025)]
#[case(15, 6, 2000)]
#[case(31, 12, 1999)]
fn partial_empty_input_with_all_defaults(
    #[case] default_day: u8,
    #[case] default_month: u8,
    #[case] default_year: i32,
) {
    let config = Config {
        day: DayConfig {
            default: Some(default_day),
            ..Default::default()
        },
        month: MonthConfig {
            default: Some(default_month),
            ..Default::default()
        },
        year: YearConfig {
            default: Some(default_year),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config("", config));

    assert_eq!(result.day.value, Extracted::Defaulted(default_day));
    assert_eq!(result.month.number, Extracted::Defaulted(default_month));
    assert_eq!(result.year.value, Extracted::Defaulted(default_year));
}

// -------------------------------------------------------------------------
// Defaults: month-only input with day and year defaults
// -------------------------------------------------------------------------

/// Month name with day and year defaults: month Found, others Defaulted.
#[rstest]
#[case("January", 1, MonthName::January, 1, 2025)]
#[case("June", 6, MonthName::June, 15, 2024)]
#[case("December", 12, MonthName::December, 1, 2000)]
#[case("Sep", 9, MonthName::September, 1, 1999)]
fn partial_month_only_with_day_and_year_defaults(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
    #[case] default_day: u8,
    #[case] default_year: i32,
) {
    let config = Config {
        day: DayConfig {
            default: Some(default_day),
            ..Default::default()
        },
        year: YearConfig {
            default: Some(default_year),
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Defaulted(default_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Defaulted(default_year));
}

// -------------------------------------------------------------------------
// No defaults: missing components always NotFound
// -------------------------------------------------------------------------

/// Empty input with no defaults — all NotFound.
#[test]
fn partial_empty_input_no_defaults() {
    let result = extract(input(""));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// IsExpected hints
// -------------------------------------------------------------------------

/// IsExpected::No on year: a number that would normally be a year should not
/// be extracted when year is explicitly not expected.
#[rstest]
#[case(order_dmy(), "12/06", 12, 6)]
#[case(order_mdy(), "06/12", 12, 6)]
#[case(order_dmy(), "15/03", 15, 3)]
fn partial_year_not_expected(
    #[case] order: ComponentOrder,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let config = Config {
        day: DayConfig {
            expected: IsExpected::Yes,
            ..Default::default()
        },
        month: MonthConfig {
            expected: IsExpected::Yes,
            ..Default::default()
        },
        year: YearConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        component_order: order,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// No-space adjacent day and month name
// -------------------------------------------------------------------------

/// Day number immediately adjacent to a month name (no separator).
/// The extractor should still identify both components.
#[rstest]
#[case("19october", 19, 10, MonthName::October)]
#[case("9july", 9, 7, MonthName::July)]
fn partial_day_adjacent_to_month_name(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

/// Month name immediately adjacent to a day number (no separator).
#[rstest]
#[case("August7", 7, 8, MonthName::August)]
fn partial_month_name_adjacent_to_day(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Day + month name with various noise words
// -------------------------------------------------------------------------

/// A bare number + month name where surrounding noise is present.
#[rstest]
#[case("25 October", 25, 10, MonthName::October)]
fn partial_day_and_month_name_with_noise(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Inputs containing only noise — no date components
// -------------------------------------------------------------------------

/// Input that contains no date components at all should return NotFound
/// for everything.
#[rstest]
#[case("No")]
#[case("Friday")]
#[case("hello world")]
fn partial_no_date_components(#[case] utterance: &str) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// "Friday 31" — "Friday" is a noise word; day 31 should still be extracted.
#[test]
fn partial_day_with_weekday_noise() {
    let result = extract(input("Friday 31"));

    assert_eq!(result.day.value, Extracted::Found(31));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// IsExpected::No on day: a value that would be a day should not be returned.
#[rstest]
#[case("June 2024", 6, MonthName::June, 2024)]
#[case("December 1999", 12, MonthName::December, 1999)]
fn partial_day_not_expected(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_name: MonthName,
    #[case] expected_year: i32,
) {
    let config = Config {
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Stronger edge cases: messy partial dates, boundary conditions
// -------------------------------------------------------------------------

/// Partial date with excessive spacing around components.
#[rstest]
#[case("  15  /  06  ", 15, 6)]
#[case(" 01 - 12 ", 1, 12)]
fn partial_excessive_spacing(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Partial date with mixed separators (day/month).
#[rstest]
#[case("15/06", 15, 6)]
#[case("15-06", 15, 6)]
#[case("15.06", 15, 6)]
#[case("15 06", 15, 6)]
fn partial_mixed_separators_day_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Day with ordinal + month name (no year).
#[rstest]
#[case("25th December", 25, 12, MonthName::December)]
#[case("1st January", 1, 1, MonthName::January)]
#[case("31st October", 31, 10, MonthName::October)]
fn partial_ordinal_day_with_month_name(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_month_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_month_name));
    assert!(result.year.value.is_not_found());
}

/// Fuzzy month name + year (no day).
#[rstest]
#[case("Decmber 2024", 12, MonthName::December, 2024)]
#[case("Januray 1999", 1, MonthName::January, 1999)]
#[case("Ocotber 2025", 10, MonthName::October, 2025)]
fn partial_fuzzy_month_with_year(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_month_name: MonthName,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_month_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Boundary day/month combinations.
#[rstest]
#[case("01/01", 1, 1)] // First day of first month
#[case("31/12", 31, 12)] // Last day of last month
#[case("29/02", 29, 2)] // Leap day
#[case("15/06", 15, 6)] // Mid-year
fn partial_boundary_day_month_combos(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Day/month with surrounding natural language text.
#[rstest]
#[case("on 15/06 there", 15, 6)]
#[case("date is 31-12 please", 31, 12)]
#[case("was 01.01 when", 1, 1)]
fn partial_day_month_surrounded_by_text(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Month name abbreviated + day (no year).
#[rstest]
#[case("Dec 25", 25, 12, MonthName::December)]
#[case("Jan 1", 1, 1, MonthName::January)]
#[case("Jun 15", 15, 6, MonthName::June)]
fn partial_abbreviated_month_with_day(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_month_name: MonthName,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_month_name));
    assert!(result.year.value.is_not_found());
}

/// Year only (2-digit and 4-digit variants).
#[rstest]
#[case("2024", 2024)] // 4-digit
#[case("24", 2024)] // 2-digit (sliding window)
#[case("99", 1999)] // 2-digit boundary (1900s)
#[case("00", 2000)] // 2-digit boundary (2000s)
fn partial_year_only_two_digit_and_four_digit(#[case] utterance: &str, #[case] expected_year: i32) {
    let result = extract(input_with_config(utterance, year_only_config()));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Month only (numeric and name variants).
#[rstest]
#[case("06", 6)]
#[case("12", 12)]
#[case("December", 12)]
#[case("Jan", 1)]
fn partial_month_only(#[case] utterance: &str, #[case] expected_month: u8) {
    let result = extract(input_with_config(utterance, month_only_config()));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Day only (numeric and ordinal variants).
#[rstest]
#[case("15", 15)]
#[case("01", 1)]
#[case("31", 31)]
#[case("15th", 15)]
#[case("1st", 1)]
fn partial_day_only(#[case] utterance: &str, #[case] expected_day: u8) {
    let result = extract(input_with_config(utterance, day_only_config()));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Partial date with leading zeros in all components.
#[rstest]
#[case("01/01", 1, 1)]
#[case("07/08", 7, 8)]
#[case("09/09", 9, 9)]
fn partial_leading_zeros(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Month name with case variations + year (no day).
#[rstest]
#[case("DECEMBER 2024", 12, MonthName::December, 2024)]
#[case("january 1999", 1, MonthName::January, 1999)]
#[case("jUnE 2025", 6, MonthName::June, 2025)]
fn partial_case_variant_month_with_year(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_month_name: MonthName,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.month.name, Extracted::Found(expected_month_name));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Three values provided but only 2 are valid (1 component becomes NotFound)
// -------------------------------------------------------------------------

/// Three numeric values where only 2 are valid: 22 22 22 in DMY order.
/// - First 22 = day (valid)
/// - Second 22 = month (invalid - 22 is not a valid month)
/// - Third 22 = year (valid when interpreted as 2022 with 2-digit expansion)
/// Result: day found, month not found, year found.
#[rstest]
#[case("22 22 22", 22, 2022)] // day month(invalid) year
#[case("15 32 2024", 15, 2024)] // day month(32 invalid) year
#[case("31 31 2020", 31, 2020)] // day month(31 invalid) year
#[case("05 13 2025", 5, 2025)] // day month(13 invalid) year
fn partial_three_values_invalid_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.month.name.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Three numeric values where only 2 are valid: first and third.
/// - First value = day (valid)
/// - Second value = month (but is actually > 31, so could be invalid day if position-swapped)
/// - Third value = year (valid 4-digit year)
/// Day is extracted in first position, middle value (too large for month) is rejected,
/// year is extracted from third position.
#[rstest]
#[case("07 45 2019", 7, 2019)] // day month(45 invalid) year
#[case("12 99 2000", 12, 2000)] // day month(99 invalid) year
#[case("28 50 2023", 28, 2023)] // day month(50 invalid) year
#[case("03 100 2021", 3, 2021)] // day month(100 invalid) year
fn partial_three_values_middle_invalid(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Three numeric values in MDY order where middle (day) value is invalid.
/// - First = month (valid)
/// - Second = day (invalid - > 31)
/// - Third = year (valid)
/// Result: month found, day not found, year found.
#[rstest]
#[case("06 32 2024", 6, 2024)] // month day(32 invalid) year
#[case("12 45 2020", 12, 2020)] // month day(45 invalid) year
#[case("01 99 2023", 1, 2023)] // month day(99 invalid) year
#[case("09 50 2025", 9, 2025)] // month day(50 invalid) year
fn partial_three_values_mdy_invalid_day(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Three numeric values where the first (day) position is invalid (> 31).
/// - First = day (invalid - > 31)
/// - Second = month (valid)
/// - Third = year (valid)
/// Result: day not found, month found, year found.
#[rstest]
#[case("32 06 2024", 6, 2024)] // day(32 invalid) month year
#[case("50 12 2020", 12, 2020)] // day(50 invalid) month year
#[case("99 03 2023", 3, 2023)] // day(99 invalid) month year
#[case("45 01 2025", 1, 2025)] // day(45 invalid) month year
fn partial_three_values_first_invalid_day(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Three numeric values where the third (year) position is invalid (only 2 digits, interpreted as day).
/// When a 2-digit year is in the third position with default sliding window expansion,
/// it may be interpreted as another day value instead of year in some contexts.
#[rstest]
#[case("15 06 32", 15, 6, 32)] // day month year(32 as 2-digit not valid year expansion)
#[case("12 08 25", 12, 8, 25)] // day month year(25 as year)
#[case("05 10 99", 5, 10, 99)] // day month year(99 as year)
fn partial_three_values_with_two_digit_year(
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

// -------------------------------------------------------------------------
// Only 1 of 3 values is valid (2 components become NotFound)
// -------------------------------------------------------------------------

/// Three numeric values where only the middle (month) is valid.
/// - First = invalid as day (> 31)
/// - Second = valid month
/// - Third = invalid as year (single digit too small, and not typical year)
/// Result: day not found, month found, year not found.
#[rstest]
#[case("45 06 2", 6)] // invalid(45) month(06) invalid(2)
#[case("99 12 1", 12)] // invalid(99) month(12) invalid(1)
#[case("50 03 3", 3)] // invalid(50) month(03) invalid(3)
fn partial_three_values_only_month_valid(#[case] utterance: &str, #[case] expected_month: u8) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Three numeric values where only the first (day) is valid.
/// - First = valid day
/// - Second = invalid month (> 12)
/// - Third = invalid year (single digit, too small for typical year interpretation)
/// Result: day found, month not found, year not found.
#[rstest]
#[case("15 45 2", 15)] // day(15) invalid(45) invalid(2)
#[case("03 99 1", 3)] // day(03) invalid(99) invalid(1)
#[case("28 50 3", 28)] // day(28) invalid(50) invalid(3)
#[case("31 13 0", 31)] // day(31) invalid(13) invalid(0)
fn partial_three_values_only_day_valid(#[case] utterance: &str, #[case] expected_day: u8) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Three numeric values where only the last (year) is valid.
/// - First = invalid as day (> 31)
/// - Second = invalid as month (> 12)
/// - Third = valid 4-digit year
/// Result: day not found, month not found, year found.
#[rstest]
#[case("45 50 2024", 2024)] // invalid(45) invalid(50) year(2024)
#[case("99 13 2000", 2000)] // invalid(99) invalid(13) year(2000)
#[case("32 99 2023", 2023)] // invalid(32) invalid(99) year(2023)
#[case("50 100 2025", 2025)] // invalid(50) invalid(100) year(2025)
fn partial_three_values_only_year_valid(#[case] utterance: &str, #[case] expected_year: i32) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Two 4-digit year values with one other component (only 1 of 3 valid)
// -------------------------------------------------------------------------

/// Three values where two are 4-digit years and one other component.
/// Since only the first valid number can be used, only the first 4-digit year is extracted.
/// The second 4-digit "year" is either rejected or treated as invalid for other positions.
/// The third component is also invalid or too large.
/// Result: only one year found, other components not found.
#[rstest]
#[case("2024 2023 45", 2024)] // year(2024) year(2023 invalid here) invalid(45)
#[case("2000 2020 99", 2000)] // year(2000) year(2020 invalid) invalid(99)
#[case("2015 2010 50", 2015)] // year(2015) year(2010 invalid) invalid(50)
#[case("2019 1999 13", 2019)] // year(2019) year(1999 invalid) month(13 invalid)
fn partial_three_values_two_years_one_invalid(#[case] utterance: &str, #[case] expected_year: i32) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Three values where two are 4-digit years: positions don't match date component order.
/// For example, in DMY order, a 4-digit year should be in position 3,
/// but if it's in position 1 or 2, only the valid-positioned year counts.
/// - First = 4-digit year (invalid position for day in DMY)
/// - Second = 4-digit year (invalid position for month in DMY)
/// - Third = small number (invalid as year, could be day)
/// Result: day found (from third), month not found, year not found (both years in wrong positions).
#[rstest]
#[case("2024 2023 15", 15)] // year(2024 invalid pos) year(2023 invalid pos) day(15)
#[case("2000 2020 28", 28)] // year(2000 invalid) year(2020 invalid) day(28)
#[case("2015 2010 03", 3)] // year(2015 invalid) year(2010 invalid) day(03)
fn partial_three_values_two_4digit_years_wrong_position(
    #[case] utterance: &str,
    #[case] expected_day: u8,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Three values where two are 4-digit numbers (both valid years) but only one can be used.
/// When two valid 4-digit numbers appear, the first one encountered (in component order)
/// claims the year position, and the second is rejected.
/// - Position 1 (day): 2024 (4-digit, won't fit as day)
/// - Position 2 (month): 2020 (4-digit, won't fit as month)
/// - Position 3 (year): 2023 (4-digit, valid year)
/// Only the third (correctly positioned) year is used. First two are rejected.
/// Result: year found (from position 3), day and month not found.
#[rstest]
#[case("2024 2020 2023", 2023)] // 4-digit(2024) 4-digit(2020) year(2023)
#[case("2000 2015 2010", 2010)] // 4-digit(2000) 4-digit(2015) year(2010)
#[case("2019 1999 2025", 2025)] // 4-digit(2019) 4-digit(1999) year(2025)
fn partial_three_values_all_4digit_numbers(#[case] utterance: &str, #[case] expected_year: i32) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// MDY Format Tests: Invalid Month in First Position
// -------------------------------------------------------------------------

/// MDY format where first position (month) is invalid (> 12).
/// - First = month position (invalid - > 12)
/// - Second = day position (valid)
/// - Third = year position (valid)
/// Result: month not found, day found, year found.
#[rstest]
#[case("13 15 2024", 15, 2024)] // month(13 invalid) day(15) year(2024)
#[case("32 28 2020", 28, 2020)] // month(32 invalid) day(28) year(2020)
#[case("99 05 2023", 5, 2023)] // month(99 invalid) day(05) year(2023)
#[case("45 31 2025", 31, 2025)] // month(45 invalid) day(31) year(2025)
#[case("50 12 2019", 12, 2019)] // month(50 invalid) day(12) year(2019)
fn partial_mdy_invalid_month_first_position(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// MDY format with invalid month (> 12) and invalid day (> 31).
/// - First = month (invalid - > 12)
/// - Second = day (invalid - > 31)
/// - Third = year (valid)
/// Result: month and day not found, year found.
#[rstest]
#[case("13 32 2024", 2024)] // month(13) day(32) year(2024)
#[case("99 50 2020", 2020)] // month(99) day(50) year(2020)
#[case("45 45 2023", 2023)] // month(45) day(45) year(2023)
#[case("50 99 2025", 2025)] // month(50) day(99) year(2025)
fn partial_mdy_invalid_month_and_day(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert!(result.day.value.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// MDY format where only month is invalid, others valid.
/// - First = month (invalid)
/// - Second = day (valid)
/// - Third = year (valid)
/// Result: only month not found.
#[rstest]
#[case("22 15 2024", 15, 2024)] // month(22>12) day(15) year(2024)
#[case("14 28 2020", 28, 2020)] // month(14>12) day(28) year(2020)
#[case("25 03 2023", 3, 2023)] // month(25>12) day(03) year(2023)
#[case("100 31 2025", 31, 2025)] // month(100>12) day(31) year(2025)
fn partial_mdy_invalid_month_only(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// MDY format with two components where month is invalid.
/// - First = month (invalid)
/// - Second = day (valid)
/// Result: only day extracted, no year.
#[rstest]
#[case("13 15", 15)] // month(13) day(15)
#[case("32 28", 28)] // month(32) day(28)
#[case("99 05", 5)] // month(99) day(05)
#[case("45 31", 31)] // month(45) day(31)
fn partial_mdy_two_values_invalid_month(#[case] utterance: &str, #[case] expected_day: u8) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// MYD Format Tests: Invalid Month in First Position
// -------------------------------------------------------------------------

/// MYD format where first position (month) is invalid (> 12).
/// - First = month position (invalid - > 12)
/// - Second = year position (valid)
/// - Third = day position (valid)
/// Result: month not found, year found, day found.
#[rstest]
#[case("13 2024 15", 2024, 15)] // month(13 invalid) year(2024) day(15)
#[case("32 2020 28", 2020, 28)] // month(32 invalid) year(2020) day(28)
#[case("99 2023 05", 2023, 5)] // month(99 invalid) year(2023) day(05)
#[case("45 2025 31", 2025, 31)] // month(45 invalid) year(2025) day(31)
#[case("50 2019 12", 2019, 12)] // month(50 invalid) year(2019) day(12)
fn partial_myd_invalid_month_first_position(
    #[case] utterance: &str,
    #[case] expected_year: i32,
    #[case] expected_day: u8,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// MYD format with invalid month (> 12) and invalid day (> 31).
/// - First = month (invalid - > 12)
/// - Second = year (valid)
/// - Third = day (invalid - > 31)
/// Result: month and day not found, year found.
#[rstest]
#[case("13 2024 32", 2024)] // month(13) year(2024) day(32)
#[case("99 2020 50", 2020)] // month(99) year(2020) day(50)
#[case("45 2023 45", 2023)] // month(45) year(2023) day(45)
#[case("50 2025 99", 2025)] // month(50) year(2025) day(99)
fn partial_myd_invalid_month_and_day(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.day.value.is_not_found());
}

/// MYD format where only month is invalid, others valid.
/// - First = month (invalid)
/// - Second = year (valid)
/// - Third = day (valid)
/// Result: only month not found.
#[rstest]
#[case("22 2024 15", 2024, 15)] // month(22>12) year(2024) day(15)
#[case("14 2020 28", 2020, 28)] // month(14>12) year(2020) day(28)
#[case("25 2023 03", 2023, 3)] // month(25>12) year(2023) day(03)
#[case("100 2025 31", 2025, 31)] // month(100>12) year(2025) day(31)
fn partial_myd_invalid_month_only(
    #[case] utterance: &str,
    #[case] expected_year: i32,
    #[case] expected_day: u8,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// MYD format with two components where month is invalid.
/// - First = month (invalid)
/// - Second = year (valid)
/// Result: only year extracted, no day.
#[rstest]
#[case("13 2024", 2024)] // month(13) year(2024)
#[case("32 2020", 2020)] // month(32) year(2020)
#[case("99 2023", 2023)] // month(99) year(2023)
#[case("45 2025", 2025)] // month(45) year(2025)
fn partial_myd_two_values_invalid_month(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.day.value.is_not_found());
}

// -------------------------------------------------------------------------
// Mixed Component Order: Invalid Month in Different Positions
// -------------------------------------------------------------------------

/// YMD format where month (middle position) is invalid.
/// In YMD order: position 1 = year, position 2 = month, position 3 = day.
/// - First = year (valid 4-digit)
/// - Second = month (invalid - > 12)
/// - Third = day (valid)
/// Result: year found, month not found, day found.
#[rstest]
#[case("2024 13 15", 2024, 15)] // year(2024) month(13) day(15)
#[case("2020 32 28", 2020, 28)] // year(2020) month(32) day(28)
#[case("2023 99 05", 2023, 5)] // year(2023) month(99) day(05)
#[case("2025 45 31", 2025, 31)] // year(2025) month(45) day(31)
fn partial_ymd_invalid_month_middle_position(
    #[case] utterance: &str,
    #[case] expected_year: i32,
    #[case] expected_day: u8,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Year,
            second: DateComponent::Month,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// DYM format where month (last position) is invalid.
/// In DYM order: position 1 = day, position 2 = year, position 3 = month.
/// - First = day (valid)
/// - Second = year (valid 4-digit)
/// - Third = month (invalid - > 12)
/// Result: day found, year found, month not found.
#[rstest]
#[case("15 2024 13", 15, 2024)] // day(15) year(2024) month(13)
#[case("28 2020 32", 28, 2020)] // day(28) year(2020) month(32)
#[case("05 2023 99", 5, 2023)] // day(05) year(2023) month(99)
#[case("31 2025 45", 31, 2025)] // day(31) year(2025) month(45)
fn partial_dym_invalid_month_last_position(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Day,
            second: DateComponent::Year,
            third: DateComponent::Month,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.month.number.is_not_found());
}

/// YDM format where month (last position) is invalid.
/// In YDM order: position 1 = year, position 2 = day, position 3 = month.
/// - First = year (valid 4-digit)
/// - Second = day (valid)
/// - Third = month (invalid - > 12)
/// Result: year found, day found, month not found.
#[rstest]
#[case("2024 15 13", 2024, 15)] // year(2024) day(15) month(13)
#[case("2020 28 32", 2020, 28)] // year(2020) day(28) month(32)
#[case("2023 05 99", 2023, 5)] // year(2023) day(05) month(99)
#[case("2025 31 45", 2025, 31)] // year(2025) day(31) month(45)
fn partial_ydm_invalid_month_last_position(
    #[case] utterance: &str,
    #[case] expected_year: i32,
    #[case] expected_day: u8,
) {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Year,
            second: DateComponent::Day,
            third: DateComponent::Month,
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
}
