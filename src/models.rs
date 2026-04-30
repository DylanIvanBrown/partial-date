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

//TODO: Refactor these to be on the Day, Month and Year structs rather than the
//configs? Only issue might be the min and max, but I think we can instead
//attach the configs to the structs to assist in that? Perhaps that makes the
//Structs too messy for returning in the PartialDate and we should have another
//intermediate struct like DayCandidate or something like that that we can map
//to a Day using From/Into when we are determining the PartialDate output at the
//end. That way we don't expose the config in the return value to the user of
//the library
impl DayConfig {
    /// Return `Some(value as u8)` when `value` is a plausible day for this
    /// config, or `None` when it is not.
    ///
    /// A value is a plausible day when:
    /// - `digit_count` is not 4 (four-digit numbers cannot be days).
    /// - The value is within the universal day range 1–31.
    /// - The value falls within the caller-configured `min`/`max` bounds.
    pub fn try_as_day_candidate(&self, value: i16, digit_count: u8) -> Option<u8> {
        if digit_count == 4 {
            return None;
        }
        let as_u8 = u8::try_from(value).ok()?;
        if (1..=31).contains(&value) && (self.min..=self.max).contains(&as_u8) {
            Some(as_u8)
        } else {
            None
        }
    }
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

impl DayConfig {
    /// Set the valid day range.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`. Use [`DayConfig::try_with_range`] when the
    /// values come from dynamic input and you need to handle the error.
    ///
    /// ```
    /// use partial_date::models::DayConfig;
    ///
    /// let config = DayConfig::default().with_range(1, 28);
    /// ```
    pub fn with_range(self, min: u8, max: u8) -> Self {
        assert!(
            min <= max,
            "DayConfig::with_range min ({min}) must not exceed max ({max})"
        );
        DayConfig { min, max, ..self }
    }

    /// Set the valid day range, returning `Err` if `min > max`.
    ///
    /// Use this when the range values come from dynamic input. For
    /// known-valid static values, prefer [`DayConfig::with_range`].
    pub fn try_with_range(self, min: u8, max: u8) -> Result<Self, ConfigRangeError> {
        if min > max {
            return Err(ConfigRangeError::MinExceedsMax {
                min: min as i32,
                max: max as i32,
            });
        }
        Ok(DayConfig { min, max, ..self })
    }

    /// Set whether a day component is expected in the input.
    ///
    /// ```
    /// use partial_date::models::{DayConfig, IsExpected};
    ///
    /// let config = DayConfig::default().with_expected(IsExpected::Yes);
    /// ```
    pub fn with_expected(self, expected: IsExpected) -> Self {
        DayConfig { expected, ..self }
    }

    /// Set the default day value to use when no day is found in the input.
    ///
    /// ```
    /// use partial_date::models::DayConfig;
    ///
    /// let config = DayConfig::default().with_default(1);
    /// ```
    pub fn with_default(self, default: u8) -> Self {
        DayConfig {
            default: Some(default),
            ..self
        }
    }
}

/// Error returned by [`DayConfig::try_with_range`], [`MonthConfig::try_with_range`],
/// and [`YearConfig::try_with_range`] when the provided range is invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigRangeError {
    /// The minimum value exceeds the maximum value.
    MinExceedsMax { min: i32, max: i32 },
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

impl MonthConfig {
    /// Return `Some(value as u8)` when `value` is a plausible month for this
    /// config, or `None` when it is not.
    ///
    /// A value is a plausible month when:
    /// - `digit_count` is not 4 (four-digit numbers cannot be months).
    /// - The value is within the universal month range 1–12.
    /// - The value falls within the caller-configured `min`/`max` bounds.
    pub fn try_as_month_candidate(&self, value: i16, digit_count: u8) -> Option<u8> {
        if digit_count == 4 {
            return None;
        }
        let as_u8 = u8::try_from(value).ok()?;
        if (1..=12).contains(&value) && (self.min..=self.max).contains(&as_u8) {
            Some(as_u8)
        } else {
            None
        }
    }
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

impl MonthConfig {
    /// Set the valid month range.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`. Use [`MonthConfig::try_with_range`] when the
    /// values come from dynamic input and you need to handle the error.
    ///
    /// ```
    /// use partial_date::models::MonthConfig;
    ///
    /// let config = MonthConfig::default().with_range(1, 6);
    /// ```
    pub fn with_range(self, min: u8, max: u8) -> Self {
        assert!(
            min <= max,
            "MonthConfig::with_range min ({min}) must not exceed max ({max})"
        );
        MonthConfig { min, max, ..self }
    }

