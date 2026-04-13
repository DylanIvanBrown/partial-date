//! Word-number recognition and substitution.
//!
//! This module converts English written numbers (e.g. `"twenty-three"`,
//! `"one thousand nine hundred eighty-four"`) into their digit equivalents,
//! allowing the tokeniser to treat them the same as numerals.
//!
//! # Approach
//!
//! The public entry point is [`replace_word_numbers`], which scans an
//! utterance for the longest contiguous word-number span it can parse and
//! replaces it with the decimal representation.  Multiple non-overlapping
//! spans are replaced left-to-right.
//!
//! Each individual word is fuzzy-matched against the canonical English
//! number vocabulary using [`crate::levenshtein::levenshtein_ratio`].  This
//! lets the module tolerate common typos, repeated characters, transpositions,
//! and phonetic spelling patterns from non-English speakers (see test suite).
//!
//! Ordinal forms (`"first"`, `"twenty-third"`, etc.) are included in the
//! vocabulary so they parse identically to their cardinal equivalents.
//!
//! Common English stop words (`"the"`, `"of"`, `"and"`, etc.) are explicitly
//! excluded so they cannot produce false-positive number matches.
//!
//! # Supported range
//!
//! 1 – 3000, covering every value that is meaningful as a day (1–31), month
//! (1–12), or year (1–3000) in the date extraction context.
//!
//! # Grammar
//!
//! ```text
//! number   ::= thousands? hundreds? tens_units
//! thousands ::= unit "thousand"
//! hundreds  ::= unit "hundred"
//! tens_units ::= tens unit?   (e.g. "twenty", "twenty-one", "twenty-third")
//!              | teen          (e.g. "fourteenth")
//!              | unit          (e.g. "seventh")
//!              | (empty)
//! ```
//!
//! Hyphenated compound words (`"twenty-one"`) are split on `-` before
//! individual word matching so the hyphen is treated as a separator.

use crate::levenshtein::levenshtein_ratio;

// ---------------------------------------------------------------------------
// Fuzzy matching threshold
// ---------------------------------------------------------------------------

/// Minimum similarity ratio for a word to be accepted as a number word.
///
/// 0.65 is high enough to prevent cross-category false positives such as
/// `"six"` → `"sixty"` (ratio 0.60) and `"three"` → `"thirteen"` (ratio
/// 0.625), while still accepting the common English misspellings and typos
/// exercised by this library's tests (e.g. `"theer"` → `"three"` at 0.80,
/// `"sevne"` → `"seven"` at 0.86).
///
/// Non-English phonetic patterns (Swahili, Hausa, Zulu, etc.) often score
/// below this threshold and are a known limitation — see the ignored
/// `word_numbers_non_english` test module.
const MATCH_THRESHOLD: f32 = 0.65;
//TODO: Increase the threshold above. The examples above are not helpful, especially naayiti. That does not need to be transformed to eighty

// ---------------------------------------------------------------------------
// Stop-word blocklist
// ---------------------------------------------------------------------------

/// Common English words that must never be interpreted as number words,
/// regardless of their fuzzy similarity to number vocabulary.
///
/// Without this list, short words like `"the"` (ratio 0.60 against `"three"`),
/// `"on"` (ratio 0.67 against `"one"`), and `"or"` (ratio 0.67 against
/// `"four"`) would produce false-positive number matches.
static STOP_WORDS: &[&str] = &[
    "the", "of", "on", "or", "in", "at", "to", "a", "an", "and", "as", "is", "it", "be", "do",
    "so", "up", "by", "if", "no", "my", "we", "he", "me", "us", "am", "are", "was", "not", "but",
    "day", "date", "year", "month", "time", "age",
];

/// Return `true` if `word` is a known stop word and should never be parsed as
/// a number word.
fn is_stop_word(word: &str) -> bool {
    STOP_WORDS.contains(&word)
}

// ---------------------------------------------------------------------------
// Vocabulary tables
// ---------------------------------------------------------------------------

