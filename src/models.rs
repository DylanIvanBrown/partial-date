//! Core types for the partial-date library.
//!
//! This module contains all the structs and enums that describe inputs,
//! configuration, and extraction results.

// ---------------------------------------------------------------------------
// Result types
// ---------------------------------------------------------------------------

/// The outcome of attempting to extract a single date component (day, month, or year).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extracted<T> {
    /// The value was found directly in the input.
    Found(T),
    /// No value could be found and no default was configured.
    NotFound,
    /// The value was not found in the input but a default was applied.
    Defaulted(T),
}

impl<T> Extracted<T> {
    /// Returns `true` if the value was found in the input.
    pub fn is_found(&self) -> bool {
        matches!(self, Extracted::Found(_))
    }

    /// Returns `true` if no value was found and no default applied.
    pub fn is_not_found(&self) -> bool {
        matches!(self, Extracted::NotFound)
    }

    /// Returns `true` if the value was defaulted.
    pub fn is_defaulted(&self) -> bool {
        matches!(self, Extracted::Defaulted(_))
    }

    /// Returns the inner value regardless of whether it was found or defaulted.
    /// Returns `None` if `NotFound`.
    pub fn value(&self) -> Option<&T> {
        match self {
            Extracted::Found(v) | Extracted::Defaulted(v) => Some(v),
            Extracted::NotFound => None,
        }
    }
}

/// A fully-resolved (possibly partial) date returned by the extractor.
#[derive(Debug)]
pub struct PartialDate {
    pub day: Day,
    pub month: Month,
    pub year: Year,
}

/// Extracted day component (1–31).
#[derive(Debug)]
pub struct Day {
    pub value: Extracted<u8>,
}

/// Extracted month component (1–12).
#[derive(Debug)]
pub struct Month {
    pub number: Extracted<u8>,
    pub name: Extracted<MonthName>,
}

/// Extracted year component.
///
/// Uses `i32` to accommodate the full range required by the spec (0–3000) and
/// to leave room for historical (negative / BC) years if needed in future.
#[derive(Debug)]
pub struct Year {
    pub value: Extracted<i32>,
}

// ---------------------------------------------------------------------------
// Configuration types
// ---------------------------------------------------------------------------

/// Indicates whether a date component is expected to be present in the input.
///
/// Used to guide disambiguation when the same token could be interpreted as
/// more than one component (e.g. `12/06` could be DD/MM or MM/DD).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IsExpected {
    /// The component is definitely expected.
    Yes,
    /// The component is definitely not expected.
    No,
    /// No strong expectation either way (the default).
    #[default]
    Maybe,
}

/// Configuration for day extraction.
#[derive(Debug, Clone)]
pub struct DayConfig {
    /// Minimum valid day value (inclusive). Default: `1`.
    pub min: u8,
    /// Maximum valid day value (inclusive). Default: `31`.
    pub max: u8,
    /// Whether a day component is expected in the input.
    pub expected: IsExpected,
    /// Default day value to use when the day is not found, if any.
    pub default: Option<u8>,
}

impl Default for DayConfig {
    fn default() -> Self {
        DayConfig {
            min: 1,
            max: 31,
            expected: IsExpected::Maybe,
            default: None,
        }
    }
}

/// Configuration for month extraction.
#[derive(Debug, Clone)]
pub struct MonthConfig {
    /// Minimum valid month value (inclusive). Default: `1`.
    pub min: u8,
    /// Maximum valid month value (inclusive). Default: `12`.
    pub max: u8,
    /// Whether a month component is expected in the input.
    pub expected: IsExpected,
    /// Default month value to use when the month is not found, if any.
    pub default: Option<u8>,
}

impl Default for MonthConfig {
    fn default() -> Self {
        MonthConfig {
            min: 1,
            max: 12,
            expected: IsExpected::Maybe,
            default: None,
        }
    }
}

/// Strategy for expanding two-digit years into four-digit years.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoDigitYearExpansion {
    /// Sliding window: 00–49 → 2000–2049, 50–99 → 1950–1999.
    /// This is the default.
    SlidingWindow(WindowRange),
    /// Always map to the 2000s: 00–99 → 2000–2099.
    Always2000s,
    /// Return the two-digit value literally (e.g. 24 stays as 24).
    Literal,
}

impl Default for TwoDigitYearExpansion {
    fn default() -> Self {
        TwoDigitYearExpansion::SlidingWindow(WindowRange::default())
    }
}

