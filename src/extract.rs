//! Extraction functions for partial date parsing.
//!
//! This module will contain the core logic for extracting day, month, and year
//! components from a raw input string.

use crate::models::{Config, Day, Extracted, Input, Month, PartialDate, Year};

/// Extract a partial date from the given input.
///
/// Returns a [`PartialDate`] where each component is either [`crate::models::Extracted::Found`],
/// [`crate::models::Extracted::Defaulted`], or [`crate::models::Extracted::NotFound`]
/// depending on what could be determined from the utterance and config.
pub fn extract(input: Input) -> PartialDate {
    let _config = input.config.unwrap_or_default();
    let _extractor = PartialDateExtractor::new(_config);
    // TODO: use _extractor.component_order to drive positional parsing

    // Temp partial date to return
    PartialDate {
        day: Day {
            value: Extracted::NotFound,
        },
        month: Month {
            number: Extracted::NotFound,
            name: Extracted::NotFound,
        },
        year: Year {
            value: Extracted::NotFound,
        },
    }
}

// Fields will be used once extraction logic is implemented.
#[allow(dead_code)]
struct PartialDateExtractor {
    config: Config,
}

impl PartialDateExtractor {
    fn new(config: Config) -> Self {
        PartialDateExtractor { config }
    }
}
