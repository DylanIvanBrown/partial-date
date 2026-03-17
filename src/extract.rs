//! Extraction functions for partial date parsing.

use crate::models::{
    ComponentOrder, Config, DateComponent, Day, DayConfig, Extracted, Input, IsExpected, Month,
    MonthConfig, MonthName, PartialDate, Token, TwoDigitYearExpansion, Year, YearConfig,
};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Extract a partial date from the given input.
///
/// Returns a [`PartialDate`] where each component is either
/// [`Extracted::Found`], [`Extracted::Defaulted`], or [`Extracted::NotFound`]
/// depending on what could be determined from the utterance and config.
pub fn extract(input: Input) -> PartialDate {
    let config = input.config.unwrap_or_default();
    let tokens = tokenise(&input.utterance, &config);
    let (day_raw, month_raw, year_raw) = interpret_tokens(&tokens, &config);

    let day_val = validate_day(day_raw, &config.day);
    let month_val = validate_month(month_raw, &config.month);
    let year_val = validate_year(year_raw, &config.year);

    PartialDate {
        day: Day {
            value: apply_default(day_val, config.day.default),
        },
        month: Month {
            number: apply_default(month_val.map(|(n, _)| n), config.month.default),
            name: match month_val {
                Some((_, Some(name))) => Extracted::Found(name),
                _ => Extracted::NotFound,
            },
        },
        year: Year {
            value: apply_default(year_val, config.year.default),
        },
    }
}

// ---------------------------------------------------------------------------
// Tokenisation
// ---------------------------------------------------------------------------

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
fn try_tokenise_no_separator(utterance: &str, order: &ComponentOrder) -> Option<Vec<Token>> {
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

// ---------------------------------------------------------------------------
// Year expansion
// ---------------------------------------------------------------------------

/// Expand a raw numeric value into a full year, given its original digit count.
///
/// - 4-digit values are returned unchanged.
/// - 2-digit values are expanded according to `config.two_digit_expansion`.
/// - Any other digit count returns `None` (invalid year).
fn expand_year(value: i16, digit_count: u8, config: &YearConfig) -> Option<i32> {
    match digit_count {
        4 => Some(value as i32),
        2 => {
            let raw = value as i32;
            let expanded = match &config.two_digit_expansion {
                TwoDigitYearExpansion::Literal => raw,
                TwoDigitYearExpansion::Always2000s => 2000 + raw,
                TwoDigitYearExpansion::SlidingWindow(wr) => {
                    // The lower range covers values 0..pivot, the upper covers pivot..100.
                    let pivot = wr.lower_range.max - wr.lower_range.min;
                    if raw < pivot {
                        wr.lower_range.min + raw
                    } else {
                        wr.upper_range.min + (raw - pivot)
                    }
                }
            };
            Some(expanded)
        }
        _ => None, // 1, 3, 5+ digit numbers are not valid years
    }
}

// ---------------------------------------------------------------------------
// Token interpretation
// ---------------------------------------------------------------------------

/// The raw candidates extracted from the token list, before validation.
///
/// - `day_raw`: `(value, digit_count)` if a day candidate was found.
/// - `month_raw`: `(number, Option<MonthName>)` if a month candidate was found.
/// - `year_raw`: expanded year value if a year candidate was found.
type RawDay = Option<(u8, u8)>;
type RawMonth = Option<(u8, Option<MonthName>)>;
type RawYear = Option<i32>;

/// Interpret up to 3 tokens as day, month, and year candidates.
fn interpret_tokens(tokens: &[Token], config: &Config) -> (RawDay, RawMonth, RawYear) {
    // Separate by kind first.
    let ordinals: Vec<u8> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::OrdinalDay(n) => Some(*n),
            _ => None,
        })
        .collect();

    let month_names: Vec<MonthName> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::MonthName(m) => Some(*m),
            _ => None,
        })
        .collect();

    let numerics: Vec<(i16, u8)> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::Numeric(v, d) => Some((*v, *d)),
            _ => None,
        })
        .collect();

    let day_expected = config.day.expected != IsExpected::No;
    let month_expected = config.month.expected != IsExpected::No;
    let year_expected = config.year.expected != IsExpected::No;

    // OrdinalDay tokens are unambiguously a day.
    let ordinal_day: Option<u8> = ordinals.into_iter().next();

    // MonthName tokens are unambiguously a month.
    let named_month: Option<MonthName> = month_names.into_iter().next();

    // With a named month, numerics are assigned to day and year slots.
    if named_month.is_some() {
        let (day_raw, year_raw) =
            assign_day_and_year(&numerics, ordinal_day, &config.component_order, config);
        let month_raw = named_month.map(|m| (m.number(), Some(m)));
        return (
            if day_expected { day_raw } else { None },
            if month_expected { month_raw } else { None },
            if year_expected { year_raw } else { None },
        );
    }

    // No named month — all information comes from numerics and ordinals.
    // Use positional assignment driven by ComponentOrder, passing IsExpected
    // flags so disabled slots are never filled.
    let (day_raw, month_raw, year_raw) = assign_positional(
        &numerics,
        ordinal_day,
        config,
        day_expected,
        month_expected,
        year_expected,
    );

    (day_raw, month_raw, year_raw)
}

