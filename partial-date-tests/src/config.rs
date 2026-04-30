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
        TwoDigitYearExpansion::default()
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
    assert_eq!(year.two_digit_expansion, TwoDigitYearExpansion::default());
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
        TwoDigitYearExpansion::default()
    );
}

/// All three expansion modes should be settable.
#[rstest]
#[case(TwoDigitYearExpansion::SlidingWindow {
    earliest_year: 1950,
    pivot: SlidingWindowPivot::new(50),
})]
#[case(TwoDigitYearExpansion::Always(Century::new(2000)))]
#[case(TwoDigitYearExpansion::Literal)]
fn config_year_expansion_mode_settable(#[case] expansion: TwoDigitYearExpansion) {
    let config = YearConfig {
        two_digit_expansion: expansion,
        ..Default::default()
    };
    assert_eq!(config.two_digit_expansion, expansion);
}

// -------------------------------------------------------------------------
// SlidingWindowPivot validation
// -------------------------------------------------------------------------

/// SlidingWindowPivot::new should succeed for values in the range 1–99.
#[rstest]
#[case(1)]
#[case(50)]
#[case(70)]
#[case(99)]
fn sliding_window_pivot_new_valid(#[case] pivot: u8) {
    // new() returns Self directly — just confirm it doesn't panic.
    let _ = SlidingWindowPivot::new(pivot);
}

/// SlidingWindowPivot::new should panic for 0.
#[test]
#[should_panic(expected = "SlidingWindowPivot must be in the range 1–99")]
fn sliding_window_pivot_new_panics_on_zero() {
    let _ = SlidingWindowPivot::new(0);
}

/// SlidingWindowPivot::new should panic for values >= 100.
#[test]
#[should_panic(expected = "SlidingWindowPivot must be in the range 1–99")]
fn sliding_window_pivot_new_panics_on_over_99() {
    let _ = SlidingWindowPivot::new(100);
}

/// SlidingWindowPivot::try_new should return Ok for values in the range 1–99.
#[rstest]
#[case(1)]
#[case(50)]
#[case(70)]
#[case(99)]
fn sliding_window_pivot_try_new_valid(#[case] pivot: u8) {
    assert!(SlidingWindowPivot::try_new(pivot).is_ok());
}

/// SlidingWindowPivot::try_new should return Err for 0 and values >= 100.
#[rstest]
#[case(0)]
#[case(100)]
#[case(255)]
fn sliding_window_pivot_try_new_invalid(#[case] pivot: u8) {
    let result = SlidingWindowPivot::try_new(pivot);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        SlidingWindowPivotError::InvalidPivot(_)
    ));
}

/// TryFrom<u8> for SlidingWindowPivot delegates to try_new.
#[test]
fn sliding_window_pivot_try_from_u8() {
    let pivot: Result<SlidingWindowPivot, _> = 50_u8.try_into();
    assert!(pivot.is_ok());
    let invalid: Result<SlidingWindowPivot, _> = 0_u8.try_into();
    assert!(invalid.is_err());
}

/// From<SlidingWindowPivot> for u8 round-trips the value.
#[test]
fn sliding_window_pivot_into_u8() {
    let pivot = SlidingWindowPivot::new(50);
    let value: u8 = pivot.into();
    assert_eq!(value, 50);
}

// -------------------------------------------------------------------------
// Century validation
// -------------------------------------------------------------------------

/// Century::new should succeed for values divisible by 100.
#[rstest]
#[case(0)]
#[case(100)]
#[case(1800)]
#[case(2000)]
#[case(2100)]
fn century_new_valid(#[case] year: i32) {
    // new() returns Self directly — just confirm it doesn't panic.
    let _ = Century::new(year);
}

/// Century::new should panic for values not divisible by 100.
#[test]
#[should_panic(expected = "Century must be divisible by 100")]
fn century_new_panics_on_non_boundary() {
    let _ = Century::new(1756);
}

/// Century::try_new should return Ok for values divisible by 100.
#[rstest]
#[case(0)]
#[case(100)]
#[case(1800)]
#[case(2000)]
#[case(2100)]
fn century_try_new_valid(#[case] year: i32) {
    assert!(Century::try_new(year).is_ok());
}

/// Century::try_new should return Err for values not divisible by 100.
#[rstest]
#[case(1)]
#[case(1756)]
#[case(1801)]
#[case(2025)]
fn century_try_new_invalid(#[case] year: i32) {
    let result = Century::try_new(year);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CenturyError::NotACenturyBoundary(_)
    ));
}

