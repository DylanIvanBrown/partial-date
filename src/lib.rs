//! # partial-date
//!
//! Deterministic partial date extraction from natural language text.
//!
//! Unlike full-date parsers, this library handles *partial* dates — inputs where only some
//! of day, month, or year are present. Missing components can be left as [`models::Extracted::NotFound`]
//! or filled with a caller-supplied default via [`models::Extracted::Defaulted`].
//!
//! ## Quick start
//!
//! ```rust
//! use partial_date::models::{Input, PartialDate};
//!
//! let input = Input {
//!     utterance: "12 June".to_string(),
//!     config: None,
//! };
//!
//! // let result: PartialDate = partial_date::extract::extract(input);
//! ```

pub mod extract;
pub mod levenshtein;
pub mod models;
