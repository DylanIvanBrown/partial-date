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
/// Three values where the middle token is invalid as month but may still be
/// used as a day.  Cases where the first token is NOT also a valid month —
/// the only month candidate in the 3-slot would come from the first token
/// (if valid), meaning the middle (invalid month) token becomes the day.
///
/// For "22 22 22": first=22 (day only, >12 for month), so month stays NotFound.
/// For "15 32 2024": 32 only valid as year (>31 for day), so day=15, month=NotFound.
/// For "31 31 2020": same — 31 not valid month.
#[rstest]
#[case("22 22 22", 22, 2022)] // day=22, month=NotFound (22>12), year=2022
#[case("15 32 2024", 15, 2024)] // day=15, month=NotFound (32>12), year=2024
#[case("31 31 2020", 31, 2020)] // day=31, month=NotFound (31>12), year=2020
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

/// "05 13 2025" DMY: 5 is valid day AND month; 13 is valid day but not month.
/// The 3-slot assignment {day=13, month=5, year=2025} beats the 2-slot
/// {day=5, year=2025} on component count.  Month IS found (5=May).
#[test]
fn partial_three_values_first_token_fills_month_from_day_position() {
    let result = extract(input("05 13 2025"));

    assert_eq!(result.day.value, Extracted::Found(13));
    assert_eq!(result.month.number, Extracted::Found(5));
    assert_eq!(result.year.value, Extracted::Found(2025));
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

/// Three numeric values where the third position contains a 2-digit year.
/// The default SlidingWindow expansion applies: 00–49 → 2000–2049, 50–99 → 1950–1999.
#[rstest]
#[case("15 06 32", 15, 6, 2032)] // day=15, month=6, year=32→2032
#[case("12 08 25", 12, 8, 2025)] // day=12, month=8, year=25→2025
#[case("05 10 99", 5, 10, 1999)] // day=5, month=10, year=99→1999
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

/// Three numeric values with an invalid-as-day first token, a valid month, and
/// a small third token.  The first token (> 31) is valid only as a 2-digit
/// year; the third token (single digit) is valid only as a day (default config
/// has single_digit_year_expansion off).  The interpreter fills all three
/// slots: year from the first token, month from the second, day from the third.
#[rstest]
#[case("45 06 2", 2, 6, 2045)] // year=2045, month=6, day=2
#[case("99 12 1", 1, 12, 1999)] // year=1999, month=12, day=1
#[case("50 03 3", 3, 3, 1950)] // year=1950, month=3, day=3
fn partial_three_values_year_month_day_from_invalid_first(
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

/// Three numeric values with a valid day first, an invalid-as-month middle
/// token, and a small third token.  The middle token (> 12) is valid only as a
/// year; the third token (single digit) is valid as day or month but not year
/// (single_digit_year_expansion is off by default).  The interpreter fills
/// day + month + year using all three tokens.
#[rstest]
#[case("15 45 2", 15, 2, 2045)] // day=15, month=2, year=2045
#[case("03 99 1", 3, 1, 1999)] // day=3, month=1, year=1999
#[case("28 50 3", 28, 3, 1950)] // day=28, month=3, year=1950
fn partial_three_values_day_month_year_from_invalid_second(
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

/// `"31 13 0"` DMY — 31 is day; 13 is day or year (→2013); 0 with digit_count=1
/// cannot be day (0 < 1), month (0 < 1), or year (single_digit_year_expansion
/// is off).  Only tokens 31 and 13 can participate.  Best 2-slot:
/// {day=31, year=2013} wins on agreement (31 at DMY position Day +1).
#[test]
fn partial_three_values_day_and_year_from_zero_value() {
    let result = extract(input("31 13 0"));

    assert_eq!(result.day.value, Extracted::Found(31));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(2013));
}

/// Three numeric values where only the last 4-digit token is a year, and the
/// first two are invalid for day and month.  The first two tokens may still be
/// valid as years (via 2-digit expansion), so the 4-digit token at the correct
/// position wins.
#[rstest]
#[case("45 50 2024", 2024)] // 45→year-only, 50→year-only, 2024→year: 1-slot year=2024
#[case("32 99 2023", 2023)] // same pattern
#[case("50 100 2025", 2025)] // 100 is a 3-digit year candidate; 2025 wins positionally
fn partial_three_values_only_year_valid(#[case] utterance: &str, #[case] expected_year: i32) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// `"99 13 2000"` DMY — 99 is year-only (→1999), 13 is day or year (→2013),
/// 2000 is year-only.  Best 2-slot: {day=13@idx1, year=2000@idx2} wins on
/// positional agreement (2000 is at the Year position in DMY).
#[test]
fn partial_three_values_year_with_day_from_ambiguous_second() {
    let result = extract(input("99 13 2000"));

    assert_eq!(result.day.value, Extracted::Found(13));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(2000));
}

// -------------------------------------------------------------------------
// Two 4-digit year values with one other component (only 1 of 3 valid)
// -------------------------------------------------------------------------

// TODO: Decide if we do want to allow for returning a value when multiple values
// are provided for one type. Should this instead return all years or should we
// have config option that allows us to return this or return an invalid result
// in some form.
/// Three values that are all valid only as years (two 4-digit, one 2-digit).
/// No day or month can be extracted.  When all tokens can only fill the year
/// slot the positional-agreement tiebreaker applies: the token at the position
/// prescribed as Year by the configured component order wins.  With the default
/// DMY order the third position is Year, so the 2-digit token at position 3
/// (expanded via the sliding-window rule) is selected.
#[rstest]
#[case("2024 2023 45", 2045)] // pos3=45 → 2045 wins on positional agreement
#[case("2000 2020 99", 1999)] // pos3=99 → 1999 wins on positional agreement
#[case("2015 2010 50", 1950)] // pos3=50 → 1950 wins on positional agreement
fn partial_three_values_all_year_only_positional_tiebreaker(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Three values: two 4-digit years and one 2-digit token that can be both a
/// day and a year.  The best assignment fills two slots (year + day) rather
/// than one, so a 2-slot assignment is preferred over a 1-slot one.
/// The 2-digit token becomes the day; the earlier 4-digit year becomes the
/// year (earliest-token tiebreaker among the equal 2-slot options).
#[test]
fn partial_three_values_two_years_and_ambiguous_day_year() {
    // "2019 1999 13" DMY: 13 can be Day(13) or Year(2013).
    // Best: 2-slot assignment {year=2019, day=13} — year uses earliest
    // 4-digit token (idx 0), day uses the ambiguous token (idx 2).
    let result = extract(input("2019 1999 13"));

    assert_eq!(result.day.value, Extracted::Found(13));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(2019));
}

/// Three values: two 4-digit years at positions 1 and 2 (wrong for DMY day/month
/// slots), plus a small day-sized value at position 3.  The interpreter ignores
/// positional mismatches for 4-digit years — they can only be years regardless of
/// position.  The earliest 4-digit token is selected as year; the small token
/// fills the day slot.
#[rstest]
#[case("2024 2023 15", 15, 2024)] // day=15, year=2024 (earliest 4-digit)
#[case("2000 2020 28", 28, 2000)]
#[case("2015 2010 03", 3, 2015)]
fn partial_three_values_two_4digit_years_wrong_position(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_year: i32,
) {
    let result = extract(input(utterance));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(expected_year));
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

/// "13 32 2024" MDY: 13 fails Month (>12) but is a valid day (1–31).
/// The library uses 13 as day; 32 can only be year (2032) but 2024 is
/// at the correct Year position and wins.  Month remains NotFound.
#[test]
fn partial_mdy_invalid_month_13_invalid_day_32() {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config("13 32 2024", config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.day.value, Extracted::Found(13));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

/// When both the month and day tokens are too large for day/month AND the
/// year is a 4-digit token, only the year is extracted.
#[rstest]
#[case("99 50 2020", 2020)]
#[case("45 45 2023", 2023)]
#[case("50 99 2025", 2025)]
fn partial_mdy_both_invalid_only_year(#[case] utterance: &str, #[case] expected_year: i32) {
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

/// "22 15 2024" and "14 28 2020" MDY: invalid month, second token also >12
/// so no month candidate.  Result: month=NotFound, day from second, year=2024.
#[rstest]
#[case("22 15 2024", 15, 2024)]
#[case("14 28 2020", 28, 2020)]
fn partial_mdy_invalid_month_second_not_month(
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

/// "25 03 2023" MDY: 25 fails Month (>12) but is a valid day.  3 is a valid
/// month AND day.  The 3-slot assignment {day=25, month=3, year=2023} beats
/// the 2-slot {day=3, year=2023} on component count, so month IS found (3).
#[test]
fn partial_mdy_invalid_month_second_is_valid_month() {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config("25 03 2023", config));

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(3));
    assert_eq!(result.year.value, Extracted::Found(2023));
}

/// "100 31 2025" MDY: 100 is a 3-digit literal year; 31 is a valid day;
/// 2025 is the 4-digit year.  Month=NotFound.
#[test]
fn partial_mdy_three_digit_invalid_month() {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        },
        ..Default::default()
    };
    let result = extract(input_with_config("100 31 2025", config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.day.value, Extracted::Found(31));
    assert_eq!(result.year.value, Extracted::Found(2025));
}

/// MDY format with two tokens where the first (prescribed Month) is invalid
/// as a month.  The library allows the first token to fill the Year slot since
/// it is a valid 2-digit year, giving a 2-slot result {day, year}.
/// The 2-slot assignment beats the 1-slot {day only} on component count.
#[rstest]
#[case("13 15", 15, 2013)] // 13→year=2013, 15→day
#[case("32 28", 28, 2032)] // 32→year=2032, 28→day
#[case("99 05", 5, 1999)] // 99→year=1999, 5→day
#[case("45 31", 31, 2045)] // 45→year=2045, 31→day
fn partial_mdy_two_values_invalid_month(
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

/// MYD format with invalid month and invalid day (> 31).
/// The first token (month position) fails Month but may be a valid day.
/// When first token is a valid day (e.g. 13), the library uses it as day.
/// When first token is >31 (not a valid day either), day=NotFound.
#[rstest]
#[case("99 2020 50", 2020)] // 99>31 as day, 50>31 as day → day=NotFound
#[case("45 2023 45", 2023)] // 45>31 as day → day=NotFound
#[case("50 2025 99", 2025)] // 50>31 → day=NotFound
fn partial_myd_invalid_month_and_both_day_positions_invalid(
    #[case] utterance: &str,
    #[case] expected_year: i32,
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
    assert!(result.day.value.is_not_found());
}

/// "13 2024 32" MYD: 13 fails Month (>12) but is a valid day (1–31).
/// The library uses 13 as day, 2024 as year. 32 can't be day (>31).
#[test]
fn partial_myd_invalid_month_13_invalid_day_32() {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config("13 2024 32", config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(2024));
    assert_eq!(result.day.value, Extracted::Found(13));
}

/// MYD format where the first token is invalid as month.
/// Cases where the third token is NOT also a valid month: month stays NotFound.
#[rstest]
#[case("22 2024 15", 2024, 15)] // 15 not a valid month (>12) — month=NotFound
#[case("14 2020 28", 2020, 28)] // 28 not a valid month — month=NotFound
#[case("100 2025 31", 2025, 31)] // 100 3-digit, 31 not month — month=NotFound
fn partial_myd_invalid_month_third_not_month(
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

/// "25 2023 03" MYD: 25 fails Month (>12) but is a valid day.  3 is a valid
/// month AND day.  The 3-slot {day=25, month=3, year=2023} beats 2-slot on
/// component count, so month IS found (3) from the third token.
#[test]
fn partial_myd_invalid_month_third_is_valid_month() {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config("25 2023 03", config));

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(3));
    assert_eq!(result.year.value, Extracted::Found(2023));
}

/// MYD format with two tokens where the first fails its Month slot.
/// When the first token is a valid day (e.g. 13, 32→no, 99→no, 45→no),
/// the library uses it as day in a 2-slot {day, year} assignment.
/// When it is >31 (not a valid day either), only year is extracted.
#[rstest]
#[case("32 2020", 2020)] // 32>31 — only year
#[case("99 2023", 2023)] // 99>31 — only year
#[case("45 2025", 2025)] // 45>31 — only year
fn partial_myd_two_values_invalid_month_no_day(
    #[case] utterance: &str,
    #[case] expected_year: i32,
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
    assert!(result.day.value.is_not_found());
}

/// "13 2024" MYD: 13 fails Month (>12) but is a valid day.  The library uses
/// 13 as day in a 2-slot {day=13, year=2024} assignment.
#[test]
fn partial_myd_two_values_invalid_month_first_becomes_day() {
    let config = Config {
        component_order: ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        },
        ..Default::default()
    };
    let result = extract(input_with_config("13 2024", config));

    assert!(result.month.number.is_not_found());
    assert_eq!(result.year.value, Extracted::Found(2024));
    assert_eq!(result.day.value, Extracted::Found(13));
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