/// Cardinal and ordinal spellings for the units 1–9.
///
/// Each entry is `(canonical_spelling, value)`.  Multiple entries with the
/// same value allow both `"one"` and `"first"` to resolve to 1.
static UNITS: &[(&str, i32)] = &[
    // Cardinals
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    // Ordinals
    ("first", 1),
    ("second", 2),
    ("third", 3),
    ("fourth", 4),
    ("fifth", 5),
    ("sixth", 6),
    ("seventh", 7),
    ("eighth", 8),
    ("ninth", 9),
];

/// Cardinal and ordinal spellings for the teens 10–19.
static TEENS: &[(&str, i32)] = &[
    // Cardinals
    ("ten", 10),
    ("eleven", 11),
    ("twelve", 12),
    ("thirteen", 13),
    ("fourteen", 14),
    ("fifteen", 15),
    ("sixteen", 16),
    ("seventeen", 17),
    ("eighteen", 18),
    ("nineteen", 19),
    // Ordinals
    ("tenth", 10),
    ("eleventh", 11),
    ("twelfth", 12),
    ("thirteenth", 13),
    ("fourteenth", 14),
    ("fifteenth", 15),
    ("sixteenth", 16),
    ("seventeenth", 17),
    ("eighteenth", 18),
    ("nineteenth", 19),
];

/// Cardinal and ordinal spellings for the tens 20–90.
static TENS: &[(&str, i32)] = &[
    // Cardinals
    ("twenty", 20),
    ("thirty", 30),
    ("forty", 40),
    ("fifty", 50),
    ("sixty", 60),
    ("seventy", 70),
    ("eighty", 80),
    ("ninety", 90),
    // Ordinals
    ("twentieth", 20),
    ("thirtieth", 30),
    ("fortieth", 40),
    ("fiftieth", 50),
    ("sixtieth", 60),
    ("seventieth", 70),
    ("eightieth", 80),
    ("ninetieth", 90),
];

/// The word "hundred" (and ordinal "hundredth").
static HUNDREDS: &[&str] = &["hundred", "hundredth"];
/// The word "thousand" (and ordinal "thousandth").
static THOUSANDS: &[&str] = &["thousand", "thousandth"];

// ---------------------------------------------------------------------------
// Internal word matching
// ---------------------------------------------------------------------------

/// Try to fuzzy-match `word` against every entry in `table`.
///
/// Returns the value of the best-matching entry if its similarity is at or
/// above [`MATCH_THRESHOLD`], or `None` if nothing is close enough.
/// Stop words are rejected before any table lookup.
fn best_match(word: &str, table: &[(&str, i32)]) -> Option<i32> {
    if is_stop_word(word) {
        return None;
    }
    table
        .iter()
        .map(|&(canonical, value)| (levenshtein_ratio(word, canonical), value))
        .filter(|&(ratio, _)| ratio >= MATCH_THRESHOLD)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(_, value)| value)
}

/// Try to match `word` as a unit or its ordinal (1–9).
fn match_unit(word: &str) -> Option<i32> {
    best_match(word, UNITS)
}

/// Try to match `word` as a teen or its ordinal (10–19).
fn match_teen(word: &str) -> Option<i32> {
    best_match(word, TEENS)
}

/// Try to match `word` as a tens value or its ordinal (20–90).
fn match_tens(word: &str) -> Option<i32> {
    best_match(word, TENS)
}

/// The number category that a word was matched into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberCategory {
    Unit,
    Teen,
    Tens,
}

/// Return the highest levenshtein ratio for `word` against any entry in
/// `table` that meets [`MATCH_THRESHOLD`], or `0.0` if none qualify.
fn best_ratio(word: &str, table: &[(&str, i32)]) -> f32 {
    if is_stop_word(word) {
        return 0.0;
    }
    table
        .iter()
        .map(|&(canonical, _)| levenshtein_ratio(word, canonical))
        .filter(|&ratio| ratio >= MATCH_THRESHOLD)
        .fold(0.0_f32, f32::max)
}