/// TryFrom<i32> for Century delegates to try_new.
#[test]
fn century_try_from_i32() {
    let century: Result<Century, _> = 1800_i32.try_into();
    assert!(century.is_ok());
    let invalid: Result<Century, _> = 1756_i32.try_into();
    assert!(invalid.is_err());
}

/// From<Century> for i32 round-trips the value.
#[test]
fn century_into_i32() {
    let century = Century::new(1800);
    let value: i32 = century.into();
    assert_eq!(value, 1800);
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

/// A custom SlidingWindow with a non-default pivot can be stored in YearConfig.
#[test]
fn config_custom_sliding_window() {
    let pivot = SlidingWindowPivot::new(70);
    let expansion = TwoDigitYearExpansion::SlidingWindow {
        earliest_year: 1970,
        pivot,
    };
    let config = Config {
        year: YearConfig {
            two_digit_expansion: expansion,
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(config.year.two_digit_expansion, expansion);
}

// -------------------------------------------------------------------------
// Config with TwoDigitYearExpansion::Always(Century)
// -------------------------------------------------------------------------

/// Always(Century) maps all two-digit values into the given century.
/// 00 → century, 99 → century + 99.
#[rstest]
#[case(Century::new(2000), "00", 2000)]
#[case(Century::new(2000), "34", 2034)]
#[case(Century::new(2000), "99", 2099)]
#[case(Century::new(1800), "00", 1800)]
#[case(Century::new(1800), "34", 1834)]
#[case(Century::new(1800), "99", 1899)]
fn config_always_century_expansion(
    #[case] century: Century,
    #[case] utterance: &str,
    #[case] expected_year: i32,
) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always(century),
            min: 0,
            max: 3000,
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

/// Always(Century) with a YearConfig::min/max that excludes part of the
/// century: values that expand outside the range should return NotFound.
#[rstest]
#[case(Century::new(1800), "00", 1800, 1850)] // 00 → 1800, inside range
#[case(Century::new(1800), "50", 1800, 1850)] // 50 → 1850, at boundary (inclusive)
#[case(Century::new(1800), "51", 1800, 1850)] // 51 → 1851, outside range
#[case(Century::new(1800), "99", 1800, 1850)] // 99 → 1899, outside range
fn config_always_century_expansion_clamped_by_min_max(
    #[case] century: Century,
    #[case] utterance: &str,
    #[case] year_min: i32,
    #[case] year_max: i32,
) {
    let config = Config {
        year: YearConfig {
            expected: IsExpected::Yes,
            two_digit_expansion: TwoDigitYearExpansion::Always(century),
            min: year_min,
            max: year_max,
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
    // "51" and "99" should be NotFound; "00" and "50" should be Found.
    let expanded = i32::from(century) + utterance.parse::<i32>().unwrap();
    if expanded >= year_min && expanded <= year_max {
        assert_eq!(result.year.value, Extracted::Found(expanded));
    } else {
        assert!(result.year.value.is_not_found());
    }
}

// -------------------------------------------------------------------------
// Builder pattern: DayConfig
// -------------------------------------------------------------------------

/// with_range sets min and max on DayConfig.
#[test]
fn day_config_builder_with_range() {
    let config = DayConfig::default().with_range(1, 28);
    assert_eq!(config.min, 1);
    assert_eq!(config.max, 28);
}

/// with_range preserves other fields unchanged.
#[test]
fn day_config_builder_with_range_preserves_other_fields() {
    let config = DayConfig::default()
        .with_expected(IsExpected::Yes)
        .with_range(5, 25);
    assert_eq!(config.expected, IsExpected::Yes);
    assert_eq!(config.min, 5);
    assert_eq!(config.max, 25);
}

/// with_range panics when min > max.
#[test]
#[should_panic(expected = "DayConfig::with_range min")]
fn day_config_builder_with_range_panics_when_min_exceeds_max() {
    let _ = DayConfig::default().with_range(28, 1);
}

/// try_with_range returns Ok when min <= max.
#[test]
fn day_config_builder_try_with_range_ok() {
    let result = DayConfig::default().try_with_range(1, 28);
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.min, 1);
    assert_eq!(config.max, 28);
}

/// try_with_range returns Err when min > max.
#[test]
fn day_config_builder_try_with_range_err() {
    let result = DayConfig::default().try_with_range(28, 1);
    assert!(matches!(
        result.unwrap_err(),
        ConfigRangeError::MinExceedsMax { min: 28, max: 1 }
    ));
}

/// with_expected sets the expected field.
#[rstest]
#[case(IsExpected::Yes)]
#[case(IsExpected::No)]
#[case(IsExpected::Maybe)]
fn day_config_builder_with_expected(#[case] expected: IsExpected) {
    let config = DayConfig::default().with_expected(expected);
    assert_eq!(config.expected, expected);
}

/// with_default sets the default field to Some.
#[test]
fn day_config_builder_with_default() {
    let config = DayConfig::default().with_default(15);
    assert_eq!(config.default, Some(15));
}

/// Builder methods on DayConfig can be chained together.
#[test]
fn day_config_builder_chained() {
    let config = DayConfig::default()
        .with_range(1, 28)
        .with_expected(IsExpected::Yes)
        .with_default(1);
    assert_eq!(config.min, 1);
    assert_eq!(config.max, 28);
    assert_eq!(config.expected, IsExpected::Yes);
    assert_eq!(config.default, Some(1));
}

// -------------------------------------------------------------------------
// Builder pattern: MonthConfig
// -------------------------------------------------------------------------

/// with_range sets min and max on MonthConfig.
#[test]
fn month_config_builder_with_range() {
    let config = MonthConfig::default().with_range(3, 9);
    assert_eq!(config.min, 3);
    assert_eq!(config.max, 9);
}

/// with_range panics when min > max.
#[test]
#[should_panic(expected = "MonthConfig::with_range min")]
fn month_config_builder_with_range_panics_when_min_exceeds_max() {
    let _ = MonthConfig::default().with_range(9, 3);
}

/// try_with_range returns Ok when min <= max.
#[test]
fn month_config_builder_try_with_range_ok() {
    let result = MonthConfig::default().try_with_range(3, 9);
    assert!(result.is_ok());
}

/// try_with_range returns Err when min > max.
#[test]
fn month_config_builder_try_with_range_err() {
    let result = MonthConfig::default().try_with_range(9, 3);
    assert!(matches!(
        result.unwrap_err(),
        ConfigRangeError::MinExceedsMax { min: 9, max: 3 }
    ));
}

/// Builder methods on MonthConfig can be chained together.
#[test]
fn month_config_builder_chained() {
    let config = MonthConfig::default()
        .with_range(1, 6)
        .with_expected(IsExpected::Yes)
        .with_default(1);
    assert_eq!(config.min, 1);
    assert_eq!(config.max, 6);
    assert_eq!(config.expected, IsExpected::Yes);
    assert_eq!(config.default, Some(1));
}

// -------------------------------------------------------------------------
// Builder pattern: YearConfig
// -------------------------------------------------------------------------

/// with_range sets min and max on YearConfig.
#[test]
fn year_config_builder_with_range() {
    let config = YearConfig::default().with_range(1760, 1840);
    assert_eq!(config.min, 1760);
    assert_eq!(config.max, 1840);
}

/// with_range panics when min > max.
#[test]
#[should_panic(expected = "YearConfig::with_range min")]
fn year_config_builder_with_range_panics_when_min_exceeds_max() {
    let _ = YearConfig::default().with_range(1840, 1760);
}

/// try_with_range returns Ok when min <= max.
#[test]
fn year_config_builder_try_with_range_ok() {
    let result = YearConfig::default().try_with_range(1760, 1840);
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.min, 1760);
    assert_eq!(config.max, 1840);
}

/// try_with_range returns Err when min > max.
#[test]
fn year_config_builder_try_with_range_err() {
    let result = YearConfig::default().try_with_range(1840, 1760);
    assert!(matches!(
        result.unwrap_err(),
        ConfigRangeError::MinExceedsMax {
            min: 1840,
            max: 1760
        }
    ));
}

/// with_expected sets the expected field on YearConfig.
#[test]
fn year_config_builder_with_expected() {
    let config = YearConfig::default().with_expected(IsExpected::Yes);
    assert_eq!(config.expected, IsExpected::Yes);
}

/// with_default sets the default field on YearConfig.
#[test]
fn year_config_builder_with_default() {
    let config = YearConfig::default().with_default(2025);
    assert_eq!(config.default, Some(2025));
}

/// with_two_digit_expansion sets the expansion strategy.
#[test]
fn year_config_builder_with_two_digit_expansion() {
    let expansion = TwoDigitYearExpansion::Always(Century::new(1800));
    let config = YearConfig::default().with_two_digit_expansion(expansion);
    assert_eq!(config.two_digit_expansion, expansion);
}

/// with_single_digit_expansion enables single-digit year expansion.
#[test]
fn year_config_builder_with_single_digit_expansion() {
    let config = YearConfig::default().with_single_digit_expansion(true);
    assert!(config.single_digit_year_expansion);
}

/// Builder methods on YearConfig can be fully chained.
#[test]
fn year_config_builder_chained() {
    let config = YearConfig::default()
        .with_range(1760, 1840)
        .with_expected(IsExpected::Yes)
        .with_two_digit_expansion(TwoDigitYearExpansion::SlidingWindow {
            earliest_year: 1750,
            pivot: SlidingWindowPivot::new(50),
        })
        .with_single_digit_expansion(false);
    assert_eq!(config.min, 1760);
    assert_eq!(config.max, 1840);
    assert_eq!(config.expected, IsExpected::Yes);
    assert!(!config.single_digit_year_expansion);
}

/// A fully chained YearConfig produces correct extraction results.
#[rstest]
#[case("00", Some(1800))]
#[case("34", Some(1834))]
#[case("50", None)] // 1750 < min 1760 → rejected
#[case("60", Some(1760))]
#[case("99", Some(1799))]
fn year_config_builder_extraction(#[case] utterance: &str, #[case] expected: Option<i32>) {
    let config = Config::default().with_year(
        YearConfig::default()
            .with_range(1760, 1840)
            .with_expected(IsExpected::Yes)
            .with_two_digit_expansion(TwoDigitYearExpansion::SlidingWindow {
                earliest_year: 1750,
                pivot: SlidingWindowPivot::new(50),
            }),
    );
    let result = extract(input_with_config(utterance, config));
    match expected {
        Some(year) => assert_eq!(result.year.value, Extracted::Found(year)),
        None => assert!(result.year.value.is_not_found()),
    }
}

// -------------------------------------------------------------------------
// Builder pattern: Config
// -------------------------------------------------------------------------

/// with_day replaces the day sub-config on Config.
#[test]
fn config_builder_with_day() {
    let config = Config::default().with_day(DayConfig::default().with_range(1, 28));
    assert_eq!(config.day.max, 28);
}

/// with_month replaces the month sub-config on Config.
#[test]
fn config_builder_with_month() {
    let config =
        Config::default().with_month(MonthConfig::default().with_expected(IsExpected::Yes));
    assert_eq!(config.month.expected, IsExpected::Yes);
}

/// with_year replaces the year sub-config on Config.
#[test]
fn config_builder_with_year() {
    let config = Config::default().with_year(YearConfig::default().with_range(1760, 1840));
    assert_eq!(config.year.min, 1760);
    assert_eq!(config.year.max, 1840);
}

/// with_component_order sets the component order.
#[test]
fn config_builder_with_component_order() {
    let order = ComponentOrder::new(
        DateComponent::Month,
        DateComponent::Day,
        DateComponent::Year,
    )
    .unwrap();
    let config = Config::default().with_component_order(order);
    assert_eq!(config.component_order.first, DateComponent::Month);
}

/// with_no_separator enables no-separator parsing.
#[test]
fn config_builder_with_no_separator() {
    let config = Config::default().with_no_separator(true);
    assert!(config.no_separator);
}

/// with_extra_separators sets additional separators.
#[test]
fn config_builder_with_extra_separators() {
    let config = Config::default().with_extra_separators(vec!["||".to_string(), " - ".to_string()]);
    assert_eq!(config.extra_separators.len(), 2);
}

/// with_letter_o_substitution disables the substitution.
#[test]
fn config_builder_with_letter_o_substitution() {
    let config = Config::default().with_letter_o_substitution(false);
    assert!(!config.letter_o_substitution);
}

/// A fully chained Config produces correct extraction results.
#[test]
fn config_builder_full_chain_extraction() {
    let config = Config::default()
        .with_day(DayConfig::default().with_expected(IsExpected::Yes))
        .with_month(MonthConfig::default().with_expected(IsExpected::Yes))
        .with_year(
            YearConfig::default()
                .with_range(2000, 2099)
                .with_expected(IsExpected::Yes)
                .with_two_digit_expansion(TwoDigitYearExpansion::Always(Century::new(2000))),
        )
        .with_component_order(
            ComponentOrder::new(
                DateComponent::Day,
                DateComponent::Month,
                DateComponent::Year,
            )
            .unwrap(),
        );
    let result = extract(input_with_config("15/06/24", config));
    assert_eq!(result.day.value, Extracted::Found(15));
    assert_eq!(result.month.number, Extracted::Found(6));
    assert_eq!(result.year.value, Extracted::Found(2024));
}
