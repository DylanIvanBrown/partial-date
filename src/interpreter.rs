//! Token interpretation: assigning tokens to date components.
//!
//! # Algorithm
//!
//! Tokens split into two groups before any assignment logic runs:
//!
//! - **Unambiguous anchors** — `OrdinalDay` is always a day; `MonthName` is
//!   always a month. They are consumed first and never enter positional
//!   assignment.
//!
//! - **Numeric tokens** — go through three steps:
//!
//!   1. **Generate viable assignments** — enumerate every way of assigning the
//!      numeric tokens to the remaining open slots. A permutation is *viable*
//!      when every token passes the candidate check for its assigned component
//!      (delegated to [`DayConfig::try_as_day_candidate`],
//!      [`MonthConfig::try_as_month_candidate`], and
//!      [`YearConfig::try_as_year_candidate`]).
//!
//!   2. **Score by component count** — prefer assignments that fill more slots.
//!
//!   3. **Score by unambiguity then config agreement**:
//!      - A token is *unambiguous* when it is valid for exactly one of the open
//!        slots. Unambiguous tokens must go to their forced slot regardless of
//!        the configured order — the score counts how many tokens are placed in
//!        their only valid slot.
//!      - Once unambiguity is equal, count how many token positions agree with
//!        `component_order`. The configured order is the tiebreaker for tokens
//!        that could validly be more than one component.
//!      - Any remaining tie is broken deterministically by positional agreement.

use crate::models::{Config, DateComponent, IsExpected, MonthName, Token};

/// The raw candidates extracted from the token list, before validation.
///
/// - `day_raw`: `(value, digit_count)` if a day candidate was found.
/// - `month_raw`: `(number, Option<MonthName>)` if a month candidate was found.
/// - `year_raw`: expanded year value if a year candidate was found.
pub type RawDay = Option<(u8, u8)>;
pub type RawMonth = Option<(u8, Option<MonthName>)>;
pub type RawYear = Option<i32>;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Interpret up to 3 tokens as day, month, and year candidates.
pub fn interpret_tokens(tokens: &[Token], config: &Config) -> (RawDay, RawMonth, RawYear) {
    let day_expected = config.day.expected != IsExpected::No;
    let month_expected = config.month.expected != IsExpected::No;
    let year_expected = config.year.expected != IsExpected::No;

    // --- Unambiguous anchors ------------------------------------------------

    // OrdinalDay tokens are unambiguously a day ("19th", "3rd").
    let ordinal_day: Option<u8> = tokens.iter().find_map(|t| match t {
        Token::OrdinalDay(n) => Some(*n),
        _ => None,
    });

    // MonthName tokens are unambiguously a month ("October", "jan").
    let named_month: Option<MonthName> = tokens.iter().find_map(|t| match t {
        Token::MonthName(m) => Some(*m),
        _ => None,
    });

    // Collect all numeric tokens. These are the ones that need assignment.
    let numerics: Vec<(i16, u8)> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::Numeric(value, digit_count) => Some((*value, *digit_count)),
            _ => None,
        })
        .collect();

    // --- Determine which slots still need filling ---------------------------
    //
    // Open slots are listed in config-order so that the agreement scoring step
    // can compare token positions directly against the configured ordering.

    let day_anchor: Option<u8> = ordinal_day;
    let month_anchor: Option<(u8, Option<MonthName>)> = named_month.map(|m| (m.number(), Some(m)));

    let open_slots: Vec<DateComponent> = [
        config.component_order.first,
        config.component_order.second,
        config.component_order.third,
    ]
    .iter()
    .filter(|&&component| match component {
        DateComponent::Day => day_anchor.is_none() && day_expected,
        DateComponent::Month => month_anchor.is_none() && month_expected,
        DateComponent::Year => year_expected,
    })
    .copied()
    .collect();

    let (numeric_day, numeric_month, numeric_year) =
        assign_numerics(&numerics, &open_slots, config);

    let raw_day = day_anchor.map(|v| (v, 1u8)).or(numeric_day);
    let raw_month = month_anchor.or(numeric_month);
    let raw_year = numeric_year;

    (
        if day_expected { raw_day } else { None },
        if month_expected { raw_month } else { None },
        if year_expected { raw_year } else { None },
    )
}

// ---------------------------------------------------------------------------
// Assignment struct
// ---------------------------------------------------------------------------