/// Defines how two-digit years are mapped into two non-overlapping,
/// contiguous century ranges.
///
/// For example, the default maps `00–49 → 2000–2049` and `50–99 → 1950–1999`.
///
/// # Validation
///
/// A valid `WindowRange` must satisfy:
/// - Neither range is empty (`min < max`).
/// - The two ranges do not overlap.
/// - Together they cover a contiguous span of exactly 100 years (no gaps).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowRange {
    /// The range that the *lower* two-digit values (e.g. 00–49) map into.
    pub lower_range: Range,
    /// The range that the *upper* two-digit values (e.g. 50–99) map into.
    pub upper_range: Range,
}

impl WindowRange {
    /// Create a new `WindowRange`, returning `Err` if the ranges are invalid.
    ///
    /// Validation rules:
    /// - Both ranges must be non-empty (`min < max`).
    /// - The ranges must not overlap.
    /// - The ranges must be contiguous (no gap between them) and together
    ///   span exactly 100 years.
    pub fn new(lower_range: Range, upper_range: Range) -> Result<Self, WindowRangeError> {
        // Each range must be non-empty.
        if lower_range.min >= lower_range.max {
            return Err(WindowRangeError::EmptyRange {
                label: "lower_range",
                min: lower_range.min,
                max: lower_range.max,
            });
        }
        if upper_range.min >= upper_range.max {
            return Err(WindowRangeError::EmptyRange {
                label: "upper_range",
                min: upper_range.min,
                max: upper_range.max,
            });
        }

        // The two ranges must not overlap.
        // Because Range is non-inclusive on max, overlap means one range's min
        // is strictly less than the other's max AND vice-versa.
        let overlaps = lower_range.min < upper_range.max && upper_range.min < lower_range.max;
        if overlaps {
            return Err(WindowRangeError::Overlapping);
        }

        // Together they must span exactly 100 years with no gap.
        let lower_span = lower_range.max - lower_range.min;
        let upper_span = upper_range.max - upper_range.min;
        let total_span = lower_span + upper_span;
        if total_span != 100 {
            return Err(WindowRangeError::InvalidTotalSpan { total_span });
        }

        // Check contiguity: one range's max must equal the other's min.
        let contiguous = lower_range.max == upper_range.min || upper_range.max == lower_range.min;
        if !contiguous {
            return Err(WindowRangeError::Gap);
        }

        Ok(WindowRange {
            lower_range,
            upper_range,
        })
    }
}

impl Default for WindowRange {
    /// The standard sliding-window default: `00–49 → 2000–2049`, `50–99 → 1950–1999`.
    fn default() -> Self {
        WindowRange {
            lower_range: Range {
                min: 2000,
                max: 2050,
            },
            upper_range: Range {
                min: 1950,
                max: 2000,
            },
        }
    }
}

/// Errors returned by [`WindowRange::new`] when validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowRangeError {
    /// One of the ranges is empty (min >= max).
    EmptyRange {
        label: &'static str,
        min: i32,
        max: i32,
    },
    /// The two ranges overlap.
    Overlapping,
    /// The combined span of both ranges is not exactly 100 years.
    InvalidTotalSpan { total_span: i32 },
    /// There is a gap between the two ranges (they are not contiguous).
    Gap,
}

/// A half-open range `[min, max)` representing a span of years.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    /// Start of the range (inclusive).
    pub min: i32,
    /// End of the range (exclusive).
    pub max: i32,
}

/// Configuration for year extraction.
#[derive(Debug, Clone)]
pub struct YearConfig {
    /// Minimum valid year value (inclusive). Default: `0`.
    pub min: i32,
    /// Maximum valid year value (inclusive). Default: `3000`.
    pub max: i32,
    /// Whether a year component is expected in the input.
    pub expected: IsExpected,
    /// Default year value to use when the year is not found, if any.
    pub default: Option<i32>,
    /// Strategy for expanding two-digit years. Default: [`TwoDigitYearExpansion::SlidingWindow`].
    pub two_digit_expansion: TwoDigitYearExpansion,
}

impl Default for YearConfig {
    fn default() -> Self {
        YearConfig {
            min: 0,
            max: 3000,
            expected: IsExpected::Maybe,
            default: None,
            two_digit_expansion: TwoDigitYearExpansion::default(),
        }
    }
}

/// A single date component: day, month, or year.
///
/// Used within [`ComponentOrder`] to describe the positional ordering of
/// components in structured (numeric) date input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateComponent {
    /// The day-of-month value (1–31).
    Day,
    /// The month value (1–12).
    Month,
    /// The year value.
    Year,
}