    /// Set the valid month range, returning `Err` if `min > max`.
    ///
    /// Use this when the range values come from dynamic input. For
    /// known-valid static values, prefer [`MonthConfig::with_range`].
    pub fn try_with_range(self, min: u8, max: u8) -> Result<Self, ConfigRangeError> {
        if min > max {
            return Err(ConfigRangeError::MinExceedsMax {
                min: min as i32,
                max: max as i32,
            });
        }
        Ok(MonthConfig { min, max, ..self })
    }

    /// Set whether a month component is expected in the input.
    ///
    /// ```
    /// use partial_date::models::{IsExpected, MonthConfig};
    ///
    /// let config = MonthConfig::default().with_expected(IsExpected::Yes);
    /// ```
    pub fn with_expected(self, expected: IsExpected) -> Self {
        MonthConfig { expected, ..self }
    }

    /// Set the default month value to use when no month is found in the input.
    ///
    /// ```
    /// use partial_date::models::MonthConfig;
    ///
    /// let config = MonthConfig::default().with_default(1);
    /// ```
    pub fn with_default(self, default: u8) -> Self {
        MonthConfig {
            default: Some(default),
            ..self
        }
    }
}

/// The pivot point for a [`TwoDigitYearExpansion::SlidingWindow`].
///
/// A valid pivot is in the range `1–99`. A pivot of `p` means two-digit
/// values `0..(p-1)` map to the upper (more recent) part of the window, and
/// values `p..99` map to the lower (earlier) part.
///
/// # Construction
///
/// Use [`SlidingWindowPivot::new`] for known-valid static values (panics on
/// invalid input) or [`SlidingWindowPivot::try_new`] when the value comes
/// from dynamic input and you need to handle the error:
///
/// ```
/// use partial_date::models::SlidingWindowPivot;
///
/// // Known-valid — panics if the value is invalid.
/// let pivot = SlidingWindowPivot::new(50);
///
/// // Dynamic input — returns Result.
/// let pivot = SlidingWindowPivot::try_new(50).unwrap();
///
/// // Via TryFrom.
/// let pivot: SlidingWindowPivot = 50_u8.try_into().unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlidingWindowPivot(u8);

impl SlidingWindowPivot {
    /// Create a new `SlidingWindowPivot`.
    ///
    /// # Panics
    ///
    /// Panics if `pivot` is `0` or greater than `99`. Use
    /// [`SlidingWindowPivot::try_new`] when the value comes from dynamic input
    /// and you need to handle the error instead of panicking.
    pub fn new(pivot: u8) -> Self {
        assert!(
            pivot > 0 && pivot <= 99,
            "SlidingWindowPivot must be in the range 1–99, got {pivot}"
        );
        SlidingWindowPivot(pivot)
    }

    /// Create a new `SlidingWindowPivot`, returning `Err` if `pivot` is `0`
    /// or greater than `99`.
    ///
    /// Use this when the pivot value comes from dynamic input (e.g. user
    /// configuration, a config file). For known-valid static values, prefer
    /// [`SlidingWindowPivot::new`].
    pub fn try_new(pivot: u8) -> Result<Self, SlidingWindowPivotError> {
        if pivot == 0 || pivot > 99 {
            return Err(SlidingWindowPivotError::InvalidPivot(pivot));
        }
        Ok(SlidingWindowPivot(pivot))
    }
}

impl TryFrom<u8> for SlidingWindowPivot {
    type Error = SlidingWindowPivotError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        SlidingWindowPivot::try_new(value)
    }
}

impl From<SlidingWindowPivot> for u8 {
    fn from(pivot: SlidingWindowPivot) -> u8 {
        pivot.0
    }
}

/// Errors returned by [`SlidingWindowPivot::try_new`] when validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlidingWindowPivotError {
    /// The pivot value must be in the range `1–99`.
    InvalidPivot(u8),
}