/// A concrete assignment of numeric tokens to date component slots, stored as
/// indices into the original `numerics` slice.
///
/// Using indices rather than values avoids ambiguity when two tokens carry the
/// same `(value, digit_count)` pair — e.g. `"01/01"` produces two identical
/// tokens; tracking by index keeps them distinct.
#[derive(Clone, Debug)]
struct Assignment {
    /// Index of the token in `numerics` assigned to the day slot, if any.
    day_index: Option<usize>,
    /// Index of the token in `numerics` assigned to the month slot, if any.
    month_index: Option<usize>,
    /// Index of the token in `numerics` assigned to the year slot, if any.
    year_index: Option<usize>,
}

impl Assignment {
    fn component_count(&self) -> usize {
        self.day_index.is_some() as usize
            + self.month_index.is_some() as usize
            + self.year_index.is_some() as usize
    }

    /// The component (if any) that the token at `token_index` was assigned to.
    fn component_for_token(&self, token_index: usize) -> Option<DateComponent> {
        if self.day_index == Some(token_index) {
            Some(DateComponent::Day)
        } else if self.month_index == Some(token_index) {
            Some(DateComponent::Month)
        } else if self.year_index == Some(token_index) {
            Some(DateComponent::Year)
        } else {
            None
        }
    }

    /// Retrieve the `(value, digit_count)` of the token assigned to
    /// `component`, or `None` if that slot is unfilled.
    fn token_for(&self, component: DateComponent, numerics: &[(i16, u8)]) -> Option<(i16, u8)> {
        let index = match component {
            DateComponent::Day => self.day_index?,
            DateComponent::Month => self.month_index?,
            DateComponent::Year => self.year_index?,
        };
        numerics.get(index).copied()
    }
}

// ---------------------------------------------------------------------------
// Core assignment logic
// ---------------------------------------------------------------------------

/// Assign `numerics` to `open_slots`, returning raw day/month/year candidates.
fn assign_numerics(
    numerics: &[(i16, u8)],
    open_slots: &[DateComponent],
    config: &Config,
) -> (RawDay, RawMonth, RawYear) {
    if numerics.is_empty() || open_slots.is_empty() {
        return (None, None, None);
    }

    // Fast path: attempt the config-order assignment directly. When the number
    // of tokens exactly matches the number of open slots and every token is
    // valid for the slot that `component_order` prescribes at its position,
    // this is guaranteed to be the best possible assignment — it has the
    // maximum component count and perfect agreement with the configured order.
    // Returning immediately avoids the full combinatorial enumeration for the
    // common case of well-formed input.
    if numerics.len() == open_slots.len()
        && let Some(direct) = try_config_order_assignment(numerics, open_slots, config)
    {
        return convert_assignment(&direct, numerics, config);
    }

    let viable = generate_viable_assignments(numerics, open_slots, config);

    if viable.is_empty() {
        return (None, None, None);
    }

    let best = pick_best_assignment(viable, numerics, config);

    convert_assignment(&best, numerics, config)
}

/// Attempt to assign `numerics` directly in `open_slots` order, one token per
/// slot, without any permutation. Returns `Some(Assignment)` only when every
/// token is valid for the slot at its corresponding position — i.e. when the
/// configured component order is fully satisfied by the input as-is.
///
/// This is the fast path for well-formed input such as `"21/02/2005"` with DMY.
fn try_config_order_assignment(
    numerics: &[(i16, u8)],
    open_slots: &[DateComponent],
    config: &Config,
) -> Option<Assignment> {
    let mut assignment = Assignment {
        day_index: None,
        month_index: None,
        year_index: None,
    };

    for (token_index, (&slot, &(value, digit_count))) in
        open_slots.iter().zip(numerics.iter()).enumerate()
    {
        let valid = match slot {
            DateComponent::Day => config
                .day
                .try_as_day_candidate(value, digit_count)
                .is_some(),
            DateComponent::Month => config
                .month
                .try_as_month_candidate(value, digit_count)
                .is_some(),
            DateComponent::Year => config
                .year
                .try_as_year_candidate(value, digit_count)
                .is_some(),
        };

        if !valid {
            return None;
        }

        match slot {
            DateComponent::Day => assignment.day_index = Some(token_index),
            DateComponent::Month => assignment.month_index = Some(token_index),
            DateComponent::Year => assignment.year_index = Some(token_index),
        }
    }

    Some(assignment)
}