/// Assign day and year from the numeric tokens when the month is already
/// known (from a MonthName token).
fn assign_day_and_year(
    numerics: &[(i16, u8)],
    ordinal_day: Option<u8>,
    order: &ComponentOrder,
    config: &Config,
) -> (RawDay, RawYear) {
    // Ordinal always wins as day.
    if let Some(d) = ordinal_day {
        // Any remaining numeric is the year.
        let year = numerics
            .iter()
            .find_map(|(v, d)| expand_year(*v, *d, &config.year));
        return (Some((d, 1)), year);
    }

    match numerics {
        [] => (None, None),
        [(v, d)] => {
            // One numeric — must figure out if it's a day or year.
            if let Some(year) = expand_year(*v, *d, &config.year) {
                // 4-digit → definitely a year; 2-digit could be either.
                if *d == 4 {
                    return (None, Some(year));
                }
            }
            // Fits as a day (1–31)?  Treat as day.
            if *v >= 1 && *v <= 31 {
                (Some((*v as u8, *d)), None)
            } else {
                (None, expand_year(*v, *d, &config.year))
            }
        }
        [(v0, d0), (v1, d1), ..] => {
            // Two or more numerics. Use component order to determine which slot
            // (ignoring the month slot, which is already filled) comes first.
            // The non-month slots in order determine day vs year position.
            let non_month_in_order = non_month_positions(order);

            // First non-month position in the order gets the first numeric, etc.
            let first_is_year_slot = non_month_in_order
                .first()
                .map(|c| *c == DateComponent::Year)
                .unwrap_or(false);

            // first_is_year_slot=true  → first numeric is year, second is day
            // first_is_year_slot=false → first numeric is day,  second is year
            let (day_num, day_dc, year_num, year_dc) = if first_is_year_slot {
                (*v1, *d1, *v0, *d0)
            } else {
                (*v0, *d0, *v1, *d1)
            };

            // Unambiguous override: if day_num > 31 it can't be a day; swap.
            let (day_num, day_dc, year_num, year_dc) = if !(1..=31).contains(&day_num) {
                (year_num, year_dc, day_num, day_dc)
            } else {
                (day_num, day_dc, year_num, year_dc)
            };

            let day = if (1..=31).contains(&day_num) {
                Some((day_num as u8, day_dc))
            } else {
                None
            };
            let year = expand_year(year_num, year_dc, &config.year);
            (day, year)
        }
    }
}

/// Returns the non-Month components from the order, preserving their relative
/// order.
fn non_month_positions(order: &ComponentOrder) -> Vec<DateComponent> {
    [order.first, order.second, order.third]
        .iter()
        .filter(|c| **c != DateComponent::Month)
        .copied()
        .collect()
}

