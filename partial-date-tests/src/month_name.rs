// Tests for MonthName conversions: TryFrom<u8> and TryFrom<&str>.

use partial_date::models::{MonthName, MonthNameError};
use rstest::rstest;

// -------------------------------------------------------------------------
// TryFrom<u8>
// -------------------------------------------------------------------------

/// All twelve valid month numbers map to the correct variant.
#[rstest]
#[case(1, MonthName::January)]
#[case(2, MonthName::February)]
#[case(3, MonthName::March)]
#[case(4, MonthName::April)]
#[case(5, MonthName::May)]
#[case(6, MonthName::June)]
#[case(7, MonthName::July)]
#[case(8, MonthName::August)]
#[case(9, MonthName::September)]
#[case(10, MonthName::October)]
#[case(11, MonthName::November)]
#[case(12, MonthName::December)]
fn try_from_u8_valid(#[case] n: u8, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(n), Ok(expected));
}

/// Numbers outside 1–12 produce NumberOutOfRange.
#[rstest]
#[case(0)]
#[case(13)]
#[case(99)]
#[case(255)]
fn try_from_u8_out_of_range(#[case] n: u8) {
    assert_eq!(
        MonthName::try_from(n),
        Err(MonthNameError::NumberOutOfRange(n))
    );
}

// -------------------------------------------------------------------------
// TryFrom<&str> — full names
// -------------------------------------------------------------------------

