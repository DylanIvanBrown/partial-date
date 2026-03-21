//! Tokenisation: splitting and classifying utterance chunks into date tokens.

use crate::models::{Config, DateComponent, MonthName, Token};

/// Split `utterance` on any standard separator or extra separator and classify
/// each resulting chunk as a [`Token`].
///
/// # What counts as a separator
///
/// The standard separator set is: ASCII whitespace (space, tab, newline,
/// carriage return), `/`, `-`, `.`, `,`, `\`. Any additional strings in
/// `config.extra_separators` are also treated as separators.
///
/// When `config.no_separator` is `true` and the utterance is a pure digit
/// string of length 6 or 8, it is sliced positionally according to
/// `config.component_order` rather than split on separators.
///
/// # Classification
///
/// Each non-separator chunk is examined for digit-to-alpha (or alpha-to-digit)
/// boundaries, allowing adjacent tokens like `"19october"` or `"August7"` to
/// be split and classified independently.
///
/// - [`Token::OrdinalDay`] — digit run followed by `st`, `nd`, `rd`, or `th`.
/// - [`Token::MonthName`] — full name, 3-letter abbreviation, unambiguous
///   prefix, or fuzzy misspelling.
/// - [`Token::Numeric`] — a run of ASCII digits; stores `(value, digit_count)`.
/// - Anything else (noise words, stray punctuation) is silently discarded.
///
/// At most **three** tokens are returned.
///
/// # Examples
///
/// ```
/// use partial_date::extract::tokenise;
/// use partial_date::models::{Config, MonthName, Token};
///
/// assert_eq!(
///     tokenise("19 October 2014", &Config::default()),
///     vec![
///         Token::Numeric(19, 2),
///         Token::MonthName(MonthName::October),
///         Token::Numeric(2014, 4),
///     ]
/// );
///
/// assert_eq!(
///     tokenise("19th October,2015", &Config::default()),
///     vec![
///         Token::OrdinalDay(19),
///         Token::MonthName(MonthName::October),
///         Token::Numeric(2015, 4),
///     ]
/// );
///
/// assert_eq!(
///     tokenise("19october", &Config::default()),
///     vec![
///         Token::Numeric(19, 2),
///         Token::MonthName(MonthName::October),
///     ]
/// );
/// ```
pub fn tokenise(utterance: &str, config: &Config) -> Vec<Token> {
    // No-separator path: pure-digit string of length 6 (DDMMYY) or 8 (DDMMYYYY).
    if config.no_separator
        && let Some(tokens) = try_tokenise_no_separator(utterance, &config.component_order)
    {
        return tokens;
    }

    // Standard separator path.
    const STANDARD_SEPS: &[char] = &[' ', '\t', '\n', '\r', '/', '-', '.', ',', '\\'];

    let mut separator_chars: Vec<char> = STANDARD_SEPS.to_vec();
    let mut multi_char_separators: Vec<&str> = Vec::new();

    for s in &config.extra_separators {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            if chars.next().is_none() {
                separator_chars.push(first);
            } else {
                multi_char_separators.push(s.as_str());
            }
        }
    }

    let separator_ranges =
        find_separator_ranges(utterance, &separator_chars, &multi_char_separators);
    let raw_chunks = spans_between_separators(utterance, &separator_ranges);

    // A date has at most three components, so we never need more than three
    // tokens. Pre-allocating exactly 3 avoids any reallocation.
    let mut tokens: Vec<Token> = Vec::with_capacity(3);

    // Label the outer loop so the inner loop can break out of both at once
    // when the token limit is reached (see the `break 'outer` below).
    'outer: for chunk in raw_chunks {
        // Skip chunks that contain no alphanumeric characters at all — e.g. a
        // stray "!" or "--" that survived the separator pass. There is nothing
        // here that could become a token.
        if !chunk.chars().any(|c| c.is_alphanumeric()) {
            continue;
        }

        // A chunk may contain a digit-to-alpha or alpha-to-digit boundary with
        // no separator — e.g. "19october" or "August7". sub_split_on_boundary
        // splits at those transitions so each run can be classified on its own.
        // For a plain chunk like "2014" this produces a single-element vec, so
        // the inner loop runs exactly once.
        //
        // Note: ordinal suffixes ("19th", "3rd") are intentionally NOT split —
        // the boundary detector leaves them intact so classify() can recognise
        // the whole thing as Token::OrdinalDay.
        for sub in sub_split_on_boundary(chunk) {
            // Stop as soon as we have day, month, and year — there is nothing
            // useful left to collect. `break 'outer` exits both loops at once;
            // a plain `break` would only exit this inner loop and the outer
            // loop would continue consuming chunks needlessly.
            if tokens.len() == 3 {
                break 'outer;
            }

            // classify() tries to turn the sub-slice into a Token::OrdinalDay,
            // Token::Numeric, or Token::MonthName. Noise words ("the", "of")
            // and unrecognised strings return None and are silently dropped —
            // no error, no placeholder.
            if let Some(token) = classify(sub) {
                tokens.push(token);
            }
        }
    }

    tokens
}