/// Assign day, month, and year purely from numeric tokens using ComponentOrder,
/// with unambiguous-override rules.
///
/// The `day_ok`, `month_ok`, `year_ok` flags suppress slots that are marked
/// `IsExpected::No` — this ensures disabled components are never filled even
/// when a matching value is present.
fn assign_positional(
    numerics: &[(i16, u8)],
    ordinal_day: Option<u8>,
    config: &Config,
    day_ok: bool,
    month_ok: bool,
    year_ok: bool,
) -> (RawDay, RawMonth, RawYear) {
    let order = &config.component_order;

    let (day_raw, month_raw, year_raw) = match (ordinal_day, numerics) {
        // Nothing at all.
        (None, []) => (None, None, None),

        // Only an ordinal — day only.
        (Some(d), []) => (Some((d, 1)), None, None),

        // Ordinal + one numeric.
        (Some(d), [(v, dc)]) => {
            let (month_raw, year_raw) =
                assign_month_or_year_from_one(*v, *dc, config, month_ok, year_ok);
            (Some((d, 1)), month_raw, year_raw)
        }

        // Ordinal + two numerics — month and year.
        (Some(d), [(v0, d0), (v1, d1), ..]) => {
            let (m_val, m_dc, y_val, y_dc) = split_month_year_by_order(*v0, *d0, *v1, *d1, order);
            let month_raw = to_month_raw(m_val, m_dc);
            let year_raw = expand_year(y_val, y_dc, &config.year);
            (Some((d, 1)), month_raw, year_raw)
        }

        // No ordinal, one numeric.
        (None, [(v, dc)]) => assign_one_numeric(*v, *dc, config, day_ok, month_ok, year_ok),

        // No ordinal, two numerics.
        (None, [(v0, d0), (v1, d1)]) => {
            assign_two_numerics(*v0, *d0, *v1, *d1, config, day_ok, month_ok, year_ok)
        }

        // No ordinal, three numerics.
        (None, three_or_more) => assign_three_numerics(three_or_more, config),
    };

    (
        if day_ok { day_raw } else { None },
        if month_ok { month_raw } else { None },
        if year_ok { year_raw } else { None },
    )
}

/// Assign a single numeric token to the most appropriate slot given config.
fn assign_one_numeric(
    v: i16,
    dc: u8,
    config: &Config,
    day_ok: bool,
    month_ok: bool,
    year_ok: bool,
) -> (RawDay, RawMonth, RawYear) {
    // 4-digit number → always year.
    if dc == 4 {
        return if year_ok {
            (None, None, expand_year(v, dc, &config.year))
        } else {
            (None, None, None)
        };
    }

    // A number > 31 can only be a year (two-digit expansion).
    if v > 31 {
        return if year_ok {
            (None, None, expand_year(v, dc, &config.year))
        } else {
            (None, None, None)
        };
    }

    // A number > 12 and ≤ 31 prefers to be a day, but can also be a two-digit year.
    if v > 12 {
        if day_ok {
            return (Some((v as u8, dc)), None, None);
        }
        // Day is disabled — try as a two-digit year instead.
        return if year_ok {
            (None, None, expand_year(v, dc, &config.year))
        } else {
            (None, None, None)
        };
    }

    // Value ≤ 12: could be day, month, or two-digit year.
    // Walk the component order, trying the first enabled slot that fits.
    for component in [
        config.component_order.first,
        config.component_order.second,
        config.component_order.third,
    ] {
        match component {
            DateComponent::Year if year_ok => {
                return (None, None, expand_year(v, dc, &config.year));
            }
            DateComponent::Month if month_ok && (1..=12).contains(&v) => {
                return (None, to_month_raw(v, dc), None);
            }
            DateComponent::Day if day_ok && (1..=31).contains(&v) => {
                return (Some((v as u8, dc)), None, None);
            }
            _ => {}
        }
    }
    (None, None, None)
}

/// Decide whether a single numeric is a month or year (used when day is
/// already known from an ordinal).
fn assign_month_or_year_from_one(
    v: i16,
    dc: u8,
    config: &Config,
    month_ok: bool,
    year_ok: bool,
) -> (RawMonth, RawYear) {
    if dc == 4 {
        return (
            None,
            if year_ok {
                expand_year(v, dc, &config.year)
            } else {
                None
            },
        );
    }
    if (1..=12).contains(&v) {
        // Could be month or two-digit year; use order.
        let order = &config.component_order;
        let non_day: Vec<DateComponent> = [order.first, order.second, order.third]
            .iter()
            .filter(|c| **c != DateComponent::Day)
            .copied()
            .collect();
        if non_day.first() == Some(&DateComponent::Year) {
            (
                None,
                if year_ok {
                    expand_year(v, dc, &config.year)
                } else {
                    None
                },
            )
        } else {
            (if month_ok { to_month_raw(v, dc) } else { None }, None)
        }
    } else {
        (
            None,
            if year_ok {
                expand_year(v, dc, &config.year)
            } else {
                None
            },
        )
    }
}

