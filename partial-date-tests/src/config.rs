// Tests for configuration validation and behavior.

use crate::helpers::*;
use partial_date::extract::extract;
use partial_date::models::*;
use rstest::rstest;

// -------------------------------------------------------------------------
// Default config tests
// -------------------------------------------------------------------------

/// Default Config should have sensible values per the spec.
#[test]
fn default_config_has_sensible_defaults() {
    let config = Config::default();

    // Day defaults
    assert_eq!(config.day.min, 1);
    assert_eq!(config.day.max, 31);
    assert_eq!(config.day.expected, IsExpected::Maybe);
    assert!(config.day.default.is_none());

    // Month defaults
    assert_eq!(config.month.min, 1);
    assert_eq!(config.month.max, 12);
    assert_eq!(config.month.expected, IsExpected::Maybe);
    assert!(config.month.default.is_none());

    // Year defaults
    assert_eq!(config.year.min, 0);
    assert_eq!(config.year.max, 3000);
    assert_eq!(config.year.expected, IsExpected::Maybe);
    assert!(config.year.default.is_none());
    assert_eq!(
        config.year.two_digit_expansion,
        TwoDigitYearExpansion::SlidingWindow(WindowRange::default())
    );

    // Component order and separator
    assert_eq!(config.component_order, ComponentOrder::default());
    assert!(!config.no_separator);
    assert!(config.extra_separators.is_empty());

    // Letter-O substitution is enabled by default.
    assert!(config.letter_o_substitution);

    // Single-digit year expansion is disabled by default.
    assert!(!config.year.single_digit_year_expansion);
}

/// Default DayConfig independently.
#[test]
fn default_day_config() {
    let day = DayConfig::default();
    assert_eq!(day.min, 1);
    assert_eq!(day.max, 31);
    assert_eq!(day.expected, IsExpected::Maybe);
    assert!(day.default.is_none());
}

/// Default MonthConfig independently.
#[test]
fn default_month_config() {
    let month = MonthConfig::default();
    assert_eq!(month.min, 1);
    assert_eq!(month.max, 12);
    assert_eq!(month.expected, IsExpected::Maybe);
    assert!(month.default.is_none());
}

/// Default YearConfig independently.
#[test]
fn default_year_config() {
    let year = YearConfig::default();
    assert_eq!(year.min, 0);
    assert_eq!(year.max, 3000);
    assert_eq!(year.expected, IsExpected::Maybe);
    assert!(year.default.is_none());
    assert_eq!(
        year.two_digit_expansion,
        TwoDigitYearExpansion::SlidingWindow(WindowRange::default())
    );
}

// -------------------------------------------------------------------------
// Valid configs with different combinations of provided values
// -------------------------------------------------------------------------

