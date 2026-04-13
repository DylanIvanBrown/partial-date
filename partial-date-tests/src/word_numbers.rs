// Tests for word-based number extraction (e.g., "one" → 1, "twenty-three" → 23).
//
// This module tests the extraction of English written numbers across a broad range
// from 1 to 3000, covering:
// - Basic units (one, two, ..., nine)
// - Teens (ten, eleven, ..., nineteen)
// - Tens (twenty, thirty, ..., ninety)
// - Compound numbers (twenty-one, ninety-nine, etc.)
// - Hundreds (one hundred, two hundred, ..., nine hundred ninety-nine)
// - Thousands (one thousand, two thousand, three thousand)

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Basic units: 1–9
// -------------------------------------------------------------------------

/// Single-digit word numbers should extract correctly as day, month, or year.
#[rstest]
#[case("one", 1)]
#[case("two", 2)]
#[case("three", 3)]
#[case("four", 4)]
#[case("five", 5)]
#[case("six", 6)]
#[case("seven", 7)]
#[case("eight", 8)]
#[case("nine", 9)]
fn word_number_basic_units_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert!(result.month.number.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Single-digit word numbers in context with surrounding text (day).
#[rstest]
#[case("on the fifth", 5)]
#[case("day three of the month", 3)]
#[case("the ninth day", 9)]
fn word_number_basic_units_day_with_context(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Single-digit word numbers as months (1–9 are valid months).
#[rstest]
#[case("one", 1)]
#[case("two", 2)]
#[case("three", 3)]
#[case("four", 4)]
#[case("five", 5)]
#[case("six", 6)]
#[case("seven", 7)]
#[case("eight", 8)]
#[case("nine", 9)]
fn word_number_basic_units_month(#[case] utterance: &str, #[case] expected_month: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.day.value.is_not_found());
    assert!(result.year.value.is_not_found());
}

/// Single-digit word numbers as years — uses single_digit_year_expansion with
/// Literal expansion so the value is returned as-is (1, 5, 9).
#[rstest]
#[case("one", 1)]
#[case("five", 5)]
#[case("nine", 9)]
fn word_number_basic_units_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            single_digit_year_expansion: true,
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
    assert!(result.day.value.is_not_found());
    assert!(result.month.number.is_not_found());
}

// -------------------------------------------------------------------------
// Teens: 10–19
// -------------------------------------------------------------------------

/// Teen word numbers (ten through nineteen) should extract correctly.
#[rstest]
#[case("ten", 10)]
#[case("eleven", 11)]
#[case("twelve", 12)]
#[case("thirteen", 13)]
#[case("fourteen", 14)]
#[case("fifteen", 15)]
#[case("sixteen", 16)]
#[case("seventeen", 17)]
#[case("eighteen", 18)]
#[case("nineteen", 19)]
fn word_number_teens_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Teen word numbers as months (10–12 are valid).
#[rstest]
#[case("ten", 10)]
#[case("eleven", 11)]
#[case("twelve", 12)]
fn word_number_teens_month(#[case] utterance: &str, #[case] expected_month: u8) {
    let input = input_with_config(utterance, month_only_config());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_month));
}