/// Assign two numeric tokens to (day, month, year) — one will be NotFound.
#[allow(clippy::too_many_arguments)]
fn assign_two_numerics(
    v0: i16,
    d0: u8,
    v1: i16,
    d1: u8,
    config: &Config,
    day_ok: bool,
    month_ok: bool,
    year_ok: bool,
) -> (RawDay, RawMonth, RawYear) {
    // If either is 4-digit, it is the year.
    let (year_val, year_dc, other_val, other_dc) = if d0 == 4 {
        (v0, d0, v1, d1)
    } else if d1 == 4 {
        (v1, d1, v0, d0)
    } else {
        // Neither is 4-digit.
        if v0 > 31 {
            // recurse with order swapped so the large value is in position 1
            return assign_two_numerics(v1, d1, v0, d0, config, day_ok, month_ok, year_ok);
        }
        if v1 > 31 {
            let year = if year_ok {
                expand_year(v1, d1, &config.year)
            } else {
                None
            };
            let day_or_month = assign_remaining_as_day_or_month(v0, d0, config);
            return match day_or_month {
                DayOrMonth::Day if day_ok => (Some((v0 as u8, d0)), None, year),
                DayOrMonth::Month if month_ok => (None, to_month_raw(v0, d0), year),
                _ => (None, None, year),
            };
        }
        // Both ≤ 31 — no clear year; split as day/month.
        return assign_day_and_month(v0, d0, v1, d1, config);
    };

    let year = if year_ok {
        expand_year(year_val, year_dc, &config.year)
    } else {
        None
    };
    let day_or_month = assign_remaining_as_day_or_month(other_val, other_dc, config);
    match day_or_month {
        DayOrMonth::Day if day_ok => (Some((other_val as u8, other_dc)), None, year),
        DayOrMonth::Month if month_ok => (None, to_month_raw(other_val, other_dc), year),
        _ => (None, None, year),
    }
}

enum DayOrMonth {
    Day,
    Month,
}

/// Decide whether a single remaining numeric (after year is known) is a day
/// or a month.
fn assign_remaining_as_day_or_month(v: i16, _dc: u8, config: &Config) -> DayOrMonth {
    if v > 12 {
        return DayOrMonth::Day; // Can't be a month.
    }
    // Ambiguous — use component order: whichever of Day/Month appears first.
    let order = &config.component_order;
    for component in [order.first, order.second, order.third] {
        match component {
            DateComponent::Day => return DayOrMonth::Day,
            DateComponent::Month => return DayOrMonth::Month,
            DateComponent::Year => {}
        }
    }
    DayOrMonth::Day // Fallback.
}

/// Assign two numerics to (day, month) with no year.
fn assign_day_and_month(
    v0: i16,
    d0: u8,
    v1: i16,
    d1: u8,
    config: &Config,
) -> (RawDay, RawMonth, RawYear) {
    // Unambiguous override: a value > 12 can only be a day.
    if v0 > 12 && v1 <= 12 {
        return (Some((v0 as u8, d0)), to_month_raw(v1, d1), None);
    }
    if v1 > 12 && v0 <= 12 {
        return (Some((v1 as u8, d1)), to_month_raw(v0, d0), None);
    }

    // Both ≤ 12 — use component order.
    let order = &config.component_order;
    // Map first/second numeric to first/second non-Year component in order.
    let non_year: Vec<DateComponent> = [order.first, order.second, order.third]
        .iter()
        .filter(|c| **c != DateComponent::Year)
        .copied()
        .collect();

    let (first_component, second_component) = match non_year.as_slice() {
        [a, b, ..] => (*a, *b),
        _ => (DateComponent::Day, DateComponent::Month),
    };

    let (day_val, day_dc, month_val, month_dc) = match first_component {
        DateComponent::Day => (v0, d0, v1, d1),
        DateComponent::Month => (v1, d1, v0, d0),
        DateComponent::Year => (v0, d0, v1, d1), // Shouldn't happen.
    };
    let _ = second_component; // Used implicitly via the swap above.

    if (1..=31).contains(&day_val) {
        (
            Some((day_val as u8, day_dc)),
            to_month_raw(month_val, month_dc),
            None,
        )
    } else {
        (None, to_month_raw(month_val, month_dc), None)
    }
}

