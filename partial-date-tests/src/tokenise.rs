// Tests for the tokenise() function.
//
// Each test group exercises one orthogonal property of the tokeniser so that
// failures point clearly at the broken behaviour.

use partial_date::extract::tokenise;
use partial_date::models::{ComponentOrder, Config, DateComponent, MonthName, Token};
use rstest::rstest;

// Helper: config with no_separator enabled and the given order.
fn no_sep_config(order: ComponentOrder) -> Config {
    Config {
        component_order: order,
        no_separator: true,
        ..Default::default()
    }
}

fn order_dmy() -> ComponentOrder {
    ComponentOrder {
        first: DateComponent::Day,
        second: DateComponent::Month,
        third: DateComponent::Year,
    }
}

fn order_ymd() -> ComponentOrder {
    ComponentOrder {
        first: DateComponent::Year,
        second: DateComponent::Month,
        third: DateComponent::Day,
    }
}

fn order_mdy() -> ComponentOrder {
    ComponentOrder {
        first: DateComponent::Month,
        second: DateComponent::Day,
        third: DateComponent::Year,
    }
}

// -------------------------------------------------------------------------
// Numeric tokens
// -------------------------------------------------------------------------

/// A bare integer produces a single Numeric token carrying (value, digit_count).
#[rstest]
#[case("19",   vec![Token::Numeric(19, 2)])]
#[case("2014", vec![Token::Numeric(2014, 4)])]
#[case("06",   vec![Token::Numeric(6, 2)])]
#[case("1",    vec![Token::Numeric(1, 1)])]
fn numeric_bare(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

// -------------------------------------------------------------------------
// OrdinalDay tokens
// -------------------------------------------------------------------------

/// Numbers with ordinal suffixes produce OrdinalDay tokens (suffix stripped).
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
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

/// Ordinal suffixes are case-insensitive; the day value is the same.
#[rstest]
#[case("1ST", vec![Token::OrdinalDay(1)])]
#[case("2ND", vec![Token::OrdinalDay(2)])]
#[case("3RD", vec![Token::OrdinalDay(3)])]
#[case("4TH", vec![Token::OrdinalDay(4)])]
fn ordinal_day_uppercase_suffix(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
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
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

/// Misspelled month names are resolved via fuzzy matching.
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
    assert_eq!(tokenise(utterance, &Config::default()), expected);
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
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

/// Month name matching is case-insensitive.
#[rstest]
#[case("january", vec![Token::MonthName(MonthName::January)])]
#[case("OCTOBER", vec![Token::MonthName(MonthName::October)])]
#[case("jUlY",    vec![Token::MonthName(MonthName::July)])]
#[case("dec",     vec![Token::MonthName(MonthName::December)])]
fn month_name_case_insensitive(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

/// Unambiguous month name prefixes (≥ 4 chars) produce the correct variant.
#[rstest]
#[case("Janu",  vec![Token::MonthName(MonthName::January)])]
#[case("Febr",  vec![Token::MonthName(MonthName::February)])]
#[case("Sept",  vec![Token::MonthName(MonthName::September)])]
#[case("Novem", vec![Token::MonthName(MonthName::November)])]
#[case("Decem", vec![Token::MonthName(MonthName::December)])]
fn month_name_unambiguous_prefix(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

/// A trailing dot is consumed as a separator; the bare abbreviation resolves.
#[rstest]
#[case("Jan.", vec![Token::MonthName(MonthName::January)])]
#[case("Feb.", vec![Token::MonthName(MonthName::February)])]
#[case("Dec.", vec![Token::MonthName(MonthName::December)])]
fn month_name_trailing_dot(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

// -------------------------------------------------------------------------
// Noise — single tokens that should produce nothing
// -------------------------------------------------------------------------

#[rstest]
#[case("No")]
#[case("Friday")]
#[case("born")]
#[case("the")]
#[case("of")]
#[case("in")]
fn noise_single_word(#[case] utterance: &str) {
    assert_eq!(tokenise(utterance, &Config::default()), vec![]);
}

#[rstest]
#[case("")]
#[case("   ")]
#[case(">")]
#[case(",,")]
fn noise_empty_or_punctuation(#[case] utterance: &str) {
    assert_eq!(tokenise(utterance, &Config::default()), vec![]);
}

// -------------------------------------------------------------------------
// Standard separator splitting
// -------------------------------------------------------------------------

#[rstest]
#[case("25/12/2024")]
#[case("25-12-2024")]
#[case("25.12.2024")]
#[case("25,12,2024")]
#[case("25 12 2024")]
#[case("25\\12\\2024")]
fn standard_separators_three_numerics(#[case] utterance: &str) {
    assert_eq!(
        tokenise(utterance, &Config::default()),
        vec![
            Token::Numeric(25, 2),
            Token::Numeric(12, 2),
            Token::Numeric(2024, 4),
        ]
    );
}

#[rstest]
#[case("25  12  2024")]
#[case("25 / 12 / 2024")]
fn consecutive_separators_no_empty_tokens(#[case] utterance: &str) {
    assert_eq!(
        tokenise(utterance, &Config::default()),
        vec![
            Token::Numeric(25, 2),
            Token::Numeric(12, 2),
            Token::Numeric(2024, 4),
        ]
    );
}

// -------------------------------------------------------------------------
// Mixed separators
// -------------------------------------------------------------------------

#[rstest]
#[case(
    "18/6. 2013",
    vec![Token::Numeric(18, 2), Token::Numeric(6, 1), Token::Numeric(2013, 4)]
)]
#[case(
    "19th  October,2015",
    vec![Token::OrdinalDay(19), Token::MonthName(MonthName::October), Token::Numeric(2015, 4)]
)]
fn mixed_separators(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

// -------------------------------------------------------------------------
// No-space digit↔month-name adjacency
// -------------------------------------------------------------------------

#[rstest]
#[case("19october", vec![Token::Numeric(19, 2), Token::MonthName(MonthName::October)])]
#[case("9july",     vec![Token::Numeric(9, 1),  Token::MonthName(MonthName::July)])]
fn digit_then_month_name_adjacent(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

#[rstest]
#[case("August7",   vec![Token::MonthName(MonthName::August),  Token::Numeric(7, 1)])]
#[case("January27", vec![Token::MonthName(MonthName::January), Token::Numeric(27, 2)])]
fn month_name_then_digit_adjacent(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

// -------------------------------------------------------------------------
// Noise words mixed in with valid date components
// -------------------------------------------------------------------------

#[rstest]
#[case(
    "I was born in 2014 October 19",
    vec![Token::Numeric(2014, 4), Token::MonthName(MonthName::October), Token::Numeric(19, 2)]
)]
#[case(
    "Friday 31",
    vec![Token::Numeric(31, 2)]
)]
#[case(
    "8 October >type 11",
    vec![Token::Numeric(8, 1), Token::MonthName(MonthName::October), Token::Numeric(11, 2)]
)]
fn noise_words_dropped(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

// -------------------------------------------------------------------------
// At-most-three token cap
// -------------------------------------------------------------------------

#[test]
fn at_most_three_tokens() {
    let result = tokenise("12 June 2024 extra 99", &Config::default());
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], Token::Numeric(12, 2));
    assert_eq!(result[1], Token::MonthName(MonthName::June));
    assert_eq!(result[2], Token::Numeric(2024, 4));
}