/// All twelve full month names, correctly capitalised, are recognised.
#[rstest]
#[case("January", MonthName::January)]
#[case("February", MonthName::February)]
#[case("March", MonthName::March)]
#[case("April", MonthName::April)]
#[case("May", MonthName::May)]
#[case("June", MonthName::June)]
#[case("July", MonthName::July)]
#[case("August", MonthName::August)]
#[case("September", MonthName::September)]
#[case("October", MonthName::October)]
#[case("November", MonthName::November)]
#[case("December", MonthName::December)]
fn try_from_str_full_name(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

/// We should also be able to handle cases where the month is misspelled, up to a certain threshold.
#[rstest]
#[case("Jauary", MonthName::January)]
#[case("Febuary", MonthName::February)]
#[case("Marsh", MonthName::March)]
#[case("Apriil", MonthName::April)]
#[case("Mey", MonthName::May)]
#[case("Juune", MonthName::June)]
#[case("Juli", MonthName::July)]
#[case("Agust", MonthName::August)]
#[case("Septeber", MonthName::September)]
#[case("Ocober", MonthName::October)]
#[case("novmber", MonthName::November)]
#[case("Decemer", MonthName::December)]
fn try_from_str_full_name_misspelled(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

/// Full month names are case-insensitive.
#[rstest]
#[case("january", MonthName::January)]
#[case("JANUARY", MonthName::January)]
#[case("jAnUaRy", MonthName::January)]
#[case("DECEMBER", MonthName::December)]
#[case("december", MonthName::December)]
fn try_from_str_full_name_case_insensitive(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

// -------------------------------------------------------------------------
// TryFrom<&str> — 3-letter abbreviations
// -------------------------------------------------------------------------

/// All standard 3-letter abbreviations are recognised.
#[rstest]
#[case("Jan", MonthName::January)]
#[case("Feb", MonthName::February)]
#[case("Mar", MonthName::March)]
#[case("Apr", MonthName::April)]
#[case("Jun", MonthName::June)]
#[case("Jul", MonthName::July)]
#[case("Aug", MonthName::August)]
#[case("Sep", MonthName::September)]
#[case("Oct", MonthName::October)]
#[case("Nov", MonthName::November)]
#[case("Dec", MonthName::December)]
fn try_from_str_abbreviation(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

/// 3-letter abbreviations are case-insensitive.
#[rstest]
#[case("jan", MonthName::January)]
#[case("JAN", MonthName::January)]
#[case("dec", MonthName::December)]
#[case("DEC", MonthName::December)]
fn try_from_str_abbreviation_case_insensitive(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

// -------------------------------------------------------------------------
// TryFrom<&str> — unambiguous prefixes (≥ 4 chars)
// -------------------------------------------------------------------------

/// Unambiguous prefixes of 4 or more characters resolve to the correct month.
#[rstest]
#[case("Janu", MonthName::January)]
#[case("Janua", MonthName::January)]
#[case("Januar", MonthName::January)]
#[case("Febr", MonthName::February)]
#[case("Febru", MonthName::February)]
#[case("Sept", MonthName::September)]
#[case("Septe", MonthName::September)]
#[case("Novem", MonthName::November)]
#[case("Decem", MonthName::December)]
#[case("Octo", MonthName::October)]
fn try_from_str_unambiguous_prefix(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

// -------------------------------------------------------------------------
// TryFrom<&str> — trailing dot stripped
// -------------------------------------------------------------------------

/// A trailing dot is stripped before matching, so "Jan." resolves correctly.
#[rstest]
#[case("Jan.", MonthName::January)]
#[case("Feb.", MonthName::February)]
#[case("Dec.", MonthName::December)]
#[case("October.", MonthName::October)]
fn try_from_str_trailing_dot(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

// -------------------------------------------------------------------------
// TryFrom<&str> — numeric strings
// -------------------------------------------------------------------------

/// Digit-only strings are parsed as month numbers.
#[rstest]
#[case("1", MonthName::January)]
#[case("01", MonthName::January)]
#[case("6", MonthName::June)]
#[case("06", MonthName::June)]
#[case("12", MonthName::December)]
fn try_from_str_numeric(#[case] s: &str, #[case] expected: MonthName) {
    assert_eq!(MonthName::try_from(s), Ok(expected));
}

/// Numeric strings outside 1–12 produce NumberOutOfRange.
#[rstest]
#[case("0")]
#[case("13")]
#[case("99")]
fn try_from_str_numeric_out_of_range(#[case] s: &str) {
    assert!(matches!(
        MonthName::try_from(s),
        Err(MonthNameError::NumberOutOfRange(_))
    ));
}

// -------------------------------------------------------------------------
// TryFrom<&str> — error cases
// -------------------------------------------------------------------------

/// Alphabetic strings that don't match any month return UnrecognisedName.
#[rstest]
#[case("Xyz")]
#[case("Foo")]
#[case("Abcde")]
#[case("No")]
#[case("Friday")]
fn try_from_str_unrecognised_name(#[case] s: &str) {
    assert_eq!(
        MonthName::try_from(s),
        Err(MonthNameError::UnrecognisedName)
    );
}

/// Mixed alphanumeric strings (neither pure alpha nor pure digit) return
/// NotAMonth.
#[rstest]
#[case("5x")]
#[case("jan2")]
#[case("12th")]
fn try_from_str_not_a_month(#[case] s: &str) {
    assert_eq!(MonthName::try_from(s), Err(MonthNameError::NotAMonth));
}

/// An empty string returns NotAMonth.
#[test]
fn try_from_str_empty() {
    assert_eq!(MonthName::try_from(""), Err(MonthNameError::NotAMonth));
}

// -------------------------------------------------------------------------
// number()
// -------------------------------------------------------------------------

/// MonthName::number() returns the correct calendar number for every variant.
#[rstest]
#[case(MonthName::January, 1)]
#[case(MonthName::February, 2)]
#[case(MonthName::March, 3)]
#[case(MonthName::April, 4)]
#[case(MonthName::May, 5)]
#[case(MonthName::June, 6)]
#[case(MonthName::July, 7)]
#[case(MonthName::August, 8)]
#[case(MonthName::September, 9)]
#[case(MonthName::October, 10)]
#[case(MonthName::November, 11)]
#[case(MonthName::December, 12)]
fn number_method(#[case] month: MonthName, #[case] expected: u8) {
    assert_eq!(month.number(), expected);
}

/// TryFrom<u8> and number() are inverse operations across the whole range.
#[test]
fn try_from_u8_and_number_are_inverses() {
    for n in 1u8..=12 {
        let month = MonthName::try_from(n).unwrap();
        assert_eq!(month.number(), n);
    }
}