/// The expected ordering of date components in positional (numeric) input.
///
/// For example, `01/06/24` is ambiguous — a `ComponentOrder` of
/// `[Day, Month, Year]` interprets it as 1 June 2024, while
/// `[Month, Day, Year]` gives 6 January 2024.
///
/// For unambiguous inputs (e.g. `31/06/24`) the correct interpretation
/// can always be determined regardless of this setting.
///
/// All three components must be present and each must appear exactly once.
/// Construct with [`ComponentOrder::new`] to enforce this invariant, or use
/// [`ComponentOrder::default`] for the standard Day/Month/Year order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentOrder {
    /// The component expected in the first position.
    pub first: DateComponent,
    /// The component expected in the second position.
    pub second: DateComponent,
    /// The component expected in the third position.
    pub third: DateComponent,
}

/// Errors returned by [`ComponentOrder::new`] when validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentOrderError {
    /// The same component appears more than once in the order.
    DuplicateComponent(DateComponent),
}

impl ComponentOrder {
    /// Create a new `ComponentOrder`, returning `Err` if any component is
    /// duplicated (which also implies another is missing).
    pub fn new(
        first: DateComponent,
        second: DateComponent,
        third: DateComponent,
    ) -> Result<Self, ComponentOrderError> {
        if first == second {
            return Err(ComponentOrderError::DuplicateComponent(first));
        }
        if first == third {
            return Err(ComponentOrderError::DuplicateComponent(first));
        }
        if second == third {
            return Err(ComponentOrderError::DuplicateComponent(second));
        }
        Ok(ComponentOrder {
            first,
            second,
            third,
        })
    }
}

impl Default for ComponentOrder {
    /// The default order is Day → Month → Year (e.g. `DD/MM/YYYY`).
    fn default() -> Self {
        ComponentOrder {
            first: DateComponent::Day,
            second: DateComponent::Month,
            third: DateComponent::Year,
        }
    }
}

/// Top-level configuration for the extractor.
///
/// The extractor always tries all standard separators (`/`, `-`, `.`, `,`,
/// `\`, and whitespace) automatically — no separator needs to be specified.
/// Use [`Config::no_separator`] to enable parsing of fully concatenated
/// date strings (e.g. `"25122024"`), and [`Config::extra_separators`] to
/// add custom separator strings (e.g. `"||"`, `" - "`).
///
/// Construct via [`Config::default()`] and override only the fields you need,
/// or build a fully custom config by setting each field explicitly.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Configuration for day extraction.
    pub day: DayConfig,
    /// Configuration for month extraction.
    pub month: MonthConfig,
    /// Configuration for year extraction.
    pub year: YearConfig,
    /// The expected ordering of date components for positional (numeric) inputs.
    /// Default: Day → Month → Year. See [`ComponentOrder`].
    pub component_order: ComponentOrder,
    /// When `true`, the extractor also attempts to parse fully concatenated
    /// date strings with no separator (e.g. `"25122024"`). Default: `false`.
    pub no_separator: bool,
    /// Additional custom separator strings to try alongside the standard set.
    /// Default: empty. Example: `vec!["||".to_string(), " - ".to_string()]`.
    pub extra_separators: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input type
// ---------------------------------------------------------------------------

/// Input to the partial date extractor.
#[derive(Debug, Clone)]
pub struct Input {
    /// The raw text from which a date should be extracted.
    pub utterance: String,
    /// Per-call config override. Falls back to the library default when `None`.
    pub config: Option<Config>,
}

/// The name of a calendar month, as extracted from natural language input.
///
/// ## Conversions
///
/// `MonthName` can be constructed from either a string or a number:
///
/// ```
/// use partial_date::models::{MonthName, MonthNameError};
/// use std::convert::TryFrom;
///
/// // From a name string (full, abbreviated, or unambiguous prefix)
/// assert_eq!(MonthName::try_from("October"), Ok(MonthName::October));
/// assert_eq!(MonthName::try_from("oct"),     Ok(MonthName::October));
/// assert_eq!(MonthName::try_from("Octo"),    Ok(MonthName::October));
///
/// // From a numeric string
/// assert_eq!(MonthName::try_from("10"), Ok(MonthName::October));
///
/// // From a u8
/// assert_eq!(MonthName::try_from(10_u8), Ok(MonthName::October));
///
/// // Errors
/// assert_eq!(MonthName::try_from(0_u8),  Err(MonthNameError::NumberOutOfRange(0)));
/// assert_eq!(MonthName::try_from(13_u8), Err(MonthNameError::NumberOutOfRange(13)));
/// assert_eq!(MonthName::try_from("Xyz"), Err(MonthNameError::UnrecognisedName));
/// assert_eq!(MonthName::try_from("5x"),  Err(MonthNameError::NotAMonth));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl MonthName {
    /// Return the calendar number of this month (1 = January … 12 = December).
    ///
    /// ```
    /// use partial_date::models::MonthName;
    /// assert_eq!(MonthName::January.number(), 1);
    /// assert_eq!(MonthName::December.number(), 12);
    /// ```
    pub fn number(self) -> u8 {
        match self {
            MonthName::January => 1,
            MonthName::February => 2,
            MonthName::March => 3,
            MonthName::April => 4,
            MonthName::May => 5,
            MonthName::June => 6,
            MonthName::July => 7,
            MonthName::August => 8,
            MonthName::September => 9,
            MonthName::October => 10,
            MonthName::November => 11,
            MonthName::December => 12,
        }
    }
}