/// Assign three numeric tokens to (day, month, year) using ComponentOrder.
fn assign_three_numerics(numerics: &[(i16, u8)], config: &Config) -> (RawDay, RawMonth, RawYear) {
    let order = &config.component_order;
    let positions = [order.first, order.second, order.third];

    // Build initial positional assignment.
    let mut day_val: Option<(i16, u8)> = None;
    let mut month_val: Option<(i16, u8)> = None;
    let mut year_val: Option<(i16, u8)> = None;

    for (i, component) in positions.iter().enumerate() {
        if let Some(&(v, dc)) = numerics.get(i) {
            match component {
                DateComponent::Day => day_val = Some((v, dc)),
                DateComponent::Month => month_val = Some((v, dc)),
                DateComponent::Year => year_val = Some((v, dc)),
            }
        }
    }

    // Unambiguous override: if a 4-digit value ended up in a day or month
    // slot, and the year slot has a 1-2 digit value, swap them.
    if let (Some((dv, 4)), Some((yv, ydc))) = (day_val, year_val)
        && ydc <= 2
    {
        year_val = Some((dv, 4));
        day_val = Some((yv, ydc));
    }
    if let (Some((mv, 4)), Some((yv, ydc))) = (month_val, year_val)
        && ydc <= 2
    {
        year_val = Some((mv, 4));
        month_val = Some((yv, ydc));
    }

    // Unambiguous override: when the month component comes BEFORE the day
    // component in the configured order (e.g. MDY), a value > 12 in the month
    // slot that is ≤ 31 must be a day — swap with the day slot if the day
    // slot's value is a valid month (≤ 12).
    //
    // We only apply this when month precedes day in the order, because that is
    // the configuration most likely to produce accidental day/month swaps in
    // real input (e.g. "31/12/19" with MDY).  When day precedes month (e.g.
    // DMY), we trust the positional assignment and let validation reject an
    // out-of-range month value.
    let month_before_day = [order.first, order.second, order.third]
        .iter()
        .position(|c| *c == DateComponent::Month)
        < [order.first, order.second, order.third]
            .iter()
            .position(|c| *c == DateComponent::Day);

    if month_before_day
        && let Some((mv, mdc)) = month_val
        && mv > 12
        && mv <= 31
        && let Some((dv, ddc)) = day_val
        && dv <= 12
    {
        month_val = Some((dv, ddc));
        day_val = Some((mv, mdc));
    }

    let raw_day = day_val.and_then(|(v, dc)| {
        if (1..=31).contains(&v) {
            Some((v as u8, dc))
        } else {
            None
        }
    });
    let raw_month = month_val.and_then(|(v, dc)| to_month_raw(v, dc));
    let raw_year = year_val.and_then(|(v, dc)| expand_year(v, dc, &config.year));

    (raw_day, raw_month, raw_year)
}

/// Split two numerics into (month, year) using the component order, for the
/// case where the day is already known from an ordinal.
fn split_month_year_by_order(
    v0: i16,
    d0: u8,
    v1: i16,
    d1: u8,
    order: &ComponentOrder,
) -> (i16, u8, i16, u8) {
    // Find which of Month/Year comes first in the order (ignoring Day).
    let non_day: Vec<DateComponent> = [order.first, order.second, order.third]
        .iter()
        .filter(|c| **c != DateComponent::Day)
        .copied()
        .collect();

    let first_is_month = non_day.first() == Some(&DateComponent::Month);

    // Unambiguous override: 4-digit is always year.
    if d0 == 4 {
        return (v1, d1, v0, d0); // (month, year)
    }
    if d1 == 4 {
        return (v0, d0, v1, d1);
    }

    if first_is_month {
        (v0, d0, v1, d1)
    } else {
        (v1, d1, v0, d0)
    }
}

/// Convert a raw (value, digit_count) numeric into an optional month tuple.
///
/// The `MonthName` is derived from the number when possible, so that
/// numeric month inputs (e.g. `"6"` in `"18/6. 2013"`) also populate
/// `month.name`.
fn to_month_raw(v: i16, _dc: u8) -> Option<(u8, Option<MonthName>)> {
    if (1..=12).contains(&v) {
        let name = MonthName::try_from(v as u8).ok();
        Some((v as u8, name))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_day(raw: RawDay, config: &DayConfig) -> Option<u8> {
    raw.and_then(|(v, _dc)| {
        if v >= config.min && v <= config.max {
            Some(v)
        } else {
            None
        }
    })
}

fn validate_month(raw: RawMonth, config: &MonthConfig) -> RawMonth {
    raw.and_then(|(n, name)| {
        if n >= config.min && n <= config.max {
            Some((n, name))
        } else {
            None
        }
    })
}

fn validate_year(raw: RawYear, config: &YearConfig) -> RawYear {
    raw.and_then(|y| {
        if y >= config.min && y <= config.max {
            Some(y)
        } else {
            None
        }
    })
}

// ---------------------------------------------------------------------------
// Default application
// ---------------------------------------------------------------------------

fn apply_default<T: Copy>(val: Option<T>, default: Option<T>) -> Extracted<T> {
    match val {
        Some(v) => Extracted::Found(v),
        None => match default {
            Some(d) => Extracted::Defaulted(d),
            None => Extracted::NotFound,
        },
    }
}
