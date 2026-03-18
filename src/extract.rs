//! Public extraction API.

use crate::interpreter;
use crate::models::{Day, Input, Month, PartialDate, Year};
use crate::tokeniser;
use crate::validator;

/// Extract a partial date from the given input.
///
/// Returns a [`PartialDate`] where each component is either
/// [`Extracted::Found`], [`Extracted::Defaulted`], or [`Extracted::NotFound`]
/// depending on what could be determined from the utterance and config.
pub fn extract(input: Input) -> PartialDate {
    let config = input.config.unwrap_or_default();
    let tokens = tokeniser::tokenise(&input.utterance, &config);
    let (day_raw, month_raw, year_raw) = interpreter::interpret_tokens(&tokens, &config);

    let day_val = validator::validate_day(day_raw, &config.day);
    let month_val = validator::validate_month(month_raw, &config.month);
    let year_val = validator::validate_year(year_raw, &config.year);

    PartialDate {
        day: Day {
            value: validator::apply_default(day_val, config.day.default),
        },
        month: Month {
            number: validator::apply_default(month_val.map(|(n, _)| n), config.month.default),
            name: match month_val {
                Some((_, Some(name))) => crate::models::Extracted::Found(name),
                _ => crate::models::Extracted::NotFound,
            },
        },
        year: Year {
            value: validator::apply_default(year_val, config.year.default),
        },
    }
}

/// Public re-export of the tokenise function.
pub use crate::tokeniser::tokenise;
