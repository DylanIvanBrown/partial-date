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

    let mut sep_chars: Vec<char> = STANDARD_SEPS.to_vec();
    let mut multi_seps: Vec<&str> = Vec::new();

    for s in &config.extra_separators {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            if chars.next().is_none() {
                sep_chars.push(first);
            } else {
                multi_seps.push(s.as_str());
            }
        }
    }

    let sep_ranges = find_separator_ranges(utterance, &sep_chars, &multi_seps);
    let raw_chunks = spans_between_separators(utterance, &sep_ranges);

    let mut tokens: Vec<Token> = Vec::with_capacity(3);

    'outer: for chunk in raw_chunks {
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
struct SepRange {
    start: usize,
    end: usize,
}

fn find_separator_ranges(
    utterance: &str,
    sep_chars: &[char],
    multi_seps: &[&str],
) -> Vec<SepRange> {
    let mut ranges: Vec<SepRange> = Vec::new();

    for (byte_pos, ch) in utterance.char_indices() {
        if sep_chars.contains(&ch) {
            ranges.push(SepRange {
                start: byte_pos,
                end: byte_pos + ch.len_utf8(),
            });
        }
    }

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

    ranges.sort_by_key(|r| r.start);
    merge_ranges(ranges)
}

fn merge_ranges(sorted: Vec<SepRange>) -> Vec<SepRange> {
    let mut merged: Vec<SepRange> = Vec::with_capacity(sorted.len());
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

fn spans_between_separators<'u>(utterance: &'u str, sep_ranges: &[SepRange]) -> Vec<&'u str> {
    let mut spans: Vec<&'u str> = Vec::new();
    let mut pos = 0usize;

    for sep in sep_ranges {
        if pos < sep.start {
            spans.push(&utterance[pos..sep.start]);
        }
        pos = sep.end;
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
            let n = sub[..digit_end].parse::<u8>().ok()?;
            Some(Token::OrdinalDay(n))
        }
        _ => None,
    }
}