/// Convert a winning [`Assignment`] into the three raw output types, using
/// the config candidate methods for all validation.
fn convert_assignment(
    assignment: &Assignment,
    numerics: &[(i16, u8)],
    config: &Config,
) -> (RawDay, RawMonth, RawYear) {
    let raw_day =
        assignment
            .token_for(DateComponent::Day, numerics)
            .and_then(|(value, digit_count)| {
                config
                    .day
                    .try_as_day_candidate(value, digit_count)
                    .map(|v| (v, digit_count))
            });

    let raw_month = assignment
        .token_for(DateComponent::Month, numerics)
        .and_then(|(value, digit_count)| {
            config
                .month
                .try_as_month_candidate(value, digit_count)
                .map(|number| {
                    let name = MonthName::try_from(number).ok();
                    (number, name)
                })
        });

    let raw_year = assignment
        .token_for(DateComponent::Year, numerics)
        .and_then(|(value, digit_count)| config.year.try_as_year_candidate(value, digit_count));

    (raw_day, raw_month, raw_year)
}

/// Enumerate every viable assignment of `numerics` tokens to `open_slots`.
///
/// A permutation is viable when every token passes the candidate check for its
/// assigned slot. We try all counts from the maximum possible down to 1,
/// stopping as soon as any viable assignment is found — the scoring step then
/// selects the best among them. This ensures that when no full assignment is
/// viable (e.g. two tokens are both forced to the same slot), partial
/// assignments are still considered.
fn generate_viable_assignments(
    numerics: &[(i16, u8)],
    open_slots: &[DateComponent],
    config: &Config,
) -> Vec<Assignment> {
    let max_token_count = numerics.len().min(open_slots.len());
    let mut viable: Vec<Assignment> = Vec::new();

    // Try from the most tokens down to 1. Stop at the first count that yields
    // at least one viable assignment — there is no point looking at smaller
    // counts if we already have valid candidates.
    for token_count in (1..=max_token_count).rev() {
        // Choose which `token_count` tokens to use (subset of all numerics).
        for token_indices in combinations(numerics.len(), token_count) {
            // Choose which `token_count` slots to fill (subset of open_slots).
            for slot_indices in combinations(open_slots.len(), token_count) {
                let chosen_slots: Vec<DateComponent> =
                    slot_indices.iter().map(|&i| open_slots[i]).collect();

                // Try every ordering of the chosen tokens into the chosen slots.
                for token_permutation in permutations(token_count) {
                    let mut assignment = Assignment {
                        day_index: None,
                        month_index: None,
                        year_index: None,
                    };
                    let mut all_valid = true;

                    for (slot_position, &perm_index) in token_permutation.iter().enumerate() {
                        // Map permutation index → actual token index in `numerics`.
                        let token_index = token_indices[perm_index];
                        let (value, digit_count) = numerics[token_index];
                        let slot = chosen_slots[slot_position];

                        // Validity is delegated entirely to the config methods —
                        // no range literals here.
                        let valid = match slot {
                            DateComponent::Day => config
                                .day
                                .try_as_day_candidate(value, digit_count)
                                .is_some(),
                            DateComponent::Month => config
                                .month
                                .try_as_month_candidate(value, digit_count)
                                .is_some(),
                            DateComponent::Year => config
                                .year
                                .try_as_year_candidate(value, digit_count)
                                .is_some(),
                        };

                        if !valid {
                            all_valid = false;
                            break;
                        }

                        match slot {
                            DateComponent::Day => assignment.day_index = Some(token_index),
                            DateComponent::Month => assignment.month_index = Some(token_index),
                            DateComponent::Year => assignment.year_index = Some(token_index),
                        }
                    }

                    if all_valid {
                        viable.push(assignment);
                    }
                }
            }
        }

        // Stop descending once we have found at least one viable assignment.
        // Smaller token counts would only produce results with fewer components,
        // which the scoring step would rank below what we already have.
        if !viable.is_empty() {
            break;
        }
    }

    viable
}

