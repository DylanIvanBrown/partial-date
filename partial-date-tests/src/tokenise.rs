// Tests for the tokenise() function.
//
// Each test group exercises one orthogonal property of the tokeniser so that
// failures point clearly at the broken behaviour.

use partial_date::extract::tokenise;
use partial_date::models::Token;
use rstest::rstest;

// -------------------------------------------------------------------------
// Numeric tokens
// -------------------------------------------------------------------------

/// A bare integer produces a single Numeric token.
#[rstest]
#[case("19", vec![Token::Numeric("19")])]
#[case("2014", vec![Token::Numeric("2014")])]
#[case("06", vec![Token::Numeric("06")])]
#[case("1", vec![Token::Numeric("1")])]
fn numeric_bare(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// OrdinalDay tokens
// -------------------------------------------------------------------------

/// Numbers with ordinal suffixes produce OrdinalDay tokens.
#[rstest]
#[case("1st", vec![Token::OrdinalDay("1st")])]
#[case("2nd", vec![Token::OrdinalDay("2nd")])]
#[case("3rd", vec![Token::OrdinalDay("3rd")])]
#[case("4th", vec![Token::OrdinalDay("4th")])]
#[case("11th", vec![Token::OrdinalDay("11th")])]
#[case("19th", vec![Token::OrdinalDay("19th")])]
#[case("21st", vec![Token::OrdinalDay("21st")])]
#[case("22nd", vec![Token::OrdinalDay("22nd")])]
#[case("23rd", vec![Token::OrdinalDay("23rd")])]
#[case("31st", vec![Token::OrdinalDay("31st")])]
fn ordinal_day_bare(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Ordinal suffixes are case-insensitive.
#[rstest]
#[case("1ST", vec![Token::OrdinalDay("1ST")])]
#[case("2ND", vec![Token::OrdinalDay("2ND")])]
#[case("3RD", vec![Token::OrdinalDay("3RD")])]
#[case("4TH", vec![Token::OrdinalDay("4TH")])]
fn ordinal_day_uppercase_suffix(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// MonthName tokens
// -------------------------------------------------------------------------

/// Full month names produce a single MonthName token.
#[rstest]
#[case("January", vec![Token::MonthName("January")])]
#[case("February", vec![Token::MonthName("February")])]
#[case("March", vec![Token::MonthName("March")])]
#[case("April", vec![Token::MonthName("April")])]
#[case("May", vec![Token::MonthName("May")])]
#[case("June", vec![Token::MonthName("June")])]
#[case("July", vec![Token::MonthName("July")])]
#[case("August", vec![Token::MonthName("August")])]
#[case("September", vec![Token::MonthName("September")])]
#[case("October", vec![Token::MonthName("October")])]
#[case("November", vec![Token::MonthName("November")])]
#[case("December", vec![Token::MonthName("December")])]
fn month_name_full(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// Misspelled months
#[rstest]
#[case("decmber", vec![Token::MonthName("decmber")])]
#[case("Janary", vec![Token::MonthName("Janary")])]
#[case("Febraury", vec![Token::MonthName("Febraury")])]
#[case("Marsh", vec![Token::MonthName("Marsh")])]
#[case("Aprill", vec![Token::MonthName("Aprill")])]
#[case("Mey", vec![Token::MonthName("Mey")])]
#[case("Dune", vec![Token::MonthName("Dune")])]
#[case("Juli", vec![Token::MonthName("Juli")])]
#[case("Augst", vec![Token::MonthName("Augst")])]
#[case("Septembr", vec![Token::MonthName("Septembr")])]
#[case("Octobr", vec![Token::MonthName("Octobr")])]
fn month_name_misspelled(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Standard 3-letter abbreviations produce a MonthName token.
#[rstest]
#[case("Jan", vec![Token::MonthName("Jan")])]
#[case("Feb", vec![Token::MonthName("Feb")])]
#[case("Mar", vec![Token::MonthName("Mar")])]
#[case("Apr", vec![Token::MonthName("Apr")])]
#[case("may", vec![Token::MonthName("may")])]
#[case("Jun", vec![Token::MonthName("Jun")])]
#[case("Jul", vec![Token::MonthName("Jul")])]
#[case("Aug", vec![Token::MonthName("Aug")])]
#[case("Sep", vec![Token::MonthName("Sep")])]
#[case("Oct", vec![Token::MonthName("Oct")])]
#[case("Nov", vec![Token::MonthName("Nov")])]
#[case("Dec", vec![Token::MonthName("Dec")])]
#[case("jan", vec![Token::MonthName("jan")])]
fn month_name_abbreviation(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Month names are case-insensitive.
#[rstest]
#[case("january", vec![Token::MonthName("january")])]
#[case("OCTOBER", vec![Token::MonthName("OCTOBER")])]
#[case("jUlY", vec![Token::MonthName("jUlY")])]
#[case("dec", vec![Token::MonthName("dec")])]
fn month_name_case_insensitive(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Unambiguous month name prefixes (≥ 4 chars) produce a MonthName token.
#[rstest]
#[case("Janu", vec![Token::MonthName("Janu")])]
#[case("Febr", vec![Token::MonthName("Febr")])]
#[case("Sept", vec![Token::MonthName("Sept")])]
#[case("Novem", vec![Token::MonthName("Novem")])]
#[case("Decem", vec![Token::MonthName("Decem")])]
fn month_name_unambiguous_prefix(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// Abbreviation with trailing dot: the dot is treated as a separator so
/// the token carries the bare abbreviation without the dot.
#[rstest]
#[case("Jan.", vec![Token::MonthName("Jan")])]
#[case("Feb.", vec![Token::MonthName("Feb")])]
#[case("Dec.", vec![Token::MonthName("Dec")])]
fn month_name_trailing_dot(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
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
        vec![
            Token::Numeric("25"),
            Token::Numeric("12"),
            Token::Numeric("2024"),
        ]
    );
}

/// Multiple consecutive separators collapse correctly (no empty tokens).
#[rstest]
#[case("25  12  2024")]
#[case("25 / 12 / 2024")]
fn consecutive_separators_no_empty_tokens(#[case] utterance: &str) {
    assert_eq!(
        tokenise(utterance, &[]),
        vec![
            Token::Numeric("25"),
            Token::Numeric("12"),
            Token::Numeric("2024"),
        ]
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
    vec![Token::Numeric("18"), Token::Numeric("6"), Token::Numeric("2013")]
)]
#[case(
    "19th  October,2015",
    vec![Token::OrdinalDay("19th"), Token::MonthName("October"), Token::Numeric("2015")]
)]
fn mixed_separators(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// No-space digit↔month-name adjacency
// -------------------------------------------------------------------------

/// A digit run immediately followed by a month name (no separator) should
/// produce two tokens: Numeric then MonthName.
#[rstest]
#[case("19october", vec![Token::Numeric("19"), Token::MonthName("october")])]
#[case("9july", vec![Token::Numeric("9"), Token::MonthName("july")])]
fn digit_then_month_name_adjacent(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

/// A month name immediately followed by a digit run (no separator) should
/// produce two tokens: MonthName then Numeric.
#[rstest]
#[case("August7", vec![Token::MonthName("August"), Token::Numeric("7")])]
#[case("January27", vec![Token::MonthName("January"), Token::Numeric("27")])]
fn month_name_then_digit_adjacent(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}

// -------------------------------------------------------------------------
// Noise words mixed in with valid date components
// -------------------------------------------------------------------------

/// Noise words between valid tokens are dropped; at most 3 tokens returned.
#[rstest]
#[case(
    "I was born in 2014 October 19",
    vec![Token::Numeric("2014"), Token::MonthName("October"), Token::Numeric("19")]
)]
#[case(
    "Friday 31",
    vec![Token::Numeric("31")]
)]
#[case(
    "8 October >type 11",
    vec![Token::Numeric("8"), Token::MonthName("October"), Token::Numeric("11")]
)]
fn noise_words_dropped(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
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
    assert_eq!(result[0], Token::Numeric("12"));
    assert_eq!(result[1], Token::MonthName("June"));
    assert_eq!(result[2], Token::Numeric("2024"));
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
        vec![
            Token::Numeric("25"),
            Token::Numeric("12"),
            Token::Numeric("2024"),
        ]
    );
}