/// Config with only day defaults set.
#[test]
fn config_with_day_default_only() {
    let config = Config {
        day: DayConfig {
            default: Some(15),
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config("June 2024", config);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Defaulted(15));
    assert_eq!(result.month.number, Extracted::Found(6));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

/// Config with all defaults set.
#[test]
fn config_with_all_defaults() {
    let config = Config {
        day: DayConfig {
            default: Some(1),
            ..Default::default()
        },
        month: MonthConfig {
            default: Some(1),
            ..Default::default()
        },
        year: YearConfig {
            default: Some(2025),
            ..Default::default()
        },
        ..Default::default()
    };
    // Providing only a day — month and year should be defaulted.
    let input = input_with_config("15", config);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(15));
    assert_eq!(result.month.number, Extracted::Defaulted(1));
    assert_eq!(result.year.value, Extracted::Defaulted(2025));
}

/// Custom min/max: values within range should be found.
#[rstest]
#[case("15/03/2025", 15, 3, 2025)]
#[case("01/01/2020", 1, 1, 2020)]
#[case("28/06/2030", 28, 6, 2030)]
fn config_custom_min_max_within_range(
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        day: DayConfig {
            min: 1,
            max: 28,
            ..Default::default()
        },
        month: MonthConfig {
            min: 1,
            max: 6,
            ..Default::default()
        },
        year: YearConfig {
            min: 2020,
            max: 2030,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// Custom min/max: day outside max=28 should not be found.
#[rstest]
#[case("29/03/2025")]
#[case("31/03/2025")]
fn config_custom_min_max_day_out_of_range(#[case] utterance: &str) {
    let config = Config {
        day: DayConfig {
            min: 1,
            max: 28,
            ..Default::default()
        },
        month: MonthConfig {
            min: 1,
            max: 6,
            ..Default::default()
        },
        year: YearConfig {
            min: 2020,
            max: 2030,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.day.value.is_not_found());
}

/// Custom min/max: month outside max=6 should not be found.
#[rstest]
#[case("15/07/2025")]
#[case("15/12/2025")]
fn config_custom_min_max_month_out_of_range(#[case] utterance: &str) {
    let config = Config {
        day: DayConfig {
            min: 1,
            max: 28,
            ..Default::default()
        },
        month: MonthConfig {
            min: 1,
            max: 6,
            ..Default::default()
        },
        year: YearConfig {
            min: 2020,
            max: 2030,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.month.number.is_not_found());
}

/// Custom min/max: year outside min=2020/max=2030 should not be found.
#[rstest]
#[case("15/03/2019")]
#[case("15/03/2031")]
fn config_custom_min_max_year_out_of_range(#[case] utterance: &str) {
    let config = Config {
        day: DayConfig {
            min: 1,
            max: 28,
            ..Default::default()
        },
        month: MonthConfig {
            min: 1,
            max: 6,
            ..Default::default()
        },
        year: YearConfig {
            min: 2020,
            max: 2030,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.year.value.is_not_found());
}

/// Config with different IsExpected combinations.
#[test]
fn config_with_expected_yes_for_all() {
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
            expected: IsExpected::Yes,
            ..Default::default()
        },
        ..Default::default()
    };
    let input = input_with_config("25/12/2024", config);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

// -------------------------------------------------------------------------
// Config with ComponentOrder
// -------------------------------------------------------------------------

/// All six valid ComponentOrder permutations can be constructed and set.
#[rstest]
#[case(order_dmy(), "25/12/2024", 25, 12, 2024)]
#[case(order_mdy(), "12/25/2024", 25, 12, 2024)]
#[case(order_ymd(), "2024/12/25", 25, 12, 2024)]
#[case(order_ydm(), "2024/25/12", 25, 12, 2024)]
#[case(order_myd(), "12/2024/25", 25, 12, 2024)]
fn config_custom_component_order(
    #[case] order: ComponentOrder,
    #[case] utterance: &str,
    #[case] expected_day: u8,
    #[case] expected_month: u8,
    #[case] expected_year: i32,
) {
    let config = Config {
        component_order: order,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(expected_day));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// ComponentOrder::new rejects duplicate components.
#[rstest]
#[case(DateComponent::Day, DateComponent::Day, DateComponent::Month)]
#[case(DateComponent::Month, DateComponent::Year, DateComponent::Month)]
#[case(DateComponent::Year, DateComponent::Month, DateComponent::Year)]
fn config_component_order_rejects_duplicates(
    #[case] first: DateComponent,
    #[case] second: DateComponent,
    #[case] third: DateComponent,
) {
    let result = ComponentOrder::new(first, second, third);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ComponentOrderError::DuplicateComponent(_)
    ));
}

/// ComponentOrder::new accepts all valid (non-duplicate) orderings.
#[rstest]
#[case(DateComponent::Day, DateComponent::Month, DateComponent::Year)]
#[case(DateComponent::Day, DateComponent::Year, DateComponent::Month)]
#[case(DateComponent::Month, DateComponent::Day, DateComponent::Year)]
#[case(DateComponent::Month, DateComponent::Year, DateComponent::Day)]
#[case(DateComponent::Year, DateComponent::Day, DateComponent::Month)]
#[case(DateComponent::Year, DateComponent::Month, DateComponent::Day)]
fn config_component_order_accepts_valid_orderings(
    #[case] first: DateComponent,
    #[case] second: DateComponent,
    #[case] third: DateComponent,
) {
    let result = ComponentOrder::new(first, second, third);
    assert!(result.is_ok());
}

// -------------------------------------------------------------------------
// Config with different separator variants
// -------------------------------------------------------------------------

/// The extractor automatically handles all standard separators without any
/// config — each of these inputs should produce the same result.
#[rstest]
#[case("25/12/2024")]
#[case("25-12-2024")]
#[case("25.12.2024")]
#[case("25\\12\\2024")]
#[case("25,12,2024")]
#[case("25 12 2024")]
fn config_separator_variants(#[case] utterance: &str) {
    let result = extract(input_with_config(utterance, config_with_order(order_dmy())));

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

/// Extra separators: custom separator strings added via `extra_separators`
/// are tried in addition to the standard set.
#[rstest]
#[case("||", "25||12||2024")]
#[case("::", "25::12::2024")]
#[case(" - ", "25 - 12 - 2024")]
fn config_extra_separator(#[case] sep: &str, #[case] utterance: &str) {
    let config = Config {
        extra_separators: vec![sep.to_string()],
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));

    assert_eq!(result.day.value, Extracted::Found(25));
    assert_eq!(result.month.number, Extracted::Found(12));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

/// Mixed separators within a single string are handled automatically.
#[rstest]
#[case("18/6. 2013", 18, 6, 2013)]
#[case("19th October,2015", 19, 10, 2015)]
fn config_mixed_separators(
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
// Config: per-call config overrides library defaults
// -------------------------------------------------------------------------

/// When both library defaults and per-call config exist,
/// per-call config should take precedence.
#[test]
fn per_call_config_overrides_defaults() {
    // Per-call config with Month → Day → Year order
    let config = Config {
        component_order: order_mdy(),
        ..Default::default()
    };
    // "01/06/2024" with MDY order: month=1, day=6
    let input = input_with_config("01/06/2024", config);
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(6));
    assert_eq!(result.month.number, Extracted::Found(1));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

/// Input with no config should use library defaults (DDMMYYYY).
#[test]
fn no_config_uses_library_defaults() {
    // "01/06/2024" with default DDMMYYYY: day=1, month=6
    let input = input("01/06/2024");
    let result = extract(input);

    assert_eq!(result.day.value, Extracted::Found(1));
    assert_eq!(result.month.number, Extracted::Found(6));
    assert_eq!(result.year.value, Extracted::Found(2024));
}

// -------------------------------------------------------------------------
// Config with TwoDigitYearExpansion variants
// -------------------------------------------------------------------------

/// SlidingWindow is the default and should work without explicit setting.
#[test]
fn config_default_year_expansion_is_sliding_window() {
    let config = Config::default();
    assert_eq!(
        config.year.two_digit_expansion,
        TwoDigitYearExpansion::SlidingWindow(WindowRange::default())
    );
}

/// All three expansion modes should be settable.
#[rstest]
#[case(TwoDigitYearExpansion::SlidingWindow(WindowRange::default()))]
#[case(TwoDigitYearExpansion::Always2000s)]
#[case(TwoDigitYearExpansion::Literal)]
fn config_year_expansion_mode_settable(#[case] expansion: TwoDigitYearExpansion) {
    let config = YearConfig {
        two_digit_expansion: expansion,
        ..Default::default()
    };
    assert_eq!(config.two_digit_expansion, expansion);
}

// -------------------------------------------------------------------------
// WindowRange validation
// -------------------------------------------------------------------------

/// Default WindowRange should have the standard 00–49 → 2000–2049, 50–99 → 1950–1999 mapping.
#[test]
fn window_range_default_values() {
    let wr = WindowRange::default();
    assert_eq!(
        wr.lower_range,
        Range {
            min: 2000,
            max: 2050
        }
    );
    assert_eq!(
        wr.upper_range,
        Range {
            min: 1950,
            max: 2000
        }
    );
}

/// WindowRange::new should accept valid, contiguous, non-overlapping ranges that span 100 years.
#[rstest]
#[case(
    Range { min: 2000, max: 2050 },
    Range { min: 1950, max: 2000 },
)]
#[case(
    Range { min: 2000, max: 2070 },
    Range { min: 1970, max: 2000 },
)]
#[case(
    Range { min: 1900, max: 1950 },
    Range { min: 1950, max: 2000 },
)]
fn window_range_new_valid(#[case] lower: Range, #[case] upper: Range) {
    let result = WindowRange::new(lower, upper);
    assert!(result.is_ok());
    let wr = result.unwrap();
    assert_eq!(wr.lower_range, lower);
    assert_eq!(wr.upper_range, upper);
}

