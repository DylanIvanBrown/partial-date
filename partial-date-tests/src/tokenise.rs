// Tests for the tokenise() function.
//
// Each test group exercises one orthogonal property of the tokeniser so that
// failures point clearly at the broken behaviour.

use partial_date::extract::tokenise;
use partial_date::models::{MonthName, Token};
use rstest::rstest;

// -------------------------------------------------------------------------
// Numeric tokens
// -------------------------------------------------------------------------

/// A bare integer produces a single Numeric token carrying the parsed value.
#[rstest]
#[case("19",   vec![Token::Numeric(19)])]
#[case("2014", vec![Token::Numeric(2014)])]
#[case("06",   vec![Token::Numeric(6)])]
#[case("1",    vec![Token::Numeric(1)])]
fn numeric_bare(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// OrdinalDay tokens
// -------------------------------------------------------------------------

/// Numbers with ordinal suffixes produce OrdinalDay tokens carrying the
/// parsed day number (suffix stripped).
#[rstest]
#[case("1st",  vec![Token::OrdinalDay(1)])]
#[case("2nd",  vec![Token::OrdinalDay(2)])]
#[case("3rd",  vec![Token::OrdinalDay(3)])]
#[case("4th",  vec![Token::OrdinalDay(4)])]
#[case("11th", vec![Token::OrdinalDay(11)])]
#[case("19th", vec![Token::OrdinalDay(19)])]
#[case("21st", vec![Token::OrdinalDay(21)])]
#[case("22nd", vec![Token::OrdinalDay(22)])]
#[case("23rd", vec![Token::OrdinalDay(23)])]
#[case("31st", vec![Token::OrdinalDay(31)])]
fn ordinal_day_bare(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Ordinal suffixes are case-insensitive; the day value is the same regardless.
#[rstest]
#[case("1ST", vec![Token::OrdinalDay(1)])]
#[case("2ND", vec![Token::OrdinalDay(2)])]
#[case("3RD", vec![Token::OrdinalDay(3)])]
#[case("4TH", vec![Token::OrdinalDay(4)])]
fn ordinal_day_uppercase_suffix(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// MonthName tokens
// -------------------------------------------------------------------------

/// Full month names produce a MonthName token with the resolved variant.
#[rstest]
#[case("January",   vec![Token::MonthName(MonthName::January)])]
#[case("February",  vec![Token::MonthName(MonthName::February)])]
#[case("March",     vec![Token::MonthName(MonthName::March)])]
#[case("April",     vec![Token::MonthName(MonthName::April)])]
#[case("May",       vec![Token::MonthName(MonthName::May)])]
#[case("June",      vec![Token::MonthName(MonthName::June)])]
#[case("July",      vec![Token::MonthName(MonthName::July)])]
#[case("August",    vec![Token::MonthName(MonthName::August)])]
#[case("September", vec![Token::MonthName(MonthName::September)])]
#[case("October",   vec![Token::MonthName(MonthName::October)])]
#[case("November",  vec![Token::MonthName(MonthName::November)])]
#[case("December",  vec![Token::MonthName(MonthName::December)])]
fn month_name_full(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Misspelled month names are resolved to the correct variant via fuzzy matching.
#[rstest]
#[case("decmber",  vec![Token::MonthName(MonthName::December)])]
#[case("Janary",   vec![Token::MonthName(MonthName::January)])]
#[case("Febraury", vec![Token::MonthName(MonthName::February)])]
#[case("Marsh",    vec![Token::MonthName(MonthName::March)])]
#[case("Aprill",   vec![Token::MonthName(MonthName::April)])]
#[case("Mey",      vec![Token::MonthName(MonthName::May)])]
#[case("Dune",     vec![Token::MonthName(MonthName::June)])]
#[case("Juli",     vec![Token::MonthName(MonthName::July)])]
#[case("Augst",    vec![Token::MonthName(MonthName::August)])]
#[case("Septembr", vec![Token::MonthName(MonthName::September)])]
#[case("Octobr",   vec![Token::MonthName(MonthName::October)])]
fn month_name_misspelled(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Standard 3-letter abbreviations produce the correct MonthName variant.
#[rstest]
#[case("Jan", vec![Token::MonthName(MonthName::January)])]
#[case("Feb", vec![Token::MonthName(MonthName::February)])]
#[case("Mar", vec![Token::MonthName(MonthName::March)])]
#[case("Apr", vec![Token::MonthName(MonthName::April)])]
#[case("may", vec![Token::MonthName(MonthName::May)])]
#[case("Jun", vec![Token::MonthName(MonthName::June)])]
#[case("Jul", vec![Token::MonthName(MonthName::July)])]
#[case("Aug", vec![Token::MonthName(MonthName::August)])]
#[case("Sep", vec![Token::MonthName(MonthName::September)])]
#[case("Oct", vec![Token::MonthName(MonthName::October)])]
#[case("Nov", vec![Token::MonthName(MonthName::November)])]
#[case("Dec", vec![Token::MonthName(MonthName::December)])]
#[case("jan", vec![Token::MonthName(MonthName::January)])]
fn month_name_abbreviation(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Month name matching is case-insensitive.
#[rstest]
#[case("january", vec![Token::MonthName(MonthName::January)])]
#[case("OCTOBER", vec![Token::MonthName(MonthName::October)])]
#[case("jUlY",    vec![Token::MonthName(MonthName::July)])]
#[case("dec",     vec![Token::MonthName(MonthName::December)])]
fn month_name_case_insensitive(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Unambiguous month name prefixes (≥ 4 chars) produce the correct variant.
#[rstest]
#[case("Janu",  vec![Token::MonthName(MonthName::January)])]
#[case("Febr",  vec![Token::MonthName(MonthName::February)])]
#[case("Sept",  vec![Token::MonthName(MonthName::September)])]
#[case("Novem", vec![Token::MonthName(MonthName::November)])]
#[case("Decem", vec![Token::MonthName(MonthName::December)])]
fn month_name_unambiguous_prefix(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// A trailing dot is consumed as a separator; the bare abbreviation still
/// resolves to the correct variant.
#[rstest]
#[case("Jan.", vec![Token::MonthName(MonthName::January)])]
#[case("Feb.", vec![Token::MonthName(MonthName::February)])]
#[case("Dec.", vec![Token::MonthName(MonthName::December)])]
fn month_name_trailing_dot(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// Noise — single tokens that should produce nothing
// -------------------------------------------------------------------------

/// Words that carry no date meaning are discarded entirely.
#[rstest]
#[case("No")]
#[case("Friday")]
#[case("born")]
#[case("the")]
#[case("of")]
#[case("in")]
fn noise_single_word(#[case] utterance: &str) {
    assert_eq!(tokenise(utterance, &[]), vec![]);
}

/// Non-alphanumeric input produces no tokens.
#[rstest]
#[case("")]
#[case("   ")]
#[case(">")]
#[case(",,")]
fn noise_empty_or_punctuation(#[case] utterance: &str) {
    assert_eq!(tokenise(utterance, &[]), vec![]);
}

// -------------------------------------------------------------------------
// Standard separator splitting
// -------------------------------------------------------------------------

/// All standard single-char separators split tokens correctly.
#[rstest]
#[case("25/12/2024")]
#[case("25-12-2024")]
#[case("25.12.2024")]
#[case("25,12,2024")]
#[case("25 12 2024")]
#[case("25\\12\\2024")]
fn standard_separators_three_numerics(#[case] utterance: &str) {
    assert_eq!(
        tokenise(utterance, &[]),
        vec![Token::Numeric(25), Token::Numeric(12), Token::Numeric(2024),]
    );
}

/// Multiple consecutive separators collapse correctly (no empty tokens).
#[rstest]
#[case("25  12  2024")]
#[case("25 / 12 / 2024")]
fn consecutive_separators_no_empty_tokens(#[case] utterance: &str) {
    assert_eq!(
        tokenise(utterance, &[]),
        vec![Token::Numeric(25), Token::Numeric(12), Token::Numeric(2024),]
    );
}

// -------------------------------------------------------------------------
// Mixed separators within a single utterance
// -------------------------------------------------------------------------

/// A string that uses different separators between different pairs of
/// components is handled correctly.
#[rstest]
#[case(
    "18/6. 2013",
    vec![Token::Numeric(18), Token::Numeric(6), Token::Numeric(2013)]
)]
#[case(
    "19th  October,2015",
    vec![Token::OrdinalDay(19), Token::MonthName(MonthName::October), Token::Numeric(2015)]
)]
fn mixed_separators(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// No-space digit↔month-name adjacency
// -------------------------------------------------------------------------

/// A digit run immediately followed by a month name (no separator) should
/// produce two tokens: Numeric then MonthName.
#[rstest]
#[case("19october", vec![Token::Numeric(19), Token::MonthName(MonthName::October)])]
#[case("9july",     vec![Token::Numeric(9),  Token::MonthName(MonthName::July)])]
fn digit_then_month_name_adjacent(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// A month name immediately followed by a digit run (no separator) should
/// produce two tokens: MonthName then Numeric.
#[rstest]
#[case("August7",   vec![Token::MonthName(MonthName::August),  Token::Numeric(7)])]
#[case("January27", vec![Token::MonthName(MonthName::January), Token::Numeric(27)])]
fn month_name_then_digit_adjacent(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// Noise words mixed in with valid date components
// -------------------------------------------------------------------------

/// Noise words between valid tokens are dropped; at most 3 tokens returned.
#[rstest]
#[case(
    "I was born in 2014 October 19",
    vec![Token::Numeric(2014), Token::MonthName(MonthName::October), Token::Numeric(19)]
)]
#[case(
    "Friday 31",
    vec![Token::Numeric(31)]
)]
#[case(
    "8 October >type 11",
    vec![Token::Numeric(8), Token::MonthName(MonthName::October), Token::Numeric(11)]
)]
fn noise_words_dropped(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// At-most-three token cap
// -------------------------------------------------------------------------

/// A string with more than three meaningful chunks: only the first three
/// tokens are returned.
#[test]
fn at_most_three_tokens() {
    // "12 June 2024 extra 99" has 4 meaningful tokens — only first 3 kept.
    let result = tokenise("12 June 2024 extra 99", &[]);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], Token::Numeric(12));
    assert_eq!(result[1], Token::MonthName(MonthName::June));
    assert_eq!(result[2], Token::Numeric(2024));
}

// -------------------------------------------------------------------------
// Extra separators
// -------------------------------------------------------------------------

/// A custom multi-char extra separator is recognised.
#[rstest]
#[case("25||12||2024", "||")]
#[case("25::12::2024", "::")]
fn extra_separator_multi_char(#[case] utterance: &str, #[case] sep: &str) {
    let extra = vec![sep.to_string()];
    assert_eq!(
        tokenise(utterance, &extra),
        vec![Token::Numeric(25), Token::Numeric(12), Token::Numeric(2024),]
    );
}

/// A custom single-char extra separator is recognised.
#[test]
fn extra_separator_single_char() {
    let extra = vec!["|".to_string()];
    assert_eq!(
        tokenise("25|12|2024", &extra),
        vec![Token::Numeric(25), Token::Numeric(12), Token::Numeric(2024),]
    );
}

// -------------------------------------------------------------------------
// Specific utterances from the test suite
// -------------------------------------------------------------------------

/// All of the real-world utterance examples used across the test suite,
/// verified to produce the correct token sequence.
#[rstest]
#[case("June", vec![Token::MonthName(MonthName::June)])]
#[case("2010", vec![Token::Numeric(2010)])]
#[case("16th", vec![Token::OrdinalDay(16)])]
#[case("18th", vec![Token::OrdinalDay(18)])]
#[case(
    "31 July  2014",
    vec![Token::Numeric(31), Token::MonthName(MonthName::July), Token::Numeric(2014)]
)]
#[case(
    "30 December 2014",
    vec![Token::Numeric(30), Token::MonthName(MonthName::December), Token::Numeric(2014)]
)]
#[case(
    "25th  October  2025",
    vec![Token::OrdinalDay(25), Token::MonthName(MonthName::October), Token::Numeric(2025)]
)]
#[case(
    "25 October",
    vec![Token::Numeric(25), Token::MonthName(MonthName::October)]
)]
#[case(
    "October 26 2012",
    vec![Token::MonthName(MonthName::October), Token::Numeric(26), Token::Numeric(2012)]
)]
#[case(
    "8 October 11",
    vec![Token::Numeric(8), Token::MonthName(MonthName::October), Token::Numeric(11)]
)]
#[case(
    "January27 2013",
    vec![Token::MonthName(MonthName::January), Token::Numeric(27), Token::Numeric(2013)]
)]
fn real_world_utterances(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}