// -------------------------------------------------------------------------
// Extra separators
// -------------------------------------------------------------------------

#[rstest]
#[case("25||12||2024", "||")]
#[case("25::12::2024", "::")]
fn extra_separator_multi_char(#[case] utterance: &str, #[case] sep: &str) {
    let config = Config {
        extra_separators: vec![sep.to_string()],
        ..Default::default()
    };
    assert_eq!(
        tokenise(utterance, &config),
        vec![
            Token::Numeric(25, 2),
            Token::Numeric(12, 2),
            Token::Numeric(2024, 4),
        ]
    );
}

#[test]
fn extra_separator_single_char() {
    let config = Config {
        extra_separators: vec!["|".to_string()],
        ..Default::default()
    };
    assert_eq!(
        tokenise("25|12|2024", &config),
        vec![
            Token::Numeric(25, 2),
            Token::Numeric(12, 2),
            Token::Numeric(2024, 4),
        ]
    );
}

// -------------------------------------------------------------------------
// No-separator path
// -------------------------------------------------------------------------

/// 8-digit concatenated strings are sliced positionally.
#[rstest]
#[case(
    "25122024",
    order_dmy(),
    Token::Numeric(25, 2),
    Token::Numeric(12, 2),
    Token::Numeric(2024, 4)
)]
#[case(
    "12252024",
    order_mdy(),
    Token::Numeric(12, 2),
    Token::Numeric(25, 2),
    Token::Numeric(2024, 4)
)]
#[case(
    "20241225",
    order_ymd(),
    Token::Numeric(2024, 4),
    Token::Numeric(12, 2),
    Token::Numeric(25, 2)
)]
fn no_separator_eight_digits(
    #[case] utterance: &str,
    #[case] order: ComponentOrder,
    #[case] t0: Token,
    #[case] t1: Token,
    #[case] t2: Token,
) {
    let config = no_sep_config(order);
    assert_eq!(tokenise(utterance, &config), vec![t0, t1, t2]);
}