/// A custom single-char extra separator is recognised.
#[test]
fn extra_separator_single_char() {
    let extra = vec!["|".to_string()];
    assert_eq!(
        tokenise("25|12|2024", &extra),
        vec![
            Token::Numeric("25"),
            Token::Numeric("12"),
            Token::Numeric("2024"),
        ]
    );
}

// -------------------------------------------------------------------------
// Specific utterances from the test suite
// -------------------------------------------------------------------------

/// All of the real-world utterance examples used across the test suite,
/// verified to produce the correct token sequence.
#[rstest]
#[case("June", vec![Token::MonthName("June")])]
#[case("2010", vec![Token::Numeric("2010")])]
#[case("16th", vec![Token::OrdinalDay("16th")])]
#[case("18th", vec![Token::OrdinalDay("18th")])]
#[case(
    "31 July  2014",
    vec![Token::Numeric("31"), Token::MonthName("July"), Token::Numeric("2014")]
)]
#[case(
    "30 December 2014",
    vec![Token::Numeric("30"), Token::MonthName("December"), Token::Numeric("2014")]
)]
#[case(
    "25th  October  2025",
    vec![Token::OrdinalDay("25th"), Token::MonthName("October"), Token::Numeric("2025")]
)]
#[case(
    "25 October",
    vec![Token::Numeric("25"), Token::MonthName("October")]
)]
#[case(
    "October 26 2012",
    vec![Token::MonthName("October"), Token::Numeric("26"), Token::Numeric("2012")]
)]
#[case(
    "8 October 11",
    vec![Token::Numeric("8"), Token::MonthName("October"), Token::Numeric("11")]
)]
#[case(
    "January27 2013",
    vec![Token::MonthName("January"), Token::Numeric("27"), Token::Numeric("2013")]
)]
fn real_world_utterances(#[case] utterance: &str, #[case] expected: Vec<Token<'_>>) {
    assert_eq!(tokenise(utterance, &[]), expected);
}