/// Attempt to tokenise a no-separator pure-digit string by positional slicing.
///
/// Handles lengths 6 (two-digit year) and 8 (four-digit year). Returns `None`
/// if the string is not purely digits or not one of the expected lengths.
fn try_tokenise_no_separator(
    utterance: &str,
    order: &crate::models::ComponentOrder,
) -> Option<Vec<Token>> {
    let bytes = utterance.as_bytes();
    if !bytes.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }

    // Determine slice widths: year slot gets 4 digits (8-char) or 2 (6-char).
    let (year_width, total) = match bytes.len() {
        8 => (4usize, 8usize),
        6 => (2usize, 6usize),
        _ => return None,
    };

    // Build (component, width) pairs in order.
    let widths = [
        (
            order.first,
            if order.first == DateComponent::Year {
                year_width
            } else {
                2
            },
        ),
        (
            order.second,
            if order.second == DateComponent::Year {
                year_width
            } else {
                2
            },
        ),
        (
            order.third,
            if order.third == DateComponent::Year {
                year_width
            } else {
                2
            },
        ),
    ];

    // Verify widths sum to the total length.
    let sum: usize = widths.iter().map(|(_, w)| w).sum();
    if sum != total {
        return None;
    }

    let mut pos = 0usize;
    let mut tokens: Vec<Token> = Vec::with_capacity(3);
    for (_, width) in &widths {
        let slice = &utterance[pos..pos + width];
        let digit_count = *width as u8;
        let value: i16 = slice.parse().ok()?;
        tokens.push(Token::Numeric(value, digit_count));
        pos += width;
    }
    Some(tokens)
}

// ---------------------------------------------------------------------------
// Separator range detection
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SeparatorRange {
    start: usize,
    end: usize,
}

fn find_separator_ranges(
    utterance: &str,
    separator_chars: &[char],
    multi_char_separators: &[&str],
) -> Vec<SeparatorRange> {
    let mut ranges: Vec<SeparatorRange> = Vec::new();

    for (byte_pos, ch) in utterance.char_indices() {
        if separator_chars.contains(&ch) {
            ranges.push(SeparatorRange {
                start: byte_pos,
                end: byte_pos + ch.len_utf8(),
            });
        }
    }

    for separator in multi_char_separators {
        let mut search_from = 0usize;
        while let Some(pos) = utterance[search_from..].find(separator) {
            let absolute_start = search_from + pos;
            let absolute_end = absolute_start + separator.len();
            ranges.push(SeparatorRange {
                start: absolute_start,
                end: absolute_end,
            });
            search_from = absolute_end;
        }
    }

    ranges.sort_by_key(|r| r.start);
    merge_ranges(ranges)
}

fn merge_ranges(sorted: Vec<SeparatorRange>) -> Vec<SeparatorRange> {
    let mut merged: Vec<SeparatorRange> = Vec::with_capacity(sorted.len());
    for r in sorted {
        if let Some(last) = merged.last_mut()
            && r.start <= last.end
        {
            last.end = last.end.max(r.end);
            continue;
        }
        merged.push(r);
    }
    merged
}

fn spans_between_separators<'u>(
    utterance: &'u str,
    separator_ranges: &[SeparatorRange],
) -> Vec<&'u str> {
    let mut spans: Vec<&'u str> = Vec::new();
    let mut pos = 0usize;

    for separator in separator_ranges {
        if pos < separator.start {
            spans.push(&utterance[pos..separator.start]);
        }
        pos = separator.end;
    }

    if pos < utterance.len() {
        spans.push(&utterance[pos..]);
    }

    spans
}

// ---------------------------------------------------------------------------
// Digit↔alpha boundary splitting
// ---------------------------------------------------------------------------

fn sub_split_on_boundary(chunk: &str) -> Vec<&str> {
    let bytes = chunk.as_bytes();
    let mut parts: Vec<&str> = Vec::new();
    let mut start = 0usize;

    for i in 1..bytes.len() {
        let prev_digit = bytes[i - 1].is_ascii_digit();
        let curr_digit = bytes[i].is_ascii_digit();
        let prev_alpha = bytes[i - 1].is_ascii_alphabetic();
        let curr_alpha = bytes[i].is_ascii_alphabetic();

        if (prev_digit && curr_alpha) || (prev_alpha && curr_digit) {
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

fn classify(sub: &str) -> Option<Token> {
    if sub.is_empty() {
        return None;
    }

    if let Some(token) = try_classify_ordinal(sub) {
        return Some(token);
    }

    if sub.chars().all(|c| c.is_ascii_digit()) {
        let digit_count = sub.len() as u8;
        return sub
            .parse::<i16>()
            .ok()
            .map(|v| Token::Numeric(v, digit_count));
    }

    if let Ok(month) = MonthName::try_from(sub) {
        return Some(Token::MonthName(month));
    }

    None
}

fn try_classify_ordinal(sub: &str) -> Option<Token> {
    let digit_end = sub
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| i)?;

    if digit_end == 0 {
        return None;
    }

    let suffix = &sub[digit_end..];
    let suffix_lower = suffix.to_ascii_lowercase();

    match suffix_lower.as_str() {
        "st" | "nd" | "rd" | "th" => {
            let day_number = sub[..digit_end].parse::<u8>().ok()?;
            Some(Token::OrdinalDay(day_number))
        }
        _ => None,
    }
}