/// Choose the best assignment from a non-empty list of viable candidates.
///
/// Scoring priority (all maximised unless noted, applied in order):
///
/// 1. **Component count** — more filled slots is better.
///
/// 2. **Positional validity score** — count how many assigned tokens are valid
///    for their prescribed slot according to `component_order`.  A token at
///    position 0 (prescribed = Day in DMY) that is invalid as a day (e.g.
///    value 25 in MDY where position 0 = Month) scores 0 for that token.
///    Prefer assignments that do not "borrow" a token from its prescribed slot
///    to fill a different one, displacing correctly-positioned tokens.
///
/// 3. **Unambiguity score** — count how many tokens in this assignment are
///    placed in the *only* slot they can validly fill across all open slots.
///    A token that could be Day or Month is ambiguous; a token that can only be
///    Day (value > 12) or only be Year (4-digit) is unambiguous. Unambiguous
///    placements must be respected before the configured order is consulted.
///
/// 3. **Token exclusivity score** — for each assigned token, count how many of
///    the open slots it can validly fill. Sum those counts across all assigned
///    tokens; *lower is better* (a token valid for only 1 slot is more
///    exclusive than one valid for 3). This is sorted ascending.
///
///    This handles cases like `"2024 2023 45"` with DMY: both 2024 and 2023
///    are valid only for Year (exclusivity = 1 each), while 45 is valid for
///    Year only via two-digit expansion (also 1). Among assignments that each
///    fill just the Year slot, the one using 2024 or 2023 (4-digit, inherently
///    unambiguous) beats the one using 45 (2-digit, could in principle have
///    been a day). When two tokens tie on exclusivity, the earlier one in the
///    input wins via the tiebreaker below.
///
/// 4. **Agreement score** — for the ambiguous tokens, count how many are placed
///    in the slot that `component_order` prescribes for their input position.
///    This is the primary mechanism for resolving genuinely ambiguous inputs.
///
/// 5. **Earliest-token tiebreaker** — when all scores tie, prefer the
///    assignment whose assigned token(s) appear earliest in the input stream.
///    This gives `"2024 2023 45"` → year=2024 rather than 2023 when both are
///    equally exclusive 4-digit years.
fn pick_best_assignment(
    mut viable: Vec<Assignment>,
    numerics: &[(i16, u8)],
    config: &Config,
) -> Assignment {
    let order = [
        config.component_order.first,
        config.component_order.second,
        config.component_order.third,
    ];

    // Helper: count how many open slots a given token (value, digit_count) can
    // validly fill. Used for the token exclusivity score.
    let slot_validity_count = |(value, digit_count): (i16, u8)| -> usize {
        let can_be_day = config
            .day
            .try_as_day_candidate(value, digit_count)
            .is_some();
        let can_be_month = config
            .month
            .try_as_month_candidate(value, digit_count)
            .is_some();
        let can_be_year = config
            .year
            .try_as_year_candidate(value, digit_count)
            .is_some();
        can_be_day as usize + can_be_month as usize + can_be_year as usize
    };

    // Helper: whether token at position `token_index` is valid for its
    // prescribed component in the configured order.
    let token_valid_for_prescribed = |token_index: usize| -> bool {
        let (value, digit_count) = match numerics.get(token_index) {
            Some(&t) => t,
            None => return false,
        };
        let prescribed = match order.get(token_index) {
            Some(&c) => c,
            None => return true, // no prescription for extra tokens — not penalised
        };
        match prescribed {
            DateComponent::Day => config
                .day
                .try_as_day_candidate(value, digit_count)
                .is_some(),
            DateComponent::Month => config
                .month
                .try_as_month_candidate(value, digit_count)
                .is_some(),
            DateComponent::Year => config
                .year
                .try_as_year_candidate(value, digit_count)
                .is_some(),
        }
    };

    // scored tuple: (component_count, positional_validity, unambiguity,
    //               exclusivity_sum, agreement, assignment)
    // exclusivity_sum is sorted ascending (lower = more exclusive tokens used).
    let mut scored: Vec<(usize, usize, usize, usize, usize, Assignment)> = viable
        .drain(..)
        .map(|assignment| {
            let component_count = assignment.component_count();

            // Positional validity score: count how many assigned tokens are
            // valid for their prescribed component (or have no prescription).
            let positional_validity_score: usize = [
                assignment.day_index,
                assignment.month_index,
                assignment.year_index,
            ]
            .iter()
            .filter_map(|&index| index)
            .filter(|&token_index| token_valid_for_prescribed(token_index))
            .count();

            // Unambiguity score: count slots that are filled by the *only*
            // token that can validly occupy them.
            //
            // For each filled slot in this assignment, check whether any other
            // token (not the one assigned here) could also fill that slot. If
            // no other token could, the slot is "singly-valid" and the
            // assignment that fills it correctly is more constrained.
            //
            // This correctly handles "31/06" with MDY: only token 1 (6) can
            // fill the Month slot (token 0 = 31 cannot be a month). So the
            // assignment placing 6 as Month scores 1 here, while assignments
            // placing 31 or nothing as Month score 0. This ranks the correct
            // assignment above alternatives that would place 6 as Day or Year.
            let unambiguity_score: usize = [
                (DateComponent::Day, assignment.day_index),
                (DateComponent::Month, assignment.month_index),
                (DateComponent::Year, assignment.year_index),
            ]
            .iter()
            .filter(|(slot, assigned_index)| {
                let Some(assigned_token_index) = assigned_index else {
                    return false;
                };
                // Count how many tokens are valid for this slot.
                let valid_token_count = numerics
                    .iter()
                    .enumerate()
                    .filter(|(token_index, token)| {
                        let (value, digit_count) = **token;
                        // Only count tokens that are actually assigned to some
                        // slot (unassigned tokens don't constrain the slot).
                        let is_assigned = assignment.component_for_token(*token_index).is_some();
                        if !is_assigned {
                            return false;
                        }
                        match slot {
                            DateComponent::Day => config
                                .day
                                .try_as_day_candidate(value, digit_count)
                                .is_some(),
                            DateComponent::Month => config
                                .month
                                .try_as_month_candidate(value, digit_count)
                                .is_some(),
                            DateComponent::Year => config
                                .year
                                .try_as_year_candidate(value, digit_count)
                                .is_some(),
                        }
                    })
                    .count();
                // The slot is singly-valid when exactly one assigned token can
                // fill it, and the assignment correctly places that token here.
                valid_token_count == 1
                    && assignment.component_for_token(*assigned_token_index) == Some(*slot)
            })
            .count();

            // Token exclusivity score: sum of how many open slots each assigned
            // token can validly fill. Lower is better — a token valid for only
            // 1 slot is more constrained than one valid for 3.
            let exclusivity_sum: usize = [
                assignment.day_index,
                assignment.month_index,
                assignment.year_index,
            ]
            .iter()
            .filter_map(|&index| index)
            .map(|token_index| slot_validity_count(numerics[token_index]))
            .sum();

            // Agreement score: count how many tokens at input position `i` are
            // assigned to `order[i]`. Resolves ambiguity for tokens that could
            // validly fill more than one slot.
            let agreement_score: usize = numerics
                .iter()
                .enumerate()
                .filter(|(index, _)| {
                    let prescribed = match order.get(*index) {
                        Some(c) => *c,
                        None => return false,
                    };
                    assignment.component_for_token(*index) == Some(prescribed)
                })
                .count();

            (
                component_count,
                positional_validity_score,
                unambiguity_score,
                exclusivity_sum,
                agreement_score,
                assignment,
            )
        })
        .collect();

    scored.sort_by(|a, b| {
        b.0.cmp(&a.0) // more components first
            .then(b.1.cmp(&a.1)) // more positionally-valid tokens first
            .then(b.2.cmp(&a.2)) // more unambiguous placements first
            .then(a.3.cmp(&b.3)) // lower exclusivity sum first (more exclusive tokens)
            .then(b.4.cmp(&a.4)) // higher agreement with config order first
            .then_with(|| {
                // Earliest-token tiebreaker: prefer the assignment whose tokens
                // appear earliest in the input stream. This is equivalent to
                // preferring the smallest sum of assigned token indices.
                let earliest_token_index = |assignment: &Assignment| -> usize {
                    [
                        assignment.day_index,
                        assignment.month_index,
                        assignment.year_index,
                    ]
                    .iter()
                    .filter_map(|&i| i)
                    .sum()
                };
                let a_earliest = earliest_token_index(&a.5);
                let b_earliest = earliest_token_index(&b.5);
                a_earliest.cmp(&b_earliest) // smaller index sum = earlier in input
            })
    });

    let best = scored.remove(0);
    best.5
}