/// Match `word` against all three categories (units, teens, tens) and return
/// the value from whichever category has the strictly highest best ratio.
///
/// On a tie the preference order is tens > teens > units, so that e.g.
/// `"twenty"` (exact tens match) is never trumped by a unit ordinal that
/// happens to tie.
///
/// This cross-category comparison prevents a word like `"seven"` from being
/// misclassified as `70` because `"seventy"` (tens) scores 0.714 and fires
/// before the exact unit match (1.0) under a fixed-order strategy.
fn match_best_single_word(word: &str) -> Option<(i32, NumberCategory)> {
    let unit_ratio = best_ratio(word, UNITS);
    let teen_ratio = best_ratio(word, TEENS);
    let tens_ratio = best_ratio(word, TENS);

    if unit_ratio == 0.0 && teen_ratio == 0.0 && tens_ratio == 0.0 {
        return None;
    }

    // Strictly highest ratio wins.  Tie-break: tens > teens > units.
    if unit_ratio > teen_ratio && unit_ratio > tens_ratio {
        match_unit(word).map(|value| (value, NumberCategory::Unit))
    } else if teen_ratio > tens_ratio {
        match_teen(word).map(|value| (value, NumberCategory::Teen))
    } else {
        match_tens(word).map(|value| (value, NumberCategory::Tens))
    }
}

/// Try to match `word` as "hundred" (or "hundredth").
fn match_hundred(word: &str) -> bool {
    if is_stop_word(word) {
        return false;
    }
    HUNDREDS
        .iter()
        .any(|&canonical| levenshtein_ratio(word, canonical) >= MATCH_THRESHOLD)
}

/// Try to match `word` as "thousand" (or "thousandth").
fn match_thousand(word: &str) -> bool {
    if is_stop_word(word) {
        return false;
    }
    THOUSANDS
        .iter()
        .any(|&canonical| levenshtein_ratio(word, canonical) >= MATCH_THRESHOLD)
}

// ---------------------------------------------------------------------------
// Tokenisation of an utterance into words
// ---------------------------------------------------------------------------

/// Split an utterance into a sequence of word tokens, treating spaces,
/// hyphens, and whitespace as separators.  Each token carries its byte
/// offset in the original string so we can reconstruct the replacement.
fn word_tokens(utterance: &str) -> Vec<(usize, &str)> {
    let mut tokens: Vec<(usize, &str)> = Vec::new();
    let mut start: Option<usize> = None;

    for (byte_offset, character) in utterance.char_indices() {
        let is_separator = character == ' '
            || character == '-'
            || character == '\t'
            || character == '\n'
            || character == '\r';

        if is_separator {
            if let Some(word_start) = start.take() {
                tokens.push((word_start, &utterance[word_start..byte_offset]));
            }
        } else if start.is_none() {
            start = Some(byte_offset);
        }
    }
    // Flush the last word.
    if let Some(word_start) = start {
        tokens.push((word_start, &utterance[word_start..]));
    }

    tokens
}

// ---------------------------------------------------------------------------
// Greedy number span parser
// ---------------------------------------------------------------------------

