//! Extraction functions for partial date parsing.
//!
//! This module will contain the core logic for extracting day, month, and year
//! components from a raw input string.

use crate::models::{Input, PartialDate};

/// Extract a partial date from the given input.
///
/// Returns a [`PartialDate`] where each component is either [`crate::models::Extracted::Found`],
/// [`crate::models::Extracted::Defaulted`], or [`crate::models::Extracted::NotFound`]
/// depending on what could be determined from the utterance and config.
pub fn extract(_input: Input) -> PartialDate {
    todo!("implement extraction logic")
}
