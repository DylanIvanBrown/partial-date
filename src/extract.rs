//! Extraction functions for partial date parsing.
//!
//! This module contains the core logic for extracting day, month, and year
//! components from a raw input string.

use crate::models::{Config, Day, Extracted, Input, Month, MonthName, PartialDate, Token, Year};

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

/// Split `utterance` on any standard separator or extra separator and classify
/// each resulting chunk as a [`Token`].
///
/// # What counts as a separator
///
/// The standard separator set is: ASCII whitespace (space, tab, newline,
/// carriage return), `/`, `-`, `.`, `,`, `\`. Any additional strings in
/// `extra_separators` are also treated as separators.
///
/// # Classification
///
/// Each non-separator chunk is examined for digit-to-alpha (or alpha-to-digit)
/// boundaries, which allows adjacent tokens like `"19october"` or `"August7"`
/// to be split and classified independently.
///
/// A chunk is classified as:
/// - [`Token::OrdinalDay`] — digit run followed by `st`, `nd`, `rd`, or `th`
///   (case-insensitive), e.g. `"19th"`, `"1ST"`.
/// - [`Token::MonthName`] — matches a known full name, standard three-letter
///   abbreviation, or unambiguous longer prefix (case-insensitive),
///   e.g. `"October"`, `"jan"`, `"Septem"`.
/// - [`Token::Numeric`] — a non-empty run of ASCII digits, e.g. `"19"`,
///   `"2014"`.
/// - Anything that matches none of the above (noise words, stray punctuation,
///   etc.) is silently discarded.
///
/// # Return value
///
/// At most **three** tokens are returned — one per possible date component.
/// Tokens beyond the third are discarded.
///
/// All returned slices point into the original `utterance` string.
///
/// # Examples
///
/// ```
/// use partial_date::extract::tokenise;
/// use partial_date::models::Token;
///
/// assert_eq!(
///     tokenise("19 October 2014", &[]),
///     vec![
///         Token::Numeric("19"),
///         Token::MonthName("October"),
///         Token::Numeric("2014"),
///     ]
/// );
///
/// assert_eq!(
///     tokenise("19th October,2015", &[]),
///     vec![
///         Token::OrdinalDay("19th"),
///         Token::MonthName("October"),
///         Token::Numeric("2015"),
///     ]
/// );
///
/// assert_eq!(
///     tokenise("19october", &[]),
///     vec![
///         Token::Numeric("19"),
///         Token::MonthName("october"),
///     ]
/// );
/// ```
pub fn tokenise<'a>(utterance: &'a str, extra_separators: &[String]) -> Vec<Token<'a>> {
    // Collect single-char separators (standard + single-char extras).
    // Multi-char extras are handled as substring boundaries below.
    const STANDARD_SEPS: &[char] = &[' ', '\t', '\n', '\r', '/', '-', '.', ',', '\\'];

    let mut sep_chars: Vec<char> = STANDARD_SEPS.to_vec();
    let mut multi_seps: Vec<&str> = Vec::new();

    for s in extra_separators {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            if chars.next().is_none() {
                // Single-char extra separator.
                sep_chars.push(first);
            } else {
                // Multi-char extra separator.
                multi_seps.push(s.as_str());
            }
        }
    }

    // Build a list of byte ranges within `utterance` that represent
    // separator spans, considering both single-char and multi-char separators.
    let sep_ranges = find_separator_ranges(utterance, &sep_chars, &multi_seps);

    // Extract non-separator spans as slices of `utterance`.
    let raw_chunks = spans_between_separators(utterance, &sep_ranges);

    // For each raw chunk, sub-split on digit↔alpha boundaries, classify each
    // sub-chunk, and collect up to 3 meaningful tokens.
    let mut tokens: Vec<Token<'a>> = Vec::with_capacity(3);

    'outer: for chunk in raw_chunks {
        // Skip chunks that contain no alphanumeric characters (stray
        // punctuation between separators, e.g. the ">" in ">type").
        if !chunk.chars().any(|c| c.is_alphanumeric()) {
            continue;
        }

        for sub in sub_split_on_boundary(chunk) {
            if tokens.len() == 3 {
                break 'outer;
            }
            if let Some(token) = classify(sub) {
                tokens.push(token);
            }
        }
    }

    tokens
}

// ---------------------------------------------------------------------------
// Separator range detection
// ---------------------------------------------------------------------------

/// A half-open byte-index range `[start, end)` within the utterance that
/// represents a separator span (to be skipped when extracting token chunks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SepRange {
    start: usize,
    end: usize,
}

/// Build a sorted, merged list of separator byte ranges within `utterance`.
///
/// A byte position is part of a separator range if it is occupied by a
/// single-char separator from `sep_chars` *or* by a multi-char separator
/// string from `multi_seps`.
fn find_separator_ranges(
    utterance: &str,
    sep_chars: &[char],
    multi_seps: &[&str],
) -> Vec<SepRange> {
    let mut ranges: Vec<SepRange> = Vec::new();

    // Single-char separators — iterate over chars to stay UTF-8 safe.
    for (byte_pos, ch) in utterance.char_indices() {
        if sep_chars.contains(&ch) {
            ranges.push(SepRange {
                start: byte_pos,
                end: byte_pos + ch.len_utf8(),
            });
        }
    }

    // Multi-char separators — find all occurrences as substrings.
    for sep in multi_seps {
        let mut search_from = 0usize;
        while let Some(pos) = utterance[search_from..].find(sep) {
            let abs_start = search_from + pos;
            let abs_end = abs_start + sep.len();
            ranges.push(SepRange {
                start: abs_start,
                end: abs_end,
            });
            search_from = abs_end;
        }
    }

    // Sort by start position then merge overlapping/adjacent ranges.
    ranges.sort_by_key(|r| r.start);
    merge_ranges(ranges)
}