/// A year that falls exactly on a century boundary (divisible by 100).
///
/// Used with [`TwoDigitYearExpansion::Always`] to express that all two-digit
/// values should map into a specific century. For example, `Century::new(1800)`
/// means `00 → 1800`, `34 → 1834`, `99 → 1899`.
///
/// # Construction
///
/// Use [`Century::new`] for known-valid static values (panics on invalid
/// input) or [`Century::try_new`] when the value comes from dynamic input
/// and you need to handle the error:
///
/// ```
/// use partial_date::models::Century;
///
/// // Known-valid — panics if the value is not a century boundary.
/// let century = Century::new(1800);
///
/// // Dynamic input — returns Result.
/// let century = Century::try_new(2000).unwrap();
///
/// // Via TryFrom.
/// let century: Century = 2000_i32.try_into().unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Century(i32);

impl Century {
    /// Create a new `Century`.
    ///
    /// # Panics
    ///
    /// Panics if `year` is not divisible by `100`. Use [`Century::try_new`]
    /// when the value comes from dynamic input and you need to handle the
    /// error instead of panicking.
    pub fn new(year: i32) -> Self {
        assert!(
            year % 100 == 0,
            "Century must be divisible by 100, got {year}"
        );
        Century(year)
    }

    /// Create a new `Century`, returning `Err` if `year` is not divisible by
    /// `100`.
    ///
    /// Use this when the year value comes from dynamic input (e.g. user
    /// configuration, a config file). For known-valid static values, prefer
    /// [`Century::new`].
    pub fn try_new(year: i32) -> Result<Self, CenturyError> {
        if year % 100 != 0 {
            return Err(CenturyError::NotACenturyBoundary(year));
        }
        Ok(Century(year))
    }
}

impl TryFrom<i32> for Century {
    type Error = CenturyError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Century::try_new(value)
    }
}

impl From<Century> for i32 {
    fn from(century: Century) -> i32 {
        century.0
    }
}

/// Errors returned by [`Century::try_new`] when validation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CenturyError {
    /// The year value must be divisible by `100` (e.g. `1800`, `2000`).
    NotACenturyBoundary(i32),
}

/// Strategy for expanding two-digit years into four-digit years.
///
/// # Choosing a strategy
///
/// - Use [`SlidingWindow`] when two-digit years could span two adjacent
///   centuries and you want to bias towards a particular era.
/// - Use [`Always`] when all two-digit years belong to the same century
///   without ambiguity (e.g. children's birthdays are all in the 2000s).
/// - Use [`Literal`] when you want the two-digit value kept as-is (e.g.
///   historical records where the year is genuinely in the range 0–99).
///
/// [`SlidingWindow`]: TwoDigitYearExpansion::SlidingWindow
/// [`Always`]: TwoDigitYearExpansion::Always
/// [`Literal`]: TwoDigitYearExpansion::Literal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoDigitYearExpansion {
    /// Splits the 100 possible two-digit values across two adjacent centuries.
    ///
    /// `earliest_year` is the smallest year the window can ever produce — it
    /// is the year that two-digit value `pivot` maps to.  Values
    /// `pivot..=99` map to `earliest_year..=(earliest_year + (99 - pivot))`,
    /// and values `0..(pivot)` map to
    /// `(earliest_year + (100 - pivot))..(earliest_year + 99)`.
    ///
    /// # Example
    ///
    /// ```
    /// use partial_date::models::{SlidingWindowPivot, TwoDigitYearExpansion};
    ///
    /// // 00–49 → 2000–2049, 50–99 → 1950–1999 (the default).
    /// let expansion = TwoDigitYearExpansion::SlidingWindow {
    ///     earliest_year: 1950,
    ///     pivot: SlidingWindowPivot::new(50),
    /// };
    ///
    /// // Industrial Revolution era: 00–49 → 1800–1849, 50–99 → 1750–1799.
    /// let expansion = TwoDigitYearExpansion::SlidingWindow {
    ///     earliest_year: 1750,
    ///     pivot: SlidingWindowPivot::new(50),
    /// };
    /// ```
    SlidingWindow {
        /// The smallest year this window can produce (the year `pivot` maps
        /// to).  Must be chosen so that the full window
        /// `[earliest_year, earliest_year + 99]` covers the values you
        /// intend to accept.  Use [`YearConfig::min`] and [`YearConfig::max`]
        /// to reject any expanded years that fall outside your valid range.
        earliest_year: i32,
        /// The two-digit value at which the window wraps from the lower
        /// (earlier) century to the upper (more recent) century.
        pivot: SlidingWindowPivot,
    },
    /// Maps all two-digit values into a single century.
    ///
    /// `00` maps to the century start, `99` maps to `century + 99`.
    ///
    /// # Example
    ///
    /// ```
    /// use partial_date::models::{Century, TwoDigitYearExpansion};
    ///
    /// // All two-digit years are in the 2000s: 00 → 2000, 34 → 2034.
    /// let expansion = TwoDigitYearExpansion::Always(Century::new(2000));
    ///
    /// // All two-digit years are in the 1800s: 00 → 1800, 34 → 1834.
    /// let expansion = TwoDigitYearExpansion::Always(Century::new(1800));
    /// ```
    Always(Century),
    /// Return the two-digit value literally (e.g. `24` stays as `24`).
    ///
    /// Useful when processing historical records where the year genuinely
    /// falls in the range `0–99`, or when you want to apply your own
    /// post-processing.
    Literal,
}

