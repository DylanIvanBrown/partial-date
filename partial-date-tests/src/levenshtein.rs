// Tests for the levenshtein_distance and levenshtein_ratio functions.

use partial_date::levenshtein::{levenshtein_distance, levenshtein_ratio};
use rstest::rstest;

// -------------------------------------------------------------------------
// levenshtein_distance — classical cases
// -------------------------------------------------------------------------

/// The canonical "kitten" / "sitting" example used in most textbooks.
#[test]
fn distance_kitten_sitting() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
}

/// Identical strings have distance 0.
#[rstest]
#[case("", "")]
#[case("a", "a")]
#[case("march", "march")]
#[case("september", "september")]
fn distance_identical(#[case] a: &str, #[case] b: &str) {
    assert_eq!(levenshtein_distance(a, b), 0);
}

/// Distance to / from an empty string equals the other string's length.
#[rstest]
#[case("abc", 3)]
#[case("january", 7)]
#[case("x", 1)]
fn distance_from_empty(#[case] s: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance("", s), expected);
    assert_eq!(levenshtein_distance(s, ""), expected);
}

/// Distance is symmetric.
#[rstest]
#[case("abc", "bc")]
#[case("sunday", "saturday")]
#[case("march", "mrach")]
fn distance_is_symmetric(#[case] a: &str, #[case] b: &str) {
    assert_eq!(levenshtein_distance(a, b), levenshtein_distance(b, a));
}

/// A single insertion costs 1.
#[rstest]
#[case("cat", "cats", 1)]
#[case("june", "juune", 1)]
#[case("aug", "august", 3)]
fn distance_insertion(#[case] a: &str, #[case] b: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance(a, b), expected);
}

/// A single deletion costs 1.
#[rstest]
#[case("cats", "cat", 1)]
#[case("jauary", "january", 1)]
fn distance_deletion(#[case] a: &str, #[case] b: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance(a, b), expected);
}

/// A single substitution costs 1.
#[rstest]
#[case("marsh", "march", 1)]
#[case("juli", "july", 1)]
fn distance_substitution(#[case] a: &str, #[case] b: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance(a, b), expected);
}

/// Adjacent transpositions cost 2 (two substitutions), not 1.
///
/// Note: this is plain Levenshtein, not Damerau-Levenshtein. A transposition
/// such as "mrach" vs "march" is counted as 2 edits (substitute r→a, a→r).
#[rstest]
#[case("mrach", "march", 2)]
#[case("apirl", "april", 2)]
#[case("ocotber", "october", 2)]
#[case("setpember", "september", 2)]
fn distance_transposition_costs_two(#[case] a: &str, #[case] b: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance(a, b), expected);
}

/// Larger known distances.
#[rstest]
#[case("sunday", "saturday", 3)]
#[case("xyzember", "december", 3)]
#[case("foo", "may", 3)]
fn distance_larger(#[case] a: &str, #[case] b: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance(a, b), expected);
}

// -------------------------------------------------------------------------
// levenshtein_ratio
// -------------------------------------------------------------------------

/// Identical strings have ratio 1.0.
#[rstest]
#[case("", "")]
#[case("march", "march")]
#[case("october", "october")]
fn ratio_identical(#[case] a: &str, #[case] b: &str) {
    assert!((levenshtein_ratio(a, b) - 1.0).abs() < f32::EPSILON);
}

/// Ratio is symmetric.
#[rstest]
#[case("abc", "bc")]
#[case("march", "mrach")]
#[case("july", "juli")]
fn ratio_is_symmetric(#[case] a: &str, #[case] b: &str) {
    let diff = (levenshtein_ratio(a, b) - levenshtein_ratio(b, a)).abs();
    assert!(diff < f32::EPSILON);
}

/// ratio = 1 - distance / max_len for specific pairs.
#[rstest]
// distance 2, max_len 5 → 1 - 2/5 = 0.6
#[case("mrach", "march", 0.6)]
#[case("apirl", "april", 0.6)]
// distance 1, max_len 7 → 1 - 1/7 ≈ 0.857
#[case("jauary",    "january",   6.0 / 7.0)]
// distance 1, max_len 5 → 0.8
#[case("marsh", "march", 0.8)]
#[case("juli", "july", 0.75)]
// distance 3, max_len 8 → 1 - 3/8 = 0.625
#[case("xyzember",  "december",  1.0 - 3.0 / 8.0)]
fn ratio_specific_pairs(#[case] a: &str, #[case] b: &str, #[case] expected: f32) {
    let diff = (levenshtein_ratio(a, b) - expected).abs();
    assert!(
        diff < 1e-5,
        "ratio({a:?}, {b:?}) = {}, expected {expected}",
        levenshtein_ratio(a, b)
    );
}

/// Ratio lies in [0.0, 1.0] for all inputs.
#[rstest]
#[case("", "abc")]
#[case("foo", "bar")]
#[case("kitten", "sitting")]
#[case("january", "xyzember")]
fn ratio_in_range(#[case] a: &str, #[case] b: &str) {
    let r = levenshtein_ratio(a, b);
    assert!(r >= 0.0 && r <= 1.0, "ratio out of range: {r}");
}

/// Misspellings that should exceed the 0.6 threshold all score ≥ 0.6.
#[rstest]
#[case("jauary", "january")]
#[case("febuary", "february")]
#[case("marsh", "march")]
#[case("apriil", "april")]
#[case("mey", "may")]
#[case("juune", "june")]
#[case("juli", "july")]
#[case("agust", "august")]
#[case("septeber", "september")]
#[case("ocober", "october")]
#[case("novmber", "november")]
#[case("decemer", "december")]
fn ratio_misspellings_above_threshold(#[case] a: &str, #[case] b: &str) {
    let r = levenshtein_ratio(a, b);
    assert!(r >= 0.6, "ratio({a:?}, {b:?}) = {r} — expected ≥ 0.6");
}

/// Noise words that should be rejected all score below 0.6 against every month.
#[rstest]
#[case("xyz")]
#[case("foo")]
#[case("friday")]
#[case("no")]
fn ratio_noise_below_threshold_vs_all_months(#[case] noise: &str) {
    const MONTHS: &[&str] = &[
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    for month in MONTHS {
        let r = levenshtein_ratio(noise, month);
        assert!(
            r < 0.6,
            "ratio({noise:?}, {month:?}) = {r} — expected < 0.6 (noise should not match)"
        );
    }
}