/// 6-digit concatenated strings are sliced with a 2-digit year slot.
#[rstest]
#[case(
    "251224",
    order_dmy(),
    Token::Numeric(25, 2),
    Token::Numeric(12, 2),
    Token::Numeric(24, 2)
)]
fn no_separator_six_digits(
    #[case] utterance: &str,
    #[case] order: ComponentOrder,
    #[case] t0: Token,
    #[case] t1: Token,
    #[case] t2: Token,
) {
    let config = no_sep_config(order);
    assert_eq!(tokenise(utterance, &config), vec![t0, t1, t2]);
}

/// Non-digit strings with no_separator=true fall through to normal tokenisation.
#[test]
fn no_separator_falls_through_for_non_digit_strings() {
    let config = no_sep_config(order_dmy());
    // Has a month name — not all-digit, so falls through to standard path.
    assert_eq!(
        tokenise("25 December 2024", &config),
        vec![
            Token::Numeric(25, 2),
            Token::MonthName(MonthName::December),
            Token::Numeric(2024, 4),
        ]
    );
}

// -------------------------------------------------------------------------
// Letter-O substitution
// -------------------------------------------------------------------------

/// Tokens consisting entirely of digits and the letter O are treated as
/// numerics with O→0 applied when `letter_o_substitution` is enabled (the
/// default).
#[rstest]
#[case("2o24", Token::Numeric(2024, 4))]
#[case("2O25", Token::Numeric(2025, 4))]
#[case("2O00", Token::Numeric(2000, 4))]
#[case("19oo", Token::Numeric(1900, 4))]
#[case("21OO", Token::Numeric(2100, 4))]
#[case("ooo1", Token::Numeric(1, 4))]
#[case("o6", Token::Numeric(6, 2))]
fn letter_o_substitution_enabled(#[case] utterance: &str, #[case] expected: Token) {
    assert_eq!(tokenise(utterance, &Config::default()), vec![expected]);
}

