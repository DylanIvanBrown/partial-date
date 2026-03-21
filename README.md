# partial-date

Deterministic partial date extraction from natural language text.

Unlike full-date parsers, `partial-date` is designed for inputs where only *some* of the date is present. A string like `"June 2024"` or `"the 15th"` or `"22-03-16"` will each yield whatever components could be determined, with the rest marked `NotFound`. Missing components can optionally be filled with caller-supplied defaults.

```toml
[dependencies]
partial-date = "0.1.0"
```

---

## Quick start

```rust
use partial_date::extract::extract;
use partial_date::models::{Extracted, Input};

let input = Input {
    utterance: "19 October 2014".to_string(),
    config: None, // use the default config
};

let result = extract(input);

assert_eq!(result.day.value,   Extracted::Found(19));
assert_eq!(result.month.number, Extracted::Found(10));
assert_eq!(result.year.value,  Extracted::Found(2014));
```

No config is required. `Config::default()` is applied automatically when `config` is `None`.

---

## What it can extract

Given any free-text string, the library attempts to pull out up to three components:

| Component | Type | Example inputs |
|-----------|------|----------------|
| Day | `u8` (1–31) | `"15"`, `"15th"`, `"3rd"` |
| Month | `u8` (1–12) + `MonthName` | `"06"`, `"June"`, `"jun"`, `"Jnuary"` |
| Year | `i32` (0–3000) | `"2024"`, `"24"`, `"99"` |

Each extracted component is wrapped in `Extracted<T>`:

```rust
pub enum Extracted<T> {
    Found(T),       // value was present in the input
    Defaulted(T),   // value was absent but a default was configured
    NotFound,       // value was absent and no default was configured
}
```

---

## Separators

The following characters are recognised as separators automatically — you do not need to configure them:

```
/   -   .   ,   \   space   tab   newline   carriage return
```

So all of these parse identically with default config:

```rust
"22/03/2016"
"22-03-2016"
"22.03.2016"
"22 03 2016"
```

---

## The default config

When no config is provided (`config: None`), the following defaults apply:

```rust
Config {
    // Day extraction
    day: DayConfig {
        min: 1,
        max: 31,
        expected: IsExpected::Maybe,
        default: None,
    },

    // Month extraction
    month: MonthConfig {
        min: 1,
        max: 12,
        expected: IsExpected::Maybe,
        default: None,
    },

    // Year extraction
    year: YearConfig {
        min: 0,
        max: 3000,
        expected: IsExpected::Maybe,
        default: None,
        two_digit_expansion: TwoDigitYearExpansion::SlidingWindow(
            // 00–49 → 2000–2049
            // 50–99 → 1950–1999
            WindowRange::default()
        ),
    },

    // Component order for positional (numeric) input
    component_order: ComponentOrder {
        first:  DateComponent::Day,
        second: DateComponent::Month,
        third:  DateComponent::Year,
    },

    // Whether to attempt parsing concatenated strings like "25122024"
    no_separator: false,

    // No extra separators beyond the standard set
    extra_separators: vec![],
}
```