/// Errors returned when a [`MonthName`] conversion fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonthNameError {
    /// The input string was alphabetic but did not match any known month name,
    /// abbreviation, or unambiguous prefix.
    UnrecognisedName,
    /// The input was a valid integer but outside the range 1–12.
    NumberOutOfRange(u8),
    /// The input was neither a pure alphabetic string nor a pure integer
    /// (e.g. `"5x"` or `"jan2"`).
    NotAMonth,
}

/// Convert a month number (`1` = January … `12` = December) into a
/// [`MonthName`].
///
/// Returns [`MonthNameError::NumberOutOfRange`] for any value outside 1–12.
impl TryFrom<u8> for MonthName {
    type Error = MonthNameError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(MonthName::January),
            2 => Ok(MonthName::February),
            3 => Ok(MonthName::March),
            4 => Ok(MonthName::April),
            5 => Ok(MonthName::May),
            6 => Ok(MonthName::June),
            7 => Ok(MonthName::July),
            8 => Ok(MonthName::August),
            9 => Ok(MonthName::September),
            10 => Ok(MonthName::October),
            11 => Ok(MonthName::November),
            12 => Ok(MonthName::December),
            _ => Err(MonthNameError::NumberOutOfRange(n)),
        }
    }
}

/// Convert a string into a [`MonthName`].
///
/// Three strategies are tried in order:
///
/// 1. **Alphabetic match** — if every character is ASCII alphabetic (after
///    stripping a trailing `.`), the lowercased string is compared against all
///    full names, standard 3-letter abbreviations, and unambiguous longer
///    prefixes.
///
/// 2. **Fuzzy match** — if no exact or prefix match was found, the
///    Levenshtein ratio is computed against every full month name.  The
///    closest match is accepted when its ratio is ≥ 0.6 and it is
///    unambiguously the best candidate (no tie).  Returns
///    [`MonthNameError::UnrecognisedName`] when no candidate passes.
///
/// 3. **Numeric match** — if every character is an ASCII digit, the value is
///    parsed as a `u8` and forwarded to [`TryFrom<u8>`].  Returns
///    [`MonthNameError::NumberOutOfRange`] when the number is outside 1–12.
///
/// If the string is neither purely alphabetic nor purely numeric (e.g.
/// `"jan2"` or `"5x"`), [`MonthNameError::NotAMonth`] is returned.
impl TryFrom<&str> for MonthName {
    type Error = MonthNameError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // Strip a trailing dot before classification (handles "Jan.", "Feb.").
        let s = s.strip_suffix('.').unwrap_or(s);

        if s.is_empty() {
            return Err(MonthNameError::NotAMonth);
        }

        if s.chars().all(|c| c.is_ascii_alphabetic()) {
            // --- Alphabetic path ---
            let lower = s.to_ascii_lowercase();
            match_month_name_str(lower.as_str())
        } else if s.chars().all(|c| c.is_ascii_digit()) {
            // --- Numeric path ---
            // A leading-zero number like "06" parses to 6, which is valid.
            // Values > 255 would overflow u8::MAX; treat them as out-of-range.
            let n: u8 = s.parse().map_err(|_| MonthNameError::NumberOutOfRange(0))?;
            MonthName::try_from(n)
        } else {
            Err(MonthNameError::NotAMonth)
        }
    }
}

/// All twelve full month names paired with their [`MonthName`] variant,
/// used for both prefix and fuzzy matching.
const FULL_MONTH_NAMES: &[(&str, MonthName)] = &[
    ("january", MonthName::January),
    ("february", MonthName::February),
    ("march", MonthName::March),
    ("april", MonthName::April),
    ("may", MonthName::May),
    ("june", MonthName::June),
    ("july", MonthName::July),
    ("august", MonthName::August),
    ("september", MonthName::September),
    ("october", MonthName::October),
    ("november", MonthName::November),
    ("december", MonthName::December),
];