// ---------------------------------------------------------------------------
// Combinatorics helpers
// ---------------------------------------------------------------------------

/// Return all combinations of `choose` indices from `0..total`.
fn combinations(total: usize, choose: usize) -> Vec<Vec<usize>> {
    if choose == 0 {
        return vec![vec![]];
    }
    if choose > total {
        return vec![];
    }
    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut indices: Vec<usize> = (0..choose).collect();
    loop {
        result.push(indices.clone());
        let mut i = choose;
        loop {
            if i == 0 {
                return result;
            }
            i -= 1;
            if indices[i] < total - choose + i {
                break;
            }
        }
        indices[i] += 1;
        for j in i + 1..choose {
            indices[j] = indices[j - 1] + 1;
        }
    }
}

/// Return all permutations of `0..n` as index vectors.
fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut indices: Vec<usize> = (0..n).collect();
    result.push(indices.clone());
    loop {
        // Knuth's algorithm L: next permutation in lexicographic order.
        let mut i = n - 1;
        while i > 0 && indices[i - 1] >= indices[i] {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        let pivot = i - 1;
        let mut j = n - 1;
        while indices[j] <= indices[pivot] {
            j -= 1;
        }
        indices.swap(pivot, j);
        indices[i..].reverse();
        result.push(indices.clone());
    }
    result
}