/// WindowRange::new should reject empty ranges (min >= max).
#[rstest]
#[case(
    Range { min: 2050, max: 2050 },
    Range { min: 1950, max: 2000 },
)]
#[case(
    Range { min: 2050, max: 2000 },
    Range { min: 1950, max: 2000 },
)]
#[case(
    Range { min: 2000, max: 2050 },
    Range { min: 2000, max: 2000 },
)]
fn window_range_new_empty_range(#[case] lower: Range, #[case] upper: Range) {
    let result = WindowRange::new(lower, upper);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        WindowRangeError::EmptyRange { .. }
    ));
}

/// WindowRange::new should reject overlapping ranges.
#[rstest]
#[case(
    Range { min: 1990, max: 2050 },
    Range { min: 2040, max: 2100 },
)]
#[case(
    Range { min: 2000, max: 2060 },
    Range { min: 2050, max: 2100 },
)]
fn window_range_new_overlapping(#[case] lower: Range, #[case] upper: Range) {
    let result = WindowRange::new(lower, upper);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), WindowRangeError::Overlapping));
}

/// WindowRange::new should reject ranges that don't span exactly 100 years.
#[rstest]
#[case(
    Range { min: 2000, max: 2050 },
    Range { min: 1950, max: 1999 },
)]
#[case(
    Range { min: 2001, max: 2060 },
    Range { min: 1960, max: 2000 },
)]
fn window_range_new_invalid_total_span(#[case] lower: Range, #[case] upper: Range) {
    let result = WindowRange::new(lower, upper);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        WindowRangeError::InvalidTotalSpan { .. }
    ));
}