/// Minimum Levenshtein ratio required for a fuzzy match to be accepted.
///
/// A ratio of 0.6 means at most 2 edits in a 5-character word, or 1 edit in
/// a 3-character word.  Empirically this passes all known real-world
/// misspellings while rejecting clearly unrelated words like `"Foo"` or
/// `"Friday"`.
const FUZZY_MATCH_THRESHOLD: f32 = 0.6;

/// Match an already-lowercased, purely-alphabetic string against all known
/// month names, abbreviations, and unambiguous prefixes, falling back to
/// fuzzy (Levenshtein ratio) matching when no exact or prefix match is found.
fn match_month_name_str(lower: &str) -> Result<MonthName, MonthNameError> {
    // --- 1. Exact match: full names and standard 3-letter abbreviations ---
    let exact = match lower {
        "january" | "jan" => Some(MonthName::January),
        "february" | "feb" => Some(MonthName::February),
        "march" | "mar" => Some(MonthName::March),
        "april" | "apr" => Some(MonthName::April),
        "may" => Some(MonthName::May),
        "june" | "jun" => Some(MonthName::June),
        "july" | "jul" => Some(MonthName::July),
        "august" | "aug" => Some(MonthName::August),
        "september" | "sep" => Some(MonthName::September),
        "october" | "oct" => Some(MonthName::October),
        "november" | "nov" => Some(MonthName::November),
        "december" | "dec" => Some(MonthName::December),
        _ => None,
    };

    if let Some(month) = exact {
        return Ok(month);
    }

    // --- 2. Unambiguous prefix match (≥ 4 characters) ---
    if lower.len() >= 4 {
        let mut found: Option<MonthName> = None;
        for (full_name, variant) in FULL_MONTH_NAMES {
            if full_name.starts_with(lower) {
                if found.is_some() {
                    // More than one month starts with this prefix — ambiguous;
                    // fall through to fuzzy matching below.
                    found = None;
                    break;
                }
                found = Some(*variant);
            }
        }
        if let Some(month) = found {
            return Ok(month);
        }
    }

    // --- 3. Fuzzy match via Levenshtein ratio ---
    fuzzy_match_month(lower)
}

/// Find the best-matching month name for `lower` using Levenshtein ratio.
///
/// Returns the matched [`MonthName`] if exactly one candidate scores above
/// [`FUZZY_MATCH_THRESHOLD`] and no other candidate ties it.  Returns
/// [`MonthNameError::UnrecognisedName`] otherwise.
fn fuzzy_match_month(lower: &str) -> Result<MonthName, MonthNameError> {
    use crate::levenshtein::levenshtein_ratio;

    let mut best_ratio: f32 = 0.0;
    let mut best_month: Option<MonthName> = None;
    let mut is_tied = false;

    for (full_name, variant) in FULL_MONTH_NAMES {
        let ratio = levenshtein_ratio(lower, full_name);
        if ratio > best_ratio {
            best_ratio = ratio;
            best_month = Some(*variant);
            is_tied = false;
        } else if (ratio - best_ratio).abs() < f32::EPSILON {
            // Two candidates have the same ratio — ambiguous.
            is_tied = true;
        }
    }

    if best_ratio >= FUZZY_MATCH_THRESHOLD && !is_tied {
        best_month.ok_or(MonthNameError::UnrecognisedName)
    } else {
        Err(MonthNameError::UnrecognisedName)
    }
}

// ---------------------------------------------------------------------------
// Tokenisation types
// ---------------------------------------------------------------------------

/// A single meaningful chunk produced by [`crate::extract::tokenise`].
///
/// The tokeniser strips separator characters and noise words, leaving only
/// tokens that *could* contribute to a date component. At most three tokens
/// are returned (one per date component: day, month, year).
///
/// Each variant stores the already-parsed value rather than the raw source
/// text, so consumers can use the token directly without re-parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A parsed integer, e.g. `19`, `2014`, `6`.
    ///
    /// Uses `i16` because the full year range required by the spec (0–3000)
    /// fits within `i16::MAX` (32,767), and day/month values are far smaller.
    Numeric(i16),
    /// The numeric day extracted from an ordinal like `"19th"` or `"1st"`,
    /// with the suffix already stripped.
    OrdinalDay(u8),
    /// A resolved [`MonthName`] variant, matched from a full name,
    /// abbreviation, unambiguous prefix, or fuzzy misspelling.
    MonthName(MonthName),
}