The most important default to be aware of is `component_order`. With `Day → Month → Year` (the European convention), `"01/06/24"` is read as the 1st of June 2024, not January 6th. If your input uses month-first ordering (US convention), set `component_order` to `Month → Day → Year` — see the [component order](#component-order) section below.

---

## Config reference

### Component order

Controls how three ambiguous numeric tokens are assigned when there is no named month to anchor them. All six orderings are supported.

```rust
use partial_date::models::{Config, ComponentOrder, DateComponent};

// Month → Day → Year (US convention)
let config = Config {
    component_order: ComponentOrder {
        first:  DateComponent::Month,
        second: DateComponent::Day,
        third:  DateComponent::Year,
    },
    ..Default::default()
};
```

Available orderings:

| Name | first | second | third | Example |
|------|-------|--------|-------|---------|
| DMY (default) | Day | Month | Year | `01/06/2024` = 1 June 2024 |
| MDY | Month | Day | Year | `06/01/2024` = 1 June 2024 |
| YMD | Year | Month | Day | `2024/06/01` = 1 June 2024 |
| YDM | Year | Day | Month | `2024/01/06` = 1 June 2024 |
| MYD | Month | Year | Day | `06/2024/01` = 1 June 2024 |
| DYM | Day | Year | Month | `01/2024/06` = 1 June 2024 |

When the input contains an **unambiguous** token — a value greater than 31 can only be a year, a value greater than 12 can only be a day — the library will override the positional assignment automatically. Component order is the tiebreaker for genuinely ambiguous values.

### Controlling which components are extracted

`IsExpected` tells the library whether a component is expected in this input. Setting a component to `IsExpected::No` suppresses it entirely, which also removes it from disambiguation.

```rust
use partial_date::models::{Config, DayConfig, MonthConfig, YearConfig, IsExpected};

// Extract only the year
let config = Config {
    day: DayConfig {
        expected: IsExpected::No,
        ..Default::default()
    },
    month: MonthConfig {
        expected: IsExpected::No,
        ..Default::default()
    },
    year: YearConfig {
        expected: IsExpected::Yes,
        ..Default::default()
    },
    ..Default::default()
};
```

The three variants:

| Variant | Meaning |
|---------|---------|
| `IsExpected::Maybe` | No strong expectation — the component will be extracted if the evidence is clear (default) |
| `IsExpected::Yes` | The component is definitely expected — helps resolve ambiguous tokens |
| `IsExpected::No` | The component should not be extracted — suppressed even if a matching value is present |

### Validation ranges

Each component has a `min` and `max` that act as a post-extraction filter. Values outside the range are discarded as `NotFound`.

```rust
use partial_date::models::{Config, DayConfig};

// Only accept days between 1 and 28 (useful for February-only contexts)
let config = Config {
    day: DayConfig {
        min: 1,
        max: 28,
        ..Default::default()
    },
    ..Default::default()
};
```

### Default values

If extraction finds nothing for a component, a fallback value can be returned as `Extracted::Defaulted(v)` rather than `Extracted::NotFound`.

```rust
use partial_date::models::{Config, YearConfig};

// Default to the year 2024 when no year is present in the input
let config = Config {
    year: YearConfig {
        default: Some(2024),
        ..Default::default()
    },
    ..Default::default()
};
```

### Two-digit year expansion

When a two-digit year is found (e.g. `"24"`), three strategies are available:

```rust
use partial_date::models::{
    Config, YearConfig, TwoDigitYearExpansion, WindowRange, Range,
};

// Strategy 1: Sliding window (default)
// 00–49 → 2000–2049, 50–99 → 1950–1999
let config = Config {
    year: YearConfig {
        two_digit_expansion: TwoDigitYearExpansion::SlidingWindow(
            WindowRange::default()
        ),
        ..Default::default()
    },
    ..Default::default()
};

// Strategy 2: Always treat as 2000s
// 00–99 → 2000–2099
let config = Config {
    year: YearConfig {
        two_digit_expansion: TwoDigitYearExpansion::Always2000s,
        ..Default::default()
    },
    ..Default::default()
};

// Strategy 3: Literal — return the value as-is
// "24" → 24
let config = Config {
    year: YearConfig {
        two_digit_expansion: TwoDigitYearExpansion::Literal,
        ..Default::default()
    },
    ..Default::default()
};

// Custom sliding window: 00–69 → 2000–2069, 70–99 → 1970–1999
let window = WindowRange::new(
    Range { min: 2000, max: 2070 },
    Range { min: 1970, max: 2000 },
).unwrap();

let config = Config {
    year: YearConfig {
        two_digit_expansion: TwoDigitYearExpansion::SlidingWindow(window),
        ..Default::default()
    },
    ..Default::default()
};
```

### No-separator mode

By default the library splits on separator characters. When `no_separator` is `true`, it additionally attempts to parse a fully concatenated digit string of length 6 or 8 by slicing it positionally according to `component_order`.

```rust
use partial_date::models::{Config, Extracted, Input};
use partial_date::extract::extract;

let config = Config {
    no_separator: true,
    ..Default::default()
};

// "25122024" → DD MM YYYY (DMY default order)
let result = extract(Input {
    utterance: "25122024".to_string(),
    config: Some(config),
});

assert_eq!(result.day.value,    Extracted::Found(25));
assert_eq!(result.month.number, Extracted::Found(12));
assert_eq!(result.year.value,   Extracted::Found(2024));
```

Supported lengths:

| Length | Interpretation (DMY example) |
|--------|------------------------------|
| 6 | `DD MM YY` (two-digit year, expanded by `two_digit_expansion`) |
| 8 | `DD MM YYYY` (four-digit year) |

### Extra separators

Any string not in the standard separator set can be added via `extra_separators`. Both single characters and multi-character strings are supported.

```rust
use partial_date::models::Config;

let config = Config {
    extra_separators: vec![
        "|".to_string(),
        " - ".to_string(),
        "::".to_string(),
    ],
    ..Default::default()
};
```

---

## Month name matching

When a token is alphabetic, the library attempts to recognise it as a month name using three strategies in order:

1. **Exact match** — full names (`january`, `february`, …) and standard three-letter abbreviations (`jan`, `feb`, …) are matched case-insensitively.

2. **Prefix match** — any unambiguous prefix of four or more characters is accepted. `"Octo"` → October, `"Septem"` → September. Prefixes that match more than one month (e.g. `"ju"` matches June and July) fall through to fuzzy matching.

3. **Fuzzy match** — the library computes the Levenshtein similarity ratio between the token and every full month name. The closest match is accepted if its ratio is ≥ 0.6 and no other month ties it.

```
"Januray"  → January   (transposed letters, ratio > 0.6)
"Feburary" → February  (omitted r, ratio > 0.6)
"Marsh"    → March     (s/c substitution, ratio > 0.6)
"Xyz"      → NotFound  (ratio < 0.6)
```

### Zero external dependencies

The library has **no runtime dependencies**. The Levenshtein algorithm is implemented from scratch inside `src/levenshtein.rs` using a standard two-row dynamic-programming approach with O(min(|a|, |b|)) memory. This was a deliberate design decision — adding a crate dependency solely for fuzzy matching would impose that dependency on every consumer of `partial-date`. The implementation is small enough (≈ 40 lines) that maintaining it in-tree is straightforward.

---

## Ordinal days

Days written as ordinals are parsed automatically without any configuration:

```
"1st"  → 1
"2nd"  → 2
"3rd"  → 3
"15th" → 15
"31st" → 31
```

The ordinal suffix is stripped before the number is extracted, and the result is treated identically to a plain numeric day.

---

## Partial extraction examples

```rust
use partial_date::extract::extract;
use partial_date::models::{Config, ComponentOrder, DateComponent, Extracted, Input};

// Month name + year, no day
let result = extract(Input {
    utterance: "June 2024".to_string(),
    config: None,
});
assert!(result.day.value.is_not_found());
assert_eq!(result.month.number, Extracted::Found(6));
assert_eq!(result.year.value,   Extracted::Found(2024));

// Ordinal day + month name, no year
let result = extract(Input {
    utterance: "3rd October".to_string(),
    config: None,
});
assert_eq!(result.day.value,    Extracted::Found(3));
assert_eq!(result.month.number, Extracted::Found(10));
assert!(result.year.value.is_not_found());

// Two-digit year with sliding-window expansion
let result = extract(Input {
    utterance: "22-03-16".to_string(),
    config: None, // DMY order, 16 → 2016
});
assert_eq!(result.day.value,    Extracted::Found(22));
assert_eq!(result.month.number, Extracted::Found(3));
assert_eq!(result.year.value,   Extracted::Found(2016));

// Misspelled month name
let result = extract(Input {
    utterance: "15 Octobar 2023".to_string(),
    config: None,
});
assert_eq!(result.day.value,    Extracted::Found(15));
assert_eq!(result.month.number, Extracted::Found(10));
assert_eq!(result.year.value,   Extracted::Found(2023));
```

---

## Language support

The library currently supports **English only**. Month name recognition (full names, abbreviations, prefix matching, and fuzzy matching) is built around the English month names January through December.

If you need support for another language — for example French month names (*janvier*, *février*, …) or Swahili (*Januari*, *Februari*, …) — please open an issue or pull request. The design is intentionally extensible: language support would be added through the `Config` struct (e.g. a `language: Language` field), allowing per-call language selection without breaking the existing API.

---

## Tokenise API

The tokeniser is also exposed directly if you need to inspect how an utterance is broken down before interpretation:

```rust
use partial_date::extract::tokenise;
use partial_date::models::{Config, MonthName, Token};

let tokens = tokenise("19 October 2014", &Config::default());

assert_eq!(tokens, vec![
    Token::Numeric(19, 2),
    Token::MonthName(MonthName::October),
    Token::Numeric(2014, 4),
]);
```

Each `Token::Numeric(value, digit_count)` carries the parsed value and the number of digits in the original source string. The digit count is what distinguishes `"03"` (a plausible day or month) from `"2024"` (an unambiguous four-digit year).

---

## License

GPL-3.0-only. See [LICENSE](LICENSE) for details.