/// Merge overlapping or adjacent [`SepRange`] entries.
fn merge_ranges(sorted: Vec<SepRange>) -> Vec<SepRange> {
    let mut merged: Vec<SepRange> = Vec::with_capacity(sorted.len());
    for r in sorted {
        if let Some(last) = merged.last_mut()
            && r.start <= last.end
        {
            // Overlapping or adjacent — extend the current range.
            last.end = last.end.max(r.end);
            continue;
        }
        merged.push(r);
    }
    merged
}

/// Return slices of `utterance` that lie *between* the separator ranges.
fn spans_between_separators<'a>(utterance: &'a str, sep_ranges: &[SepRange]) -> Vec<&'a str> {
    let mut spans: Vec<&'a str> = Vec::new();
    let mut pos = 0usize;

    for sep in sep_ranges {
        if pos < sep.start {
            spans.push(&utterance[pos..sep.start]);
        }
        pos = sep.end;
    }

    // Remainder after the last separator.
    if pos < utterance.len() {
        spans.push(&utterance[pos..]);
    }

    spans
}

// ---------------------------------------------------------------------------
// Digit↔alpha boundary splitting
// ---------------------------------------------------------------------------

/// Split a single chunk into sub-chunks at every digit↔alpha boundary.
///
/// For example `"19october"` → `["19", "october"]` and
/// `"August7"` → `["August", "7"]`.
///
/// Purely numeric or purely alphabetic chunks are returned as a single-element
/// slice.
///
/// **Ordinal suffixes are never split off from the leading digit run.**
/// `"1st"`, `"19th"`, `"3RD"` etc. are returned intact as single chunks so
/// that [`classify`] can identify them as [`Token::OrdinalDay`].
fn sub_split_on_boundary(chunk: &str) -> Vec<&str> {
    let bytes = chunk.as_bytes();
    let mut parts: Vec<&str> = Vec::new();
    let mut start = 0usize;

    for i in 1..bytes.len() {
        let prev_digit = bytes[i - 1].is_ascii_digit();
        let curr_digit = bytes[i].is_ascii_digit();
        let prev_alpha = bytes[i - 1].is_ascii_alphabetic();
        let curr_alpha = bytes[i].is_ascii_alphabetic();

        // Candidate digit→alpha or alpha→digit split point.
        if (prev_digit && curr_alpha) || (prev_alpha && curr_digit) {
            // Do not split if the tail (from `i` to the end of the chunk) is
            // an ordinal suffix — keep "19th", "1st", "3RD" intact.
            let tail = &chunk[i..];
            let tail_lower = tail.to_ascii_lowercase();
            if prev_digit && matches!(tail_lower.as_str(), "st" | "nd" | "rd" | "th") {
                continue;
            }

            parts.push(&chunk[start..i]);
            start = i;
        }
    }
    parts.push(&chunk[start..]);
    parts
}

// ---------------------------------------------------------------------------
// Token classification
// ---------------------------------------------------------------------------

/// Classify a single sub-chunk as a [`Token`], or return `None` for noise.
fn classify(sub: &str) -> Option<Token<'_>> {
    if sub.is_empty() {
        return None;
    }

    // Ordinal day: digit run + ordinal suffix (st/nd/rd/th).
    if let Some(token) = try_classify_ordinal(sub) {
        return Some(token);
    }

    // Pure numeric.
    if sub.chars().all(|c| c.is_ascii_digit()) {
        return Some(Token::Numeric(sub));
    }

    // Month name (full, abbreviated, or unambiguous prefix).
    if MonthName::try_from(sub).is_ok() {
        return Some(Token::MonthName(sub));
    }

    // Noise — discard.
    None
}

/// Return `Some(Token::OrdinalDay(sub))` if `sub` is a digit run followed by
/// `st`, `nd`, `rd`, or `th` (case-insensitive), `None` otherwise.
fn try_classify_ordinal(sub: &str) -> Option<Token<'_>> {
    // Find where the leading digit run ends.
    let digit_end = sub
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| i)?;

    if digit_end == 0 {
        return None; // Starts with a non-digit.
    }

    let suffix = &sub[digit_end..];
    let suffix_lower = suffix.to_ascii_lowercase();

    match suffix_lower.as_str() {
        "st" | "nd" | "rd" | "th" => Some(Token::OrdinalDay(sub)),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Extractor struct (scaffolding — used once extraction logic is implemented)
// ---------------------------------------------------------------------------

// Field will be used once extraction logic is implemented.
#[allow(dead_code)]
struct PartialDateExtractor {
    config: Config,
}

impl PartialDateExtractor {
    fn new(config: Config) -> Self {
        PartialDateExtractor { config }
    }
}
