//! Levenshtein distance and ratio algorithms.
//!
//! These are implemented from scratch — the library has zero external
//! dependencies and this module must not introduce any.
//!
//! # Algorithms
//!
//! - [`levenshtein_distance`] — the classic edit distance (insertions,
//!   deletions, substitutions each cost 1).
//! - [`levenshtein_ratio`] — a normalised similarity score in `[0.0, 1.0]`
//!   derived from the distance.  Two identical strings score `1.0`; two
//!   completely different strings of maximum length score `0.0`.

// ---------------------------------------------------------------------------
// Distance
// ---------------------------------------------------------------------------

/// Compute the Levenshtein edit distance between two strings.
///
/// The distance counts the minimum number of single-character edits
/// (insertions, deletions, or substitutions) required to transform `a` into
/// `b`.
///
/// The implementation uses two rolling rows of a standard dynamic-programming
/// matrix so memory is O(min(|a|, |b|)) rather than O(|a| · |b|).
///
/// Both strings are treated as sequences of bytes.  All inputs in this library
/// are ASCII (the caller lowercases before calling), so byte-level comparisons
/// are correct and cheaper than char-level ones.
///
/// # Examples
///
/// ```
/// use partial_date::levenshtein::levenshtein_distance;
///
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(levenshtein_distance("", "abc"), 3);
/// assert_eq!(levenshtein_distance("abc", "abc"), 0);
/// assert_eq!(levenshtein_distance("march", "mrach"), 2);
/// ```
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    // Make `a` the shorter string so the inner loop (over `a`) uses less
    // memory and cache pressure.
    let (a, b) = if a.len() <= b.len() {
        (a.as_bytes(), b.as_bytes())
    } else {
        (b.as_bytes(), a.as_bytes())
    };

    let len_a = a.len();
    let len_b = b.len();

    // prev[j] = distance(a[0..i-1], b[0..j])
    // curr[j] = distance(a[0..i],   b[0..j])
    let mut prev: Vec<usize> = (0..=len_b).collect();
    let mut curr: Vec<usize> = vec![0; len_b + 1];

    for i in 1..=len_a {
        curr[0] = i;
        for j in 1..=len_b {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1) // deletion
                .min(curr[j - 1] + 1) // insertion
                .min(prev[j - 1] + cost); // substitution
        }
        prev.clone_from(&curr);
    }

    prev[len_b]
}

// ---------------------------------------------------------------------------
// Ratio
// ---------------------------------------------------------------------------

/// Compute the normalised Levenshtein similarity ratio between two strings.
///
/// The ratio is defined as:
///
/// ```text
/// ratio = 1.0 - distance(a, b) / max(len(a), len(b))
/// ```
///
/// and lies in the range `[0.0, 1.0]`:
///
/// - `1.0` — the strings are identical (or both empty).
/// - `0.0` — the strings are completely different (distance equals the length
///   of the longer string).
///
/// # Examples
///
/// ```
/// use partial_date::levenshtein::levenshtein_ratio;
///
/// assert!((levenshtein_ratio("march",  "march")  - 1.0).abs() < f32::EPSILON);
/// assert!((levenshtein_ratio("mrach",  "march")  - 0.6).abs() < f32::EPSILON);
/// assert!((levenshtein_ratio("january","jauary") - (6.0/7.0)).abs() < f32::EPSILON);
/// ```
pub fn levenshtein_ratio(a: &str, b: &str) -> f32 {
    let max_len = a.len().max(b.len());
    if max_len == 0 {
        return 1.0; // Both strings are empty — they are identical.
    }
    let dist = levenshtein_distance(a, b);
    1.0 - (dist as f32 / max_len as f32)
}