/// Attempt to parse a number starting at `tokens[cursor]`.
///
/// Returns `(value, words_consumed)` if a number was parsed, or `None`.
///
/// Grammar (greedy, left-to-right):
/// ```text
/// number ::= thousands? hundreds? tens_units
/// thousands ::= unit "thousand"
/// hundreds  ::= unit "hundred"
/// tens_units ::= (tens unit?) | teen | unit | ε
/// ```
fn try_parse_number(tokens: &[(usize, &str)], cursor: usize) -> Option<(i32, usize)> {
    let lower_word = |index: usize| -> Option<String> {
        tokens.get(index).map(|(_, word)| word.to_ascii_lowercase())
    };

    let mut position = cursor;
    let mut total: i32 = 0;

    // --- Thousands component ------------------------------------------------
    // Pattern: <unit> "thousand"
    if let Some(unit_word) = lower_word(position)
        && let Some(unit_value) = match_unit(&unit_word)
        && let Some(thousand_word) = lower_word(position + 1)
        && match_thousand(&thousand_word)
    {
        total += unit_value * 1000;
        position += 2;
    }

    // --- Hundreds component -------------------------------------------------
    // Pattern: <unit> "hundred"
    if let Some(unit_word) = lower_word(position)
        && let Some(unit_value) = match_unit(&unit_word)
        && let Some(hundred_word) = lower_word(position + 1)
        && match_hundred(&hundred_word)
    {
        total += unit_value * 100;
        position += 2;
    }

    // --- Tens-and-units component -------------------------------------------
    //
    // Use cross-category best-ratio selection so that a word like "seven" is
    // never misclassified as 70 just because "seventy" (tens) clears the
    // threshold before the exact unit match is checked.
    if let Some(word) = lower_word(position) {
        match match_best_single_word(&word) {
            Some((value, NumberCategory::Tens)) => {
                total += value;
                position += 1;
                // A tens word may optionally be followed by a unit
                // (e.g. "twenty" + "one" → 21).
                if let Some(unit_word) = lower_word(position)
                    && let Some((unit_value, _)) = match_best_single_word(&unit_word)
                    // Only accept a Unit or Teen here, not another Tens.
                    && matches!(
                        match_best_single_word(&unit_word),
                        Some((_, NumberCategory::Unit | NumberCategory::Teen))
                    )
                {
                    total += unit_value;
                    position += 1;
                }
            }
            Some((value, NumberCategory::Teen | NumberCategory::Unit)) => {
                total += value;
                position += 1;
            }
            None => {
                // No match at this position — fine if thousands/hundreds
                // already accumulated something.
            }
        }
    }

    let words_consumed = position - cursor;

    // Require at least one word consumed and a positive total.
    if words_consumed == 0 || total <= 0 {
        return None;
    }

    Some((total, words_consumed))
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Scan `utterance` for word-number spans and replace each with its decimal
/// representation, returning the modified string.
///
/// Spans are found greedily left-to-right.  The longest parseable span
/// starting at each position is consumed; overlapping spans are not
/// considered.  Words that do not participate in a recognised number span
/// are left in place unchanged (including month names, noise words, etc.).
///
/// Ordinal forms (`"first"`, `"twenty-third"`, etc.) are treated identically
/// to their cardinal equivalents (`"one"`, `"twenty-three"`).
///
/// # Examples
///
/// ```
/// use partial_date::word_numbers::replace_word_numbers;
///
/// assert_eq!(replace_word_numbers("twenty-three"), "23");
/// assert_eq!(replace_word_numbers("the twenty-third day"), "the 23 day");
/// assert_eq!(replace_word_numbers("two thousand twenty-four"), "2024");
/// assert_eq!(replace_word_numbers("31 December two thousand fourteen"), "31 December 2014");
/// ```
pub fn replace_word_numbers(utterance: &str) -> String {
    let tokens = word_tokens(utterance);

    if tokens.is_empty() {
        return utterance.to_string();
    }

    let mut result = String::with_capacity(utterance.len());
    // Byte offset up to which we have already written into `result`.
    let mut output_up_to: usize = 0;
    let mut token_cursor: usize = 0;

    while token_cursor < tokens.len() {
        match try_parse_number(&tokens, token_cursor) {
            Some((value, words_consumed)) => {
                // Write the original utterance bytes that precede this span
                // (any separators / non-number content between the last output
                // position and the start of the first consumed word).
                let span_start = tokens[token_cursor].0;
                if span_start > output_up_to {
                    result.push_str(&utterance[output_up_to..span_start]);
                }

                // Write the digit string.
                result.push_str(&value.to_string());

                // Advance output_up_to past the last consumed word.
                let last_consumed_index = token_cursor + words_consumed - 1;
                let (last_word_start, last_word) = tokens[last_consumed_index];
                output_up_to = last_word_start + last_word.len();

                token_cursor += words_consumed;
            }
            None => {
                // This word is not part of a number span — advance past it.
                token_cursor += 1;
            }
        }
    }

    // Flush any remaining original bytes after the last replacement.
    if output_up_to < utterance.len() {
        result.push_str(&utterance[output_up_to..]);
    }

    result
}
