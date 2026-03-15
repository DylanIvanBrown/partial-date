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

/// DD-first formats with "12/06": day=12, month=6, year=NotFound.
#[rstest]
#[case(Format::DDMMYY, "12/06", 12, 6)]
#[case(Format::DDMMYYYY, "12/06", 12, 6)]
#[case(Format::DDMMYY, "01/01", 1, 1)]
#[case(Format::DDMMYYYY, "01/01", 1, 1)]
#[case(Format::DDMMYY, "31/12", 31, 12)]
#[case(Format::DDMMYYYY, "31/12", 31, 12)]
#[case(Format::DDMMYY, "28/02", 28, 2)]
#[case(Format::DDMMYYYY, "28/02", 28, 2)]
#[case(Format::DDMMYY, "15/06", 15, 6)]
#[case(Format::DDMMYYYY, "15/06", 15, 6)]
fn partial_dd_first_day_and_month(
    #[case] format: Format,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_format(format)));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// MM-first formats with numeric input: month and day swapped.
#[rstest]
#[case(Format::MMDDYY, "12/06", 6, 12)]
#[case(Format::MMDDYYYY, "12/06", 6, 12)]
#[case(Format::MMDDYY, "06/15", 15, 6)]
#[case(Format::MMDDYYYY, "06/15", 15, 6)]
#[case(Format::MMDDYY, "01/28", 28, 1)]
#[case(Format::MMDDYYYY, "01/28", 28, 1)]
fn partial_mm_first_day_and_month(
    #[case] format: Format,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_format(format)));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

// -------------------------------------------------------------------------
// Unambiguous two-component dates — format irrelevant
// -------------------------------------------------------------------------

/// When the first value exceeds 12 it can only be a day, making the split
/// unambiguous regardless of the configured format.
#[rstest]
#[case(Format::DDMMYY, "31/06", 31, 6)]
#[case(Format::MMDDYY, "31/06", 31, 6)]
#[case(Format::DDMMYYYY, "31/06", 31, 6)]
#[case(Format::MMDDYYYY, "31/06", 31, 6)]
#[case(Format::DDMMYY, "29/11", 29, 11)]
#[case(Format::MMDDYY, "29/11", 29, 11)]
#[case(Format::DDMMYY, "25/03", 25, 3)]
#[case(Format::MMDDYY, "25/03", 25, 3)]
#[case(Format::DDMMYY, "30/09", 30, 9)]
#[case(Format::MMDDYY, "30/09", 30, 9)]
fn partial_unambiguous_large_day(
    #[case] format: Format,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let result = extract(input_with_config(utterance, config_with_format(format)));

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
    let result = extract(input_with_config(
        utterance,
        config_with_format(Format::DDMMYY),
    ));

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
    let result = extract(input_with_config(
        utterance,
        config_with_format(Format::DDMMYYYY),
    ));

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
#[case(Format::DDMMYY, "12/06", 12, 6, 2025)]
#[case(Format::DDMMYYYY, "12/06", 12, 6, 2025)]
#[case(Format::DDMMYY, "01/01", 1, 1, 2000)]
#[case(Format::DDMMYYYY, "31/12", 31, 12, 1999)]
#[case(Format::MMDDYY, "06/12", 12, 6, 2025)]
fn partial_numeric_with_default_year(
    #[case] format: Format,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] default_year: i32,
) {
    let config = Config {
        primary_format: format,
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
        primary_format: Format::DDMMYYYY,
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
#[case(Format::DDMMYYYY, "12/06", 12, 6)]
#[case(Format::MMDDYYYY, "06/12", 12, 6)]
#[case(Format::DDMMYY, "15/03", 15, 3)]
fn partial_year_not_expected(
    #[case] format: Format,
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
        primary_format: format,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
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
