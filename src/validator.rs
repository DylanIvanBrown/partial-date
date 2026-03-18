//! Validation and default application for extracted date components.

use crate::interpreter::{RawDay, RawMonth, RawYear};
use crate::models::{DayConfig, Extracted, MonthConfig, YearConfig};

/// Validate a raw day value against the day config bounds.
pub fn validate_day(raw: RawDay, config: &DayConfig) -> Option<u8> {
    raw.and_then(|(v, _dc)| {
        if v >= config.min && v <= config.max {
            Some(v)
        } else {
            None
        }
    })
}

/// Validate a raw month value against the month config bounds.
pub fn validate_month(raw: RawMonth, config: &MonthConfig) -> RawMonth {
    raw.and_then(|(n, name)| {
        if n >= config.min && n <= config.max {
            Some((n, name))
        } else {
            None
        }
    })
}

/// Validate a raw year value against the year config bounds.
pub fn validate_year(raw: RawYear, config: &YearConfig) -> RawYear {
    raw.and_then(|y| {
        if y >= config.min && y <= config.max {
            Some(y)
        } else {
            None
        }
    })
}

/// Apply default value logic: convert `Option<T>` to `Extracted<T>`.
///
/// If a value was found, return `Extracted::Found(v)`.
/// If no value but a default is configured, return `Extracted::Defaulted(default)`.
/// Otherwise, return `Extracted::NotFound`.
pub fn apply_default<T: Copy>(val: Option<T>, default: Option<T>) -> Extracted<T> {
    match val {
        Some(v) => Extracted::Found(v),
        None => match default {
            Some(d) => Extracted::Defaulted(d),
            None => Extracted::NotFound,
        },
    }
}