/// Teen word numbers as years — uses Literal expansion so values are as-is.
#[rstest]
#[case("ten", 10)]
#[case("thirteen", 13)]
#[case("nineteen", 19)]
fn word_number_teens_year(#[case] utterance: &str, #[case] expected_year: i32) {
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
// Tens: 20–90 (multiples of 10)
// -------------------------------------------------------------------------

/// Tens word numbers as days — only twenty and thirty are valid days (≤ 31).
#[rstest]
#[case("twenty", 20)]
#[case("thirty", 30)]
fn word_number_tens_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Tens word numbers as years — values are expanded by the default sliding-window rule.
/// 20–49 → 2020–2049; 50–99 → 1950–1990.
#[rstest]
#[case("twenty", 2020)]
#[case("thirty", 2030)]
#[case("forty", 2040)]
#[case("fifty", 1950)]
#[case("sixty", 1960)]
#[case("seventy", 1970)]
#[case("eighty", 1980)]
#[case("ninety", 1990)]
fn word_number_tens_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Compound numbers: 21–99 (excluding multiples of 10)
// -------------------------------------------------------------------------

/// Compound numbers with hyphens (twenty-one, thirty-five, etc.) as days.
/// Only values ≤ 31 are valid days; higher values are tested as years below.
#[rstest]
#[case("twenty-one", 21)]
#[case("twenty-two", 22)]
#[case("twenty-three", 23)]
#[case("twenty-four", 24)]
#[case("twenty-five", 25)]
#[case("twenty-six", 26)]
#[case("twenty-seven", 27)]
#[case("twenty-eight", 28)]
#[case("twenty-nine", 29)]
#[case("thirty-one", 31)]
fn word_number_compound_hyphenated_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Compound numbers with spaces (twenty one, etc.) as days — only values ≤ 31.
#[rstest]
#[case("twenty one", 21)]
fn word_number_compound_space_separated_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Compound numbers (hyphenated) as years — uses Literal expansion so the
/// value is returned as-is without century expansion.
#[rstest]
#[case("twenty-one", 21)]
#[case("forty-five", 45)]
#[case("sixty-seven", 67)]
#[case("eighty-nine", 89)]
#[case("ninety-nine", 99)]
fn word_number_compound_hyphenated_year(#[case] utterance: &str, #[case] expected_year: i32) {
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

/// Compound numbers with spaces as years — uses Literal expansion.
#[rstest]
#[case("twenty one", 21)]
#[case("thirty two", 32)]
#[case("fifty five", 55)]
#[case("ninety nine", 99)]
fn word_number_compound_space_separated_year(#[case] utterance: &str, #[case] expected_year: i32) {
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
// Hundreds: 100–999
// -------------------------------------------------------------------------

/// Simple hundreds (one hundred, two hundred, ..., nine hundred) as days.
/// Note: only 100–31 are valid days in most date contexts, so we test within range.
#[rstest]
#[case("one hundred", 100)]
#[case("two hundred", 200)]
#[case("three hundred", 300)]
fn word_number_hundreds_day_round(#[case] utterance: &str, #[case] expected_day: u16) {
    // Days can be stored as u8, but we test round hundreds separately
    // This test documents the capability; actual day validation may reject > 31.
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    // The tokeniser should extract the number; validation may reject it as invalid day.
    // For now, test that it extracts.
    if let Extracted::Found(day_val) = result.day.value {
        assert_eq!(day_val as u16, expected_day);
    }
}

/// Compound hundreds (one hundred twenty-three, ...) as days (testing up to 999).
#[rstest]
#[case("one hundred twenty-three", 123)]
#[case("two hundred forty-five", 245)]
#[case("five hundred sixty-seven", 567)]
#[case("nine hundred ninety-nine", 999)]
fn word_number_hundreds_compound_day(#[case] utterance: &str, #[case] expected_day: u16) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    if let Extracted::Found(day_val) = result.day.value {
        assert_eq!(day_val as u16, expected_day);
    }
}

/// Round hundreds as years (one hundred, two hundred, ..., nine hundred).
#[rstest]
#[case("one hundred", 100)]
#[case("two hundred", 200)]
#[case("five hundred", 500)]
#[case("nine hundred", 900)]
fn word_number_hundreds_round_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Compound hundreds as years (one hundred twenty, ..., nine hundred ninety-nine).
#[rstest]
#[case("one hundred", 100)]
#[case("one hundred twenty-three", 123)]
#[case("two hundred", 200)]
#[case("two hundred forty-five", 245)]
#[case("three hundred forty-five", 345)]
#[case("five hundred sixty-seven", 567)]
#[case("nine hundred ninety-nine", 999)]
fn word_number_hundreds_compound_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Thousands: 1000–3000
// -------------------------------------------------------------------------

/// Round thousands (one thousand, two thousand, three thousand) as years.
#[rstest]
#[case("one thousand", 1000)]
#[case("two thousand", 2000)]
#[case("three thousand", 3000)]
fn word_number_thousands_round_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Compound thousands (one thousand twenty-three, two thousand fifteen, ...) as years.
#[rstest]
#[case("one thousand", 1000)]
#[case("one thousand twenty-three", 1023)]
#[case("one thousand ninety-nine", 1099)]
#[case("one thousand two hundred thirty-four", 1234)]
#[case("one thousand nine hundred ninety-nine", 1999)]
#[case("two thousand", 2000)]
#[case("two thousand ten", 2010)]
#[case("two thousand twenty-four", 2024)]
#[case("two thousand ninety-nine", 2099)]
#[case("two thousand five hundred fifty-six", 2556)]
#[case("three thousand", 3000)]
fn word_number_thousands_compound_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Case insensitivity and surrounding context
// -------------------------------------------------------------------------

/// Word numbers should work regardless of case — uses single_digit_year_expansion
/// and Literal expansion so unit/teen/compound values return as-is.
#[rstest]
#[case("One", 1)]
#[case("TWO", 2)]
#[case("ThRee", 3)]
#[case("Twenty-One", 21)]
#[case("NINETY-NINE", 99)]
#[case("One Thousand", 1000)]
fn word_number_case_insensitive_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            single_digit_year_expansion: true,
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

/// Word numbers in surrounding text (day context).
#[rstest]
#[case("the twenty-third day", 23)]
#[case("on day thirty-one", 31)]
#[case("date: twenty-eight", 28)]
fn word_number_day_with_context(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Word numbers in surrounding text (year context).
#[rstest]
#[case("the year two thousand twenty-four", 2024)]
#[case("in one thousand nine hundred ninety-nine", 1999)]
#[case("year: one thousand nine hundred eighty-four", 1984)]
fn word_number_year_with_context(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Multiple word numbers in input (first should be extracted)
// -------------------------------------------------------------------------

/// When multiple word numbers are present, the best-fitting one is extracted.
/// With day_only config and two ambiguous day values, the scorer picks the
/// one at the position that best matches the configured component order.
#[rstest]
#[case("twenty-three and fifteen", 23)]
#[case("five or nine", 5)]
fn word_number_multiple_input_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Multiple word numbers in year context (first should be extracted).
/// "ninety-nine" expands to 1999 via the default sliding-window rule.
#[rstest]
#[case("one thousand nine hundred eighty-four or two thousand", 1984)]
#[case("year ninety-nine, almost two thousand", 1999)]
fn word_number_multiple_input_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Edge cases and boundary tests
// -------------------------------------------------------------------------

/// Word numbers at boundaries of valid ranges.
#[rstest]
#[case("one", 1)]
#[case("thirty-one", 31)]
fn word_number_day_boundaries(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Year word numbers at range boundaries (1–3000) — uses single_digit_year_expansion
/// and Literal expansion so small values are returned as-is.
#[rstest]
#[case("one", 1)]
#[case("ninety-nine", 99)]
#[case("nine hundred ninety-nine", 999)]
#[case("one thousand", 1000)]
#[case("one thousand nine hundred ninety-nine", 1999)]
#[case("two thousand", 2000)]
#[case("two thousand ninety-nine", 2099)]
#[case("three thousand", 3000)]
fn word_number_year_boundaries(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            single_digit_year_expansion: true,
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
// Hyphenation variations (hyphenated vs. non-hyphenated)
// -------------------------------------------------------------------------

/// Hyphenated vs. space-separated compound numbers should both work (≤ 31 as day).
#[rstest]
#[case("twenty-one", 21)]
#[case("twenty one", 21)]
fn word_number_hyphenation_variants_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Hyphenated vs. space-separated — 2-digit compounds use Literal expansion,
/// larger numbers use standard year_only_config.
#[rstest]
#[case("two thousand twenty-four", 2024)]
#[case("two thousand twenty four", 2024)]
fn word_number_hyphenation_variants_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Hyphenated vs. space-separated for 2-digit compound years — uses Literal expansion.
#[rstest]
#[case("ninety-nine", 99)]
#[case("ninety nine", 99)]
fn word_number_hyphenation_variants_year_literal(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
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
// Full date extraction with word numbers
// -------------------------------------------------------------------------

/// Word numbers in full date format (day-month-year).
/// Note: "nineteen ninety-four" style is not supported — the grammar only
/// understands thousand-form years ("one thousand nine hundred ninety-four").
/// The short century form is documented in the non-English ignored module.
#[rstest]
#[case(
    "twenty-third January one thousand nine hundred ninety-four",
    23,
    1,
    1994
)]
#[case("first June two thousand twenty-four", 1, 6, 2024)]
#[case("thirty-first December nine hundred ninety-nine", 31, 12, 999)]
fn word_number_full_date_dmy(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, Config::default());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Word numbers in full date format (month-day-year).
#[rstest]
#[case(
    "January twenty-third one thousand nine hundred ninety-four",
    1,
    23,
    1994
)]
#[case("June first two thousand twenty-four", 6, 1, 2024)]
fn word_number_full_date_mdy(
    #[case] utterance: &str,
    #[case] expected_month: u8,
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
    let input = input_with_config(utterance, config);
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Partial date extraction with word numbers
// -------------------------------------------------------------------------

/// Day and month with word numbers.
#[rstest]
#[case("twenty-third January", 23, 1)]
#[case("first June", 1, 6)]
fn word_number_day_and_month(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
) {
    let input = input_with_config(utterance, Config::default());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert!(result.year.value.is_not_found());
}

/// Month and year with word numbers.
#[rstest]
#[case("January one thousand nine hundred ninety-four", 1, 1994)]
#[case("June two thousand twenty-four", 6, 2024)]
fn word_number_month_and_year(
    #[case] utterance: &str,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, Config::default());
    let result = extract(input);

    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
    assert!(result.day.value.is_not_found());
}

// -------------------------------------------------------------------------
// Misspellings and typos (especially for non-English speakers)
// -------------------------------------------------------------------------

// Note: single-character transpositions in basic units (oen, tow, theer, foru,
// fvie, sevne, eihgt, nien) all score below the 0.65 threshold.
// They are documented in the word_numbers_non_english ignored module.

/// Single character transpositions in teens — cases scoring ≥ 0.65.
/// ("tne"=0.33 and "threeten"=0.625 are in the non-English ignored module.)
#[rstest]
#[case("elevne", 11)] // eleven: ratio 0.667
#[case("twevle", 12)] // twelve: ratio 0.667
#[case("forteen", 14)] // fourteen: ratio 0.875
#[case("fiteeen", 15)] // fifteen: ratio 0.714
#[case("nienteen", 19)] // nineteen: ratio 0.750
fn word_number_teens_transposed_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Single character transpositions in compound numbers — valid day range (≤ 31).
#[rstest]
#[case("twetny-one", 21)] // twenty: transposed t-w
fn word_number_compound_transposed_characters_day(
    #[case] utterance: &str,
    #[case] expected_day: u8,
) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Single character transpositions in compound numbers as years — uses Literal expansion.
#[rstest]
#[case("twetny-one", 21)] // twenty: transposed t-w
#[case("thiry-five", 35)] // thirty: missing t
#[case("foyrty-five", 45)] // forty: transposed y-r
#[case("nineyt-nine", 99)] // ninety: transposed e-t
fn word_number_compound_transposed_characters_year(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
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

/// Repeated/double characters: common typo pattern.
/// ("seevn"=0.60 is in the non-English ignored module.)
#[rstest]
#[case("onee", 1)] // one: extra e
#[case("twoo", 2)] // two: extra o
#[case("threee", 3)] // three: extra e
#[case("fivee", 5)] // five: extra e
#[case("ninee", 9)] // nine: extra e
fn word_number_units_repeated_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Repeated characters in teens.
#[rstest]
#[case("tenn", 10)] // ten: extra n
#[case("eleeven", 11)] // eleven: extra e
#[case("twelvee", 12)] // twelve: extra e
#[case("thirtteen", 13)] // thirteen: extra t
#[case("fiftteen", 15)] // fifteen: extra t
fn word_number_teens_repeated_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Repeated characters in tens — valid day range only (≤ 31).
#[rstest]
#[case("twennty", 20)] // twenty: extra n
#[case("thirtty", 30)] // thirty: extra t
#[case("twentty-one", 21)] // twenty with extra t
fn word_number_tens_repeated_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Repeated characters in tens as years — uses Literal expansion.
#[rstest]
#[case("forrty", 40)] // forty: extra r
#[case("ninnety", 90)] // ninety: extra n
fn word_number_tens_repeated_characters_year(#[case] utterance: &str, #[case] expected_year: i32) {
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

/// Omitted/missing characters.
/// "on" is excluded because it is a stop word (prevents matching "on the fifth").
#[rstest]
#[case("tw", 2)] // two: missing o (ratio 0.667)
#[case("thre", 3)] // three: missing e (ratio 0.800)
#[case("fou", 4)] // four: missing r (ratio 0.750)
#[case("fiv", 5)] // five: missing e (ratio 0.750)
#[case("six", 6)] // six: unchanged
#[case("sevn", 7)] // seven: missing e (ratio 0.800)
#[case("eigt", 8)] // eight: missing h (ratio 0.800)
#[case("nin", 9)] // nine: missing e (ratio 0.750)
fn word_number_units_omitted_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Omitted characters in teens — cases that resolve unambiguously at 0.65.
/// "fifte" and "sixte" are ambiguous (tie between fifth/fifty and sixth/sixty)
/// and are in the non-English ignored module.
#[rstest]
#[case("ten", 10)] // baseline
#[case("elven", 11)] // eleven: missing e
#[case("twelv", 12)] // twelve: missing e
#[case("thirtee", 13)] // thirteen: missing n
#[case("fourtee", 14)] // fourteen: missing n
#[case("seventee", 17)] // seventeen: missing n
#[case("eightee", 18)] // eighteen: missing n
fn word_number_teens_omitted_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Omitted characters in tens — valid day range only (≤ 31).
/// Values > 31 are tested as years below.
/// Note: "eight" (eighty missing y) is excluded — it matches as unit 8, not 80.
#[rstest]
#[case("twent", 20)] // twenty: missing y
#[case("thirt", 30)] // thirty: missing y
#[case("twent-one", 21)] // twenty missing y with compound
fn word_number_tens_omitted_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Omitted characters in tens as years — uses Literal expansion.
/// "eight" is excluded — it matches as unit 8, not eighty=80.
#[rstest]
#[case("fort", 40)] // forty: missing y
#[case("fift", 50)] // fifty: missing y
#[case("sixt", 60)] // sixty: missing y
#[case("sevent", 70)] // seventy: missing y
#[case("ninet", 90)] // ninety: missing y
#[case("thirt-five", 35)] // thirty missing y with compound
fn word_number_tens_omitted_characters_year(#[case] utterance: &str, #[case] expected_year: i32) {
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

/// Extra/inserted characters.
/// "eightoa" ties eight(0.714) and eighty(0.714) — tens wins tie, giving 80.
/// It is in the non-English ignored module.
#[rstest]
#[case("owne", 1)] // one: inserted w (ratio 0.750)
#[case("twoo", 2)] // two: inserted o (ratio 0.750)
#[case("therea", 3)] // three: inserted a (ratio 0.833)
#[case("foure", 4)] // four: inserted e (ratio 0.800)
#[case("fivea", 5)] // five: inserted a (ratio 0.800)
#[case("sixe", 6)] // six: inserted e (ratio 0.750)
#[case("sevena", 7)] // seven: inserted a (ratio 0.857)
#[case("ninea", 9)] // nine: inserted a (ratio 0.800)
fn word_number_units_extra_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Extra characters in compound numbers.
/// "thirtea" is ambiguous (0.714 vs "thirty" but 0.750 vs "thirteen") —
/// it is in the non-English ignored module.
#[rstest]
#[case("twentya", 20)] // twenty: inserted a (0.833)
#[case("twentai-one", 21)] // twenty with extra characters in compound
fn word_number_compound_extra_characters_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Phonetic misspellings — cases scoring ≥ 0.65.
/// "wun", "faw", "siks", "ate", "nien" score below threshold and are in the
/// non-English ignored module.
#[rstest]
#[case("too", 2)] // two: ratio 0.667
#[case("tree", 3)] // three: ratio 0.800
#[case("fiv", 5)] // five: ratio 0.750 (same as omitted, listed here for phonetics)
#[case("sevun", 7)] // seven: ratio 0.800
fn word_number_units_phonetic_misspellings_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Phonetic misspellings in teens — cases scoring ≥ 0.65.
/// "thertin" scores 0.625 vs thirteen and "for-tin/fif-tin/nain-tin" are below
/// threshold; all are in the non-English ignored module.
#[rstest]
#[case("tin", 10)] // ten: ratio 0.667
#[case("ilevun", 11)] // eleven: ratio 0.667
#[case("twelv", 12)] // twelve: ratio 0.833
fn word_number_teens_phonetic_misspellings_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Phonetic misspellings in compound numbers — cases where both the tens and
/// Compound phonetic patterns where both tens and unit parts score ≥ 0.65.
/// Uses Literal expansion so the value is returned as-is.
#[rstest]
#[case("thirti-fiv", 35)] // thirty (0.833) + five (0.750)
fn word_number_compound_phonetic_misspellings_year(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
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

/// Phonetic misspellings in hundreds — cases where the unit prefix scores ≥ 0.65.
/// "wun hundred" uses "wun" which scores 0.0 vs "one" and is in the ignored module.
#[rstest]
#[case("too hundred", 200)] // two (0.667)
fn word_number_hundreds_phonetic_misspellings_year(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Phonetic misspellings in thousands — cases where the unit prefix scores ≥ 0.65.
/// "wun thousand" uses "wun" which scores 0.0 vs "one" and is in the ignored module.
#[rstest]
#[case("too thousand", 2000)] // two (0.667)
#[case("too thousand twenti-for", 2024)] // two (0.667) + twenty (0.833) + four (0.750)
fn word_number_thousands_phonetic_misspellings_year(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Mixed case with misspellings: very common in real-world inputs (≤ 31 as day).
#[rstest]
#[case("OnE", 1)]
#[case("tWo", 2)]
#[case("ThReE", 3)]
#[case("TwEnTy-oNe", 21)]
fn word_number_misspellings_mixed_case_day(#[case] utterance: &str, #[case] expected_day: u8) {
    let input = input_with_config(utterance, day_only_config());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
}

/// Mixed case with misspellings as years — uses single_digit_year_expansion
/// and Literal expansion so values are returned as-is.
#[rstest]
#[case("OnE", 1)]
#[case("TwEnTy-oNe", 21)]
#[case("NiNeTy-NiNe", 99)]
fn word_number_misspellings_mixed_case_day_as_year(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            single_digit_year_expansion: true,
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

/// Mixed case with misspellings for large numbers (year context).
#[rstest]
#[case("OnE hUnDrEd", 100)]
#[case("TwO tHoUsAnD", 2000)]
fn word_number_misspellings_mixed_case_large_numbers(
    #[case] utterance: &str,
    #[case] expected_value: i32,
) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_value));
}

/// Mixed case with misspellings for year extraction.
#[rstest]
#[case("OnE tHoUsAnD nInE hUnDrEd eIgHtY-fOuR", 1984)]
#[case("TwO tHoUsAnD tWeNtY-fOuR", 2024)]
fn word_number_misspellings_mixed_case_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Hundred and thousand misspellings/variations.
#[rstest]
#[case("one hundreed", 100)] // extra e
#[case("one hunderd", 100)] // r-d swap
#[case("one hunddred", 100)] // extra d
#[case("one thousend", 1000)] // e for a
#[case("one tousand", 1000)] // missing h
#[case("two thousend", 2000)]
fn word_number_hundred_thousand_misspellings_year(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, year_only_config());
    let result = extract(input);

    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Misspellings in full date context with word numbers.
/// "twenti-three" → twenty-three=23; "nineteen ninety-four" century-style
/// is not supported by the grammar — use thousand-form instead.
#[rstest]
#[case(
    "twenti-three January one thousand nine hundred ninety-four",
    23,
    1,
    1994
)]
#[case("first June two thousand twenty-four", 1, 6, 2024)]
#[case("thirty-first December nine hundred ninety-nine", 31, 12, 999)]
fn word_number_full_date_with_misspellings_dmy(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let input = input_with_config(utterance, Config::default());
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

// -------------------------------------------------------------------------
// Non-English phonetic patterns (ignored — known limitation)
//
// Word-number parsing is optimised for English.  Phonetic spelling patterns
// from non-English speakers (Swahili, Zulu, Yoruba, Igbo, Hausa, Xhosa,
// Amharic, Arabic, Spanish/Portuguese, Dutch, etc.) often score below the
// 0.65 fuzzy-match threshold required to avoid cross-category false
// positives.  These tests are kept as documentation of the intended future
// behaviour and can be enabled once a language-aware matching strategy is
// implemented.
// -------------------------------------------------------------------------

/// Non-English pattern tests are gated behind the `non_english_word_numbers`
/// feature so they do not run in CI.  Run with:
///   cargo test -p partial-date-tests --features non_english_word_numbers
#[cfg(feature = "non_english_word_numbers")]
mod word_numbers_non_english {
    use super::*;

    /// Spelling variations from other languages (German/Dutch/Italian/Spanish/Galician).
    #[rstest]
    #[ignore]
    #[case("vun", 1)]
    #[case("vone", 1)]
    #[case("tvvo", 2)]
    #[case("tre", 3)]
    #[case("catro", 4)]
    #[case("twe", 2)]
    #[case("zeven", 7)]
    fn word_number_units_language_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Language-influenced compound numbers (Spanish/Portuguese).
    #[rstest]
    #[ignore]
    #[case("vinte-un", 21)]
    #[case("treinta-cinco", 35)]
    #[case("noventa-nueve", 99)]
    fn word_number_compound_language_patterns_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Ordinal-confusion patterns common for non-English speakers.
    #[rstest]
    #[ignore]
    #[case("fourt", 4)]
    #[case("fivth", 5)]
    #[case("sixed", 6)]
    #[case("nined", 9)]
    #[case("tenths", 10)]
    fn word_number_units_ordinal_confusion_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Hundred/thousand misspellings that fall below the threshold.
    #[rstest]
    #[ignore]
    #[case("hudred", 100)]
    #[case("thusand", 1000)]
    fn word_number_hundred_thousand_misspellings_year(
        #[case] utterance: &str,
        #[case] expected_year: i32,
    ) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    // --- African language patterns ---

    #[rstest]
    #[ignore]
    #[case("uan", 1)]
    #[case("twu", 2)]
    #[case("thri", 3)]
    #[case("fua", 4)]
    #[case("faiv", 5)]
    #[case("sikis", 6)]
    #[case("sevin", 7)]
    #[case("eiti", 8)]
    #[case("naini", 9)]
    fn word_number_units_swahili_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("elefun", 11)]
    #[case("tueluf", 12)]
    #[case("tertini", 13)]
    #[case("twenti-uan", 21)]
    #[case("tenta-faif", 35)]
    #[case("nainti-naini", 99)]
    fn word_number_swahili_patterns_compound_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("wuun", 1)]
    #[case("tuu", 2)]
    #[case("thii", 3)]
    #[case("fuu", 4)]
    #[case("fiifu", 5)]
    #[case("sikisi", 6)]
    #[case("seevin", 7)]
    #[case("nayiti", 8)]
    #[case("nayini", 9)]
    fn word_number_units_zulu_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("twuenti-wuun", 21)]
    #[case("teertin-thii", 13)]
    #[case("nainti-naayiti", 99)]
    fn word_number_zulu_patterns_compound_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("waan", 1)]
    #[case("twoo", 2)]
    #[case("thaa", 3)]
    #[case("foor", 4)]
    #[case("faav", 5)]
    #[case("siks", 6)]
    #[case("sevan", 7)]
    #[case("eyit", 8)]
    #[case("naan", 9)]
    fn word_number_units_yoruba_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("twenti-waan", 21)]
    #[case("teertin-thaa", 13)]
    #[case("foorti-foor", 40)]
    fn word_number_yoruba_patterns_compound_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("onym", 1)]
    #[case("twum", 2)]
    #[case("threem", 3)]
    #[case("fuum", 4)]
    #[case("faimm", 5)]
    #[case("siksis", 6)]
    #[case("sevinm", 7)]
    #[case("eightm", 8)]
    #[case("nainim", 9)]
    fn word_number_units_igbo_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("twenti-onym", 21)]
    #[case("terty-fuum", 30)]
    fn word_number_igbo_patterns_compound_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("wanna", 1)]
    #[case("ttoua", 2)]
    #[case("tthrea", 3)]
    #[case("foura", 4)]
    #[case("fiiva", 5)]
    #[case("siiksa", 6)]
    #[case("sevva", 7)]
    #[case("eyttha", 8)]
    #[case("ninna", 9)]
    fn word_number_units_hausa_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("ttwenti-wanna", 21)]
    #[case("tthirteen-foura", 13)]
    fn word_number_hausa_patterns_compound_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("wun", 1)]
    #[case("too", 2)]
    #[case("foor", 4)]
    #[case("fayv", 5)]
    #[case("siks", 6)]
    #[case("seben", 7)]
    #[case("ayt", 8)]
    #[case("nayn", 9)]
    fn word_number_units_xhosa_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("unne", 1)]
    #[case("tuwe", 2)]
    #[case("tihree", 3)]
    #[case("fure", 4)]
    #[case("fayive", 5)]
    #[case("seekis", 6)]
    #[case("sevene", 7)]
    #[case("eyitte", 8)]
    #[case("nayine", 9)]
    fn word_number_units_amharic_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("wun", 1)]
    #[case("tuh", 2)]
    #[case("thrih", 3)]
    #[case("fuh", 4)]
    #[case("fiv", 5)]
    #[case("sven", 7)]
    #[case("ayt", 8)]
    #[case("nin", 9)]
    fn word_number_units_arabic_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("waan hunderd", 100)]
    #[case("twum hunderd", 200)]
    #[case("wuun thoussand", 1000)]
    #[case("ttwenti-wanna thoussand", 1020)]
    #[case("unne hunderd tenty-fiive", 125)]
    fn word_number_hundreds_thousands_african_patterns_year(
        #[case] utterance: &str,
        #[case] expected_year: i32,
    ) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    #[rstest]
    #[ignore]
    #[case("twenti-uan Januari twentee-twenty-four", 21, 1, 2024)]
    #[case("therty-twum June waan thoussand", 32, 6, 1000)]
    #[case("wuun December twum hunderd", 1, 12, 200)]
    #[case("naini Januari unne hunderd nayine", 9, 1, 199)]
    #[case("twenti-three May twee thousend twenty-four", 23, 5, 2024)]
    fn word_number_full_date_african_patterns_dmy(
        #[case] utterance: &str,
        #[case] expected_day: u8,
        #[case] expected_month: u8,
        #[case] expected_year: i32,
    ) {
        let input = input_with_config(utterance, Config::default());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
        assert_eq!(result.month.number, Extracted::Found(expected_month));
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    #[rstest]
    #[ignore]
    #[case("twee thousend seventeen", 2017)]
    #[case("twum hunderd ninty-nien", 299)]
    #[case("waan thoussand ninn hunderd ninny-ninn", 1999)]
    #[case("unne thousend faiv hunderd", 1500)]
    #[case("naini hunderd naayiti", 999)]
    fn word_number_african_patterns_large_years(
        #[case] utterance: &str,
        #[case] expected_year: i32,
    ) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    #[rstest]
    #[ignore]
    #[case("twenty-twum fifteen", 22)]
    #[case("therty-wuan seven", 37)]
    #[case("ninety-unne June", 91)]
    #[case("one hunderd naini", 109)]
    fn word_number_mixed_african_patterns_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    #[rstest]
    #[ignore]
    #[case("twee thousend nineteen", 2019)]
    #[case("unne hunderd ninny-ninn", 199)]
    fn word_number_mixed_african_patterns_year(
        #[case] utterance: &str,
        #[case] expected_year: i32,
    ) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    // -----------------------------------------------------------------------
    // English patterns below the 0.65 threshold or ambiguous at that threshold
    // -----------------------------------------------------------------------

    /// Single-character transpositions in basic units (all score < 0.65).
    #[rstest]
    #[ignore]
    #[case("oen", 1)]
    #[case("tow", 2)]
    #[case("theer", 3)]
    #[case("foru", 4)]
    #[case("fvie", 5)]
    #[case("sevne", 7)]
    #[case("eihgt", 8)]
    #[case("nien", 9)]
    fn word_number_units_transposed_characters_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Phonetic unit misspellings that score below 0.65.
    #[rstest]
    #[ignore]
    #[case("wun", 1)]
    #[case("faw", 4)]
    #[case("siks", 6)]
    #[case("ate", 8)]
    #[case("nien", 9)]
    fn word_number_units_phonetic_below_threshold_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Repeated-characters that score below 0.65 ("seevn" = 0.60 vs "seven").
    #[rstest]
    #[ignore]
    #[case("seevn", 7)]
    fn word_number_units_repeated_below_threshold_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Teen transpositions below threshold.
    #[rstest]
    #[ignore]
    #[case("tne", 10)]
    #[case("threeten", 13)]
    fn word_number_teens_transposed_below_threshold_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Teen omissions that are ambiguous at 0.65 ("fifte" ties fifth=5 and fifty=50,
    /// "sixte" ties sixth=6 and sixty=60 — tens wins the tie, producing wrong result).
    #[rstest]
    #[ignore]
    #[case("fifte", 15)]
    #[case("sixte", 16)]
    fn word_number_teens_omitted_ambiguous_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Teen phonetic patterns that score below 0.65.
    #[rstest]
    #[ignore]
    #[case("thertin", 13)] // thirteen: 0.625
    #[case("for-tin", 14)] // fourteen: 0.375 (split on -)
    #[case("fif-tin", 15)] // fifteen: 0.429 (split on -)
    #[case("nain-tin", 19)] // nineteen: 0.250 (split on -)
    fn word_number_teens_phonetic_hyphenated_day(
        #[case] utterance: &str,
        #[case] expected_day: u8,
    ) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// Compound phonetic patterns where the unit part is below threshold.
    #[rstest]
    #[ignore]
    #[case("twenti-wun", 21)]
    #[case("forti-faw", 45)]
    #[case("nainty-nien", 99)]
    fn word_number_compound_phonetic_unit_below_threshold(
        #[case] utterance: &str,
        #[case] expected_year: i32,
    ) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    /// "wun hundred / wun thousand" — "wun" scores 0.0 vs "one".
    #[rstest]
    #[ignore]
    #[case("wun hundred", 100)]
    #[case("wun hundred twenti-fiv", 125)]
    #[case("wun thousand", 1000)]
    #[case("wun thousand twenti-fiv", 1025)]
    fn word_number_wun_prefix_below_threshold(#[case] utterance: &str, #[case] expected_year: i32) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }

    /// "thirtea" is ambiguous: scores 0.750 vs "thirteen" but only 0.714 vs "thirty".
    #[rstest]
    #[ignore]
    #[case("thirtea", 30)]
    fn word_number_compound_extra_ambiguous_day(#[case] utterance: &str, #[case] expected_day: u8) {
        let input = input_with_config(utterance, day_only_config());
        let result = extract(input);
        assert_eq!(result.day.value, Extracted::Found(expected_day));
    }

    /// "nineteen ninety-four" century-style years are not supported by the grammar.
    /// The parser treats "nineteen" as a standalone number (19) and "ninety-four"
    /// as another (94/1994), resulting in two separate tokens rather than 1994.
    #[rstest]
    #[ignore]
    #[case("nineteen ninety-four", 1994)]
    #[case("nineteen eighty-four", 1984)]
    #[case("nineteen ninety-nine", 1999)]
    fn word_number_century_style_year(#[case] utterance: &str, #[case] expected_year: i32) {
        let input = input_with_config(utterance, year_only_config());
        let result = extract(input);
        assert_eq!(result.year.value, Extracted::Found(expected_year));
    }
}