/// WindowRange::new should reject ranges with a gap between them.
#[rstest]
#[case(
    Range { min: 2000, max: 2050 },
    Range { min: 1900, max: 1950 },
)]
fn window_range_new_gap(#[case] lower: Range, #[case] upper: Range) {
    let result = WindowRange::new(lower, upper);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), WindowRangeError::Gap));
}

// -------------------------------------------------------------------------
// Config: letter_o_substitution
// -------------------------------------------------------------------------

/// When `letter_o_substitution` is enabled (the default), a token that looks
/// like a number but contains the letter O is interpreted with O→0 applied.
/// This covers common OCR and typing errors such as "2O24" being entered
/// instead of "2024".
#[rstest]
#[case("2o24", 2024)]
#[case("2O25", 2025)]
#[case("2O00", 2000)]
#[case("19oo", 1900)]
#[case("21OO", 2100)]
#[case("ooo1", 1)]
fn config_letter_o_substitution_enabled_year(#[case] utterance: &str, #[case] expected_year: i32) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
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
        letter_o_substitution: true,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// When `letter_o_substitution` is disabled, a token whose year-significance
/// depends entirely on the O→0 substitution is not recognised.  These cases
/// use inputs where the non-O numeric fragments are too short (1 digit) to be
/// a valid year on their own, so disabling substitution leaves no valid year.
///
/// For example `"2ooo"` → without substitution the tokeniser produces only
/// `Numeric(2, 1)` (one digit); 1-digit values are not valid year candidates,
/// so the year is NotFound.
#[rstest]
#[case("2ooo")]
#[case("2OOO")]
#[case("1oo4")]
fn config_letter_o_substitution_disabled_year_not_found(#[case] utterance: &str) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
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
        letter_o_substitution: false,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert!(result.year.value.is_not_found());
}

/// When `letter_o_substitution` is enabled, the substitution does NOT apply to
/// a token that is a valid month name containing the letter O (e.g. "october",
/// "november"). The month name classification takes precedence over numeric
/// substitution because the sub-token produced by sub_split_on_boundary for a
/// word like "october" contains non-O alphabetic characters, so the O-only
/// check never fires.
#[rstest]
#[case("october", 10)]
#[case("november", 11)]
#[case("oct", 10)]
fn config_letter_o_substitution_does_not_affect_month_names(
    #[case] utterance: &str,
    #[case] expected_month: u8,
) {
    let config = Config {
        month: MonthConfig {
            expected: IsExpected::Yes,
            ..Default::default()
        },
        day: DayConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        year: YearConfig {
            expected: IsExpected::No,
            ..Default::default()
        },
        letter_o_substitution: true,
        ..Default::default()
    };
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.month.number, Extracted::Found(expected_month));
}

