//! Core types for the partial-date library.
//!
//! This module contains all the structs and enums that describe inputs,
//! configuration, and extraction results.

// ---------------------------------------------------------------------------
// Result types
// ---------------------------------------------------------------------------

/// The outcome of attempting to extract a single date component (day, month, or year).
pub enum Extracted<T> {
    /// The value was found directly in the input.
    Found(T),
    /// No value could be found and no default was configured.
    NotFound,
    /// The value was not found in the input but a default was applied.
    Defaulted(T),
}

/// A fully-resolved (possibly partial) date returned by the extractor.
pub struct PartialDate {
    pub day: Day,
    pub month: Month,
    pub year: Year,
}

/// Extracted day component (1–31).
pub struct Day {
    pub value: Extracted<u8>,
}

/// Extracted month component (1–12).
pub struct Month {
    pub value: Extracted<u8>,
}

/// Extracted year component.
///
/// Uses `i32` to accommodate the full range required by the spec (0–3000) and
/// to leave room for historical (negative / BC) years if needed in future.
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
}

impl Default for YearConfig {
    fn default() -> Self {
        YearConfig {
            min: 0,
            max: 3000,
            expected: IsExpected::Maybe,
            default: None,
        }
    }
}

/// The expected ordering of date components when the input is ambiguous.
///
/// For example, `01/06/24` is ambiguous — `Format::DDMMYY` interprets it as
/// 1 June 2024, while `Format::MMDDYY` gives 6 January 2024.
///
/// For unambiguous inputs (e.g. `31/06/24`) the correct interpretation can
/// always be determined regardless of this setting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    DDMMYY,
    DDMMYYYY,
    MMDDYY,
    MMDDYYYY,
    MMYYDD,
    MMYYYYDD,
    YYMMDD,
    YYDDMM,
    YYYYMMDD,
    YYYYDDMM,
    /// A custom format string. Use `D`, `M`, and `Y` to denote day, month,
    /// and year positions respectively (e.g. `"D-M-Y"`).
    Other(String),
}

impl Default for Format {
    fn default() -> Self {
        Format::DDMMYYYY
    }
}

/// A field separator that may appear between date components.
///
/// The extractor will attempt all separator variants automatically; this field
/// is used to weight the primary (expected) separator more heavily.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Separator {
    /// `-`
    Dash,
    /// ASCII whitespace
    Space,
    /// `/`
    ForwardSlash,
    /// `\`
    BackSlash,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// No separator, e.g. `011224` or `19941231`.
    NoSeparator,
    /// A custom separator string.
    Other(String),
}

impl Default for Separator {
    fn default() -> Self {
        Separator::ForwardSlash
    }
}

/// Top-level configuration for the extractor.
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
    /// The expected date component ordering for ambiguous inputs.
    pub primary_format: Format,
    /// The primary separator to expect between date components.
    pub primary_separator: Separator,
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