impl Default for TwoDigitYearExpansion {
    /// The standard modern sliding window: `00–49 → 2000–2049`, `50–99 → 1950–1999`.
    fn default() -> Self {
        TwoDigitYearExpansion::SlidingWindow {
            earliest_year: 1950,
            pivot: SlidingWindowPivot(50),
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
    /// Strategy for expanding two-digit years. Default: [`TwoDigitYearExpansion::SlidingWindow`].
    pub two_digit_expansion: TwoDigitYearExpansion,
    /// When `true`, a single-digit token (`1`–`9`) is treated as a two-digit
    /// year by prepending a zero — `5` becomes `05` — and then expanded
    /// according to [`YearConfig::two_digit_expansion`].
    ///
    /// This option only applies when the other date components (day and month)
    /// have already been filled by unambiguous tokens, so the interpreter can
    /// confirm that the single digit is genuinely intended as a year.
    ///
    /// Default: `false`.  Enable when processing inputs like `"1 January 5"`
    /// where `5` means year AD 5 (literal) or year 2005 (sliding window).
    pub single_digit_year_expansion: bool,
}

impl YearConfig {
    /// Return the expanded year value when `value` (with `digit_count` original
    /// digits) is a plausible year for this config, or `None` when it is not.
    ///
    /// - 4-digit values are used as-is.
    /// - 3-digit values (100–999) are treated as literal years.
    /// - 2-digit values are expanded according to [`TwoDigitYearExpansion`].
    /// - 1-digit values are accepted only when
    ///   [`YearConfig::single_digit_year_expansion`] is `true`, in which case
    ///   `value` is treated as `0value` (e.g. `5` → `05`) and then expanded
    ///   using the same two-digit expansion strategy.
    /// - All other digit counts return `None`.
    ///
    /// The expanded value must also fall within the configured `min`/`max`
    /// bounds.
    pub fn try_as_year_candidate(&self, value: i16, digit_count: u8) -> Option<i32> {
        // Normalise single-digit values to their two-digit equivalent when the
        // option is enabled, then fall through to the two-digit expansion path.
        let (effective_value, effective_digit_count) =
            if digit_count == 1 && self.single_digit_year_expansion {
                // Prepend a zero: "5" → "05".  The digit count is now 2.
                (value, 2u8)
            } else {
                (value, digit_count)
            };

        let expanded = match effective_digit_count {
            4 => effective_value as i32,
            // 3-digit values (100–999) are treated as literal years: year 100,
            // year 999, etc.  This covers word-number inputs like "nine hundred
            // ninety-nine" which replace to the 3-digit numeral 999.
            3 => effective_value as i32,
            2 => {
                let raw = effective_value as i32;
                match &self.two_digit_expansion {
                    TwoDigitYearExpansion::Literal => raw,
                    TwoDigitYearExpansion::Always(century) => i32::from(*century) + raw,
                    TwoDigitYearExpansion::SlidingWindow {
                        earliest_year,
                        pivot,
                    } => {
                        let pivot = u8::from(*pivot) as i32;
                        if raw < pivot {
                            // Upper (more recent) half: 0..(pivot-1)
                            earliest_year + (100 - pivot) + raw
                        } else {
                            // Lower (earlier) half: pivot..99
                            earliest_year + (raw - pivot)
                        }
                    }
                }
            }
            _ => return None,
        };
        if expanded >= self.min && expanded <= self.max {
            Some(expanded)
        } else {
            None
        }
    }
}

impl Default for YearConfig {
    fn default() -> Self {
        YearConfig {
            min: 0,
            max: 3000,
            expected: IsExpected::Maybe,
            default: None,
            two_digit_expansion: TwoDigitYearExpansion::default(),
            single_digit_year_expansion: false,
        }
    }
}

impl YearConfig {
    /// Set the valid year range.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`. Use [`YearConfig::try_with_range`] when the
    /// values come from dynamic input and you need to handle the error.
    ///
    /// ```
    /// use partial_date::models::YearConfig;
    ///
    /// let config = YearConfig::default().with_range(1760, 1840);
    /// ```
    pub fn with_range(self, min: i32, max: i32) -> Self {
        assert!(
            min <= max,
            "YearConfig::with_range min ({min}) must not exceed max ({max})"
        );
        YearConfig { min, max, ..self }
    }

    /// Set the valid year range, returning `Err` if `min > max`.
    ///
    /// Use this when the range values come from dynamic input. For
    /// known-valid static values, prefer [`YearConfig::with_range`].
    pub fn try_with_range(self, min: i32, max: i32) -> Result<Self, ConfigRangeError> {
        if min > max {
            return Err(ConfigRangeError::MinExceedsMax { min, max });
        }
        Ok(YearConfig { min, max, ..self })
    }

    /// Set whether a year component is expected in the input.
    ///
    /// ```
    /// use partial_date::models::{IsExpected, YearConfig};
    ///
    /// let config = YearConfig::default().with_expected(IsExpected::Yes);
    /// ```
    pub fn with_expected(self, expected: IsExpected) -> Self {
        YearConfig { expected, ..self }
    }

    /// Set the default year value to use when no year is found in the input.
    ///
    /// ```
    /// use partial_date::models::YearConfig;
    ///
    /// let config = YearConfig::default().with_default(2025);
    /// ```
    pub fn with_default(self, default: i32) -> Self {
        YearConfig {
            default: Some(default),
            ..self
        }
    }

    /// Set the two-digit year expansion strategy.
    ///
    /// ```
    /// use partial_date::models::{
    ///     Century, SlidingWindowPivot, TwoDigitYearExpansion, YearConfig,
    /// };
    ///
    /// // All two-digit years map to the 2000s.
    /// let config = YearConfig::default()
    ///     .with_two_digit_expansion(TwoDigitYearExpansion::Always(Century::new(2000)));
    ///
    /// // Industrial Revolution era window.
    /// let config = YearConfig::default()
    ///     .with_range(1760, 1840)
    ///     .with_two_digit_expansion(TwoDigitYearExpansion::SlidingWindow {
    ///         earliest_year: 1750,
    ///         pivot: SlidingWindowPivot::new(50),
    ///     });
    /// ```
    pub fn with_two_digit_expansion(self, two_digit_expansion: TwoDigitYearExpansion) -> Self {
        YearConfig {
            two_digit_expansion,
            ..self
        }
    }

    /// Set whether single-digit tokens are expanded as two-digit years.
    ///
    /// When `true`, a single-digit token (e.g. `5`) is treated as `05` and
    /// then expanded according to the configured
    /// [`YearConfig::two_digit_expansion`] strategy.
    ///
    /// ```
    /// use partial_date::models::YearConfig;
    ///
    /// let config = YearConfig::default().with_single_digit_expansion(true);
    /// ```
    pub fn with_single_digit_expansion(self, enabled: bool) -> Self {
        YearConfig {
            single_digit_year_expansion: enabled,
            ..self
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
#[derive(Debug, Clone)]
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
    /// When `true`, the tokeniser substitutes the letter `O` (upper or lower
    /// case) for the digit `0` inside tokens that consist entirely of digits
    /// and the letter O — for example `"2O24"` is treated as `"2024"`.
    ///
    /// This handles OCR and keyboard-entry errors where the letter O is typed
    /// in place of zero. The substitution is applied only to tokens that would
    /// otherwise be entirely numeric-with-O; it is never applied when the O
    /// appears as part of a longer alphabetic run (e.g. `"7october"` — the
    /// `"october"` portion is left as-is and classified as a month name).
    ///
    /// Default: `true`.
    pub letter_o_substitution: bool,
    //TODO: Add a Config section for fuzzy matching options, include letter o substitution in that config
}

impl Default for Config {
    fn default() -> Self {
        Config {
            day: DayConfig::default(),
            month: MonthConfig::default(),
            year: YearConfig::default(),
            component_order: ComponentOrder::default(),
            no_separator: false,
            extra_separators: Vec::new(),
            letter_o_substitution: true,
        }
    }
}

impl Config {
    /// Set the day extraction configuration.
    ///
    /// ```
    /// use partial_date::models::{Config, DayConfig, IsExpected};
    ///
    /// let config = Config::default()
    ///     .with_day(DayConfig::default().with_range(1, 28).with_expected(IsExpected::Yes));
    /// ```
    pub fn with_day(self, day: DayConfig) -> Self {
        Config { day, ..self }
    }

    /// Set the month extraction configuration.
    ///
    /// ```
    /// use partial_date::models::{Config, IsExpected, MonthConfig};
    ///
    /// let config = Config::default()
    ///     .with_month(MonthConfig::default().with_expected(IsExpected::Yes));
    /// ```
    pub fn with_month(self, month: MonthConfig) -> Self {
        Config { month, ..self }
    }

    /// Set the year extraction configuration.
    ///
    /// ```
    /// use partial_date::models::{Config, IsExpected, YearConfig};
    ///
    /// let config = Config::default()
    ///     .with_year(YearConfig::default().with_range(1760, 1840).with_expected(IsExpected::Yes));
    /// ```
    pub fn with_year(self, year: YearConfig) -> Self {
        Config { year, ..self }
    }

    /// Set the expected ordering of date components for positional inputs.
    ///
    /// ```
    /// use partial_date::models::{ComponentOrder, Config, DateComponent};
    ///
    /// let config = Config::default().with_component_order(
    ///     ComponentOrder::new(
    ///         DateComponent::Month,
    ///         DateComponent::Day,
    ///         DateComponent::Year,
    ///     )
    ///     .unwrap(),
    /// );
    /// ```
    pub fn with_component_order(self, component_order: ComponentOrder) -> Self {
        Config {
            component_order,
            ..self
        }
    }

    /// Enable or disable no-separator parsing (e.g. `"25122024"`).
    ///
    /// ```
    /// use partial_date::models::Config;
    ///
    /// let config = Config::default().with_no_separator(true);
    /// ```
    pub fn with_no_separator(self, no_separator: bool) -> Self {
        Config {
            no_separator,
            ..self
        }
    }

    /// Set additional custom separator strings to try alongside the standard
    /// set (`/`, `-`, `.`, `,`, `\`, and whitespace).
    ///
    /// ```
    /// use partial_date::models::Config;
    ///
    /// let config = Config::default()
    ///     .with_extra_separators(vec!["||".to_string(), " - ".to_string()]);
    /// ```
    pub fn with_extra_separators(self, extra_separators: Vec<String>) -> Self {
        Config {
            extra_separators,
            ..self
        }
    }

    /// Enable or disable substitution of the letter `O` for the digit `0`.
    ///
    /// ```
    /// use partial_date::models::Config;
    ///
    /// let config = Config::default().with_letter_o_substitution(false);
    /// ```
    pub fn with_letter_o_substitution(self, letter_o_substitution: bool) -> Self {
        Config {
            letter_o_substitution,
            ..self
        }
    }
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
    /// A parsed integer together with the number of digits in the original
    /// source string.
    ///
    /// The digit count is required for year disambiguation: `"24"` (2 digits)
    /// must be expanded via [`TwoDigitYearExpansion`], while `"2024"` (4
    /// digits) is used as-is.  Three-digit and five-digit numbers are never
    /// valid date components.
    ///
    /// Uses `i16` for the value because the full year range required by the
    /// spec (0–3000) fits within `i16::MAX` (32,767), and day/month values
    /// are far smaller.
    Numeric(i16, u8),
    /// The numeric day extracted from an ordinal like `"19th"` or `"1st"`,
    /// with the suffix already stripped.
    OrdinalDay(u8),
    /// A resolved [`MonthName`] variant, matched from a full name,
    /// abbreviation, unambiguous prefix, or fuzzy misspelling.
    MonthName(MonthName),
}