// -------------------------------------------------------------------------
// Config: single_digit_year_expansion
// -------------------------------------------------------------------------

/// When `single_digit_year_expansion` is enabled, a single-digit token is
/// treated as a two-digit year with a leading zero prepended (`5` → `05`),
/// then expanded by the configured `two_digit_expansion` strategy.
///
/// With the default `SlidingWindow` strategy `0`–`49` map to 2000–2049, so
/// `5` → `05` → `2005`.
#[rstest]
#[case("1", 2001)]
#[case("5", 2005)]
#[case("9", 2009)]
fn config_single_digit_year_expansion_sliding_window(
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            single_digit_year_expansion: true,
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
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// With `single_digit_year_expansion` and `TwoDigitYearExpansion::Literal`,
/// a single-digit value is treated as the literal year `0N` (e.g. `5` → year 5).
/// This is useful for historical dates where year 5 genuinely means AD 5.
#[rstest]
#[case("1", 1)]
#[case("5", 5)]
#[case("9", 9)]
fn config_single_digit_year_expansion_literal(#[case] utterance: &str, #[case] expected_year: i32) {
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
    let result = extract(input_with_config(utterance, config));
    assert_eq!(result.year.value, Extracted::Found(expected_year));
}

/// When `single_digit_year_expansion` is disabled (the default), a lone
/// single-digit token is never treated as a year — it can only be a day or
/// month.
#[rstest]
#[case("1")]
#[case("5")]
#[case("9")]
fn config_single_digit_year_expansion_disabled_year_not_found(#[case] utterance: &str) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            single_digit_year_expansion: false,
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
    let result = extract(input_with_config(utterance, config));
    assert!(result.year.value.is_not_found());
}

/// Single-digit year expansion applies when day and month are already filled
/// by other tokens, leaving the single digit for the year slot.
/// Input: "1 January 5" (DMY) — day=1 (ordinal), month=January (name),
/// year=5 → expanded to 2005 with sliding window.
#[test]
fn config_single_digit_year_expansion_with_full_date_context() {
    let config = Config {
        year: YearConfig {
            single_digit_year_expansion: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = extract(input_with_config("1 January 5", config));
    assert_eq!(result.day.value, Extracted::Found(1));
    assert_eq!(result.month.number, Extracted::Found(1));
    assert_eq!(result.year.value, Extracted::Found(2005));
}

/// A custom WindowRange can be used in TwoDigitYearExpansion.
#[test]
fn config_custom_window_range() {
    let wr = WindowRange::new(
        Range {
            min: 2000,
            max: 2070,
        },
        Range {
            min: 1970,
            max: 2000,
        },
    )
    .unwrap();
    let config = Config {
        year: YearConfig {
            two_digit_expansion: TwoDigitYearExpansion::SlidingWindow(wr),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(
        config.year.two_digit_expansion,
        TwoDigitYearExpansion::SlidingWindow(wr)
    );
}