/// When `letter_o_substitution` is disabled, the O→0 substitution is not
/// performed at the chunk level, so `sub_split_on_boundary` splits the chunk
/// at digit↔alpha boundaries as normal.  Each resulting purely-numeric
/// fragment is still tokenised.  For example `"2o24"` splits into `"2"`,
/// `"o"`, and `"24"` — the `"o"` portion is not a valid month name and is
/// dropped, but `"2"` and `"24"` survive as `Numeric` tokens.
///
/// This means disabling substitution does *not* drop the token entirely;
/// it prevents the whole-chunk substitution that produces a single four-digit
/// token (e.g. `Numeric(2024, 4)`), leaving the split fragments instead.
#[rstest]
#[case("2o24", vec![Token::Numeric(2, 1), Token::Numeric(24, 2)])]
#[case("2O25", vec![Token::Numeric(2, 1), Token::Numeric(25, 2)])]
#[case("19oo", vec![Token::Numeric(19, 2)])]
#[case("21OO", vec![Token::Numeric(21, 2)])]
fn letter_o_substitution_disabled_splits_instead(
    #[case] utterance: &str,
    #[case] expected: Vec<Token>,
) {
    let config = Config {
        letter_o_substitution: false,
        ..Default::default()
    };
    assert_eq!(tokenise(utterance, &config), expected);
}

/// Letter-O substitution does not interfere with month names, because
/// sub_split_on_boundary has already split digit runs from alpha runs before
/// classify() is called.  "7october" splits into "7" (numeric) and "october"
/// (month name); the "october" token contains non-O alphabetic characters so
/// the O-only gate never fires for it.
#[rstest]
#[case(
    "7october",
    vec![Token::Numeric(7, 1), Token::MonthName(MonthName::October)]
)]
#[case(
    "november",
    vec![Token::MonthName(MonthName::November)]
)]
#[case(
    "oct",
    vec![Token::MonthName(MonthName::October)]
)]
fn letter_o_substitution_does_not_affect_month_names(
    #[case] utterance: &str,
    #[case] expected: Vec<Token>,
) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}

/// A full date string containing OCR-style letter-O in the year component is
/// correctly tokenised when substitution is enabled.
#[test]
fn letter_o_substitution_in_full_date_year() {
    assert_eq!(
        tokenise("19 October 2O14", &Config::default()),
        vec![
            Token::Numeric(19, 2),
            Token::MonthName(MonthName::October),
            Token::Numeric(2014, 4),
        ]
    );
}

// -------------------------------------------------------------------------
// Specific utterances from the test suite
// -------------------------------------------------------------------------

#[rstest]
#[case("June", vec![Token::MonthName(MonthName::June)])]
#[case("2010", vec![Token::Numeric(2010, 4)])]
#[case("16th", vec![Token::OrdinalDay(16)])]
#[case("18th", vec![Token::OrdinalDay(18)])]
#[case(
    "31 July  2014",
    vec![Token::Numeric(31, 2), Token::MonthName(MonthName::July), Token::Numeric(2014, 4)]
)]
#[case(
    "30 December 2014",
    vec![Token::Numeric(30, 2), Token::MonthName(MonthName::December), Token::Numeric(2014, 4)]
)]
#[case(
    "25th  October  2025",
    vec![Token::OrdinalDay(25), Token::MonthName(MonthName::October), Token::Numeric(2025, 4)]
)]
#[case(
    "25 October",
    vec![Token::Numeric(25, 2), Token::MonthName(MonthName::October)]
)]
#[case(
    "October 26 2012",
    vec![Token::MonthName(MonthName::October), Token::Numeric(26, 2), Token::Numeric(2012, 4)]
)]
#[case(
    "8 October 11",
    vec![Token::Numeric(8, 1), Token::MonthName(MonthName::October), Token::Numeric(11, 2)]
)]
#[case(
    "January27 2013",
    vec![Token::MonthName(MonthName::January), Token::Numeric(27, 2), Token::Numeric(2013, 4)]
)]
fn real_world_utterances(#[case] utterance: &str, #[case] expected: Vec<Token>) {
    assert_eq!(tokenise(utterance, &Config::default()), expected);
}
