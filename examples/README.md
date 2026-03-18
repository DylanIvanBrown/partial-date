# partial-date Examples

This directory contains 22+ comprehensive examples demonstrating the `partial-date` library with various configurations and use cases.

## Running the Examples

### Run all examples at once:
```bash
cargo run --example all_examples
```

### Run individual examples:
```bash
# Each example is also available as a standalone file
cargo run --example example_01_basic_dmy
cargo run --example example_02_basic_mdy
cargo run --example example_03_basic_ymd
# ... etc
```

## Example Overview

### Basic Date Formats (1-3)
- **Example 1**: DMY (Day-Month-Year) format - `25/12/2024`
- **Example 2**: MDY (Month-Day-Year) format - `12/25/2024`
- **Example 3**: YMD (Year-Month-Day) format - `2024/12/25`

### Partial Dates (4-6)
- **Example 4**: Day + Month only - `15/06`
- **Example 5**: Month + Year only - `June 2025`
- **Example 6**: Day only - `the 15th`

### Natural Language Month Names (7-9)
- **Example 7**: Full month name - `25 December 2024`
- **Example 8**: Abbreviated month - `15 Dec 2025`
- **Example 9**: Fuzzy/misspelled month - `31 Decmber 2024` (typo correction)

### Year Expansion Modes (10-12)
- **Example 10**: Sliding window (default) - `00-49→2000-2049, 50-99→1950-1999`
- **Example 11**: Always2000s - All 2-digit years become `2000-2099`
- **Example 12**: Literal - 2-digit years kept as-is (e.g., `24` → `24`)

### Ordinal Days (13-15)
- **Example 13**: Ordinal suffix 'st' - `1st December 2024`
- **Example 14**: Various ordinal suffixes - `21st`, `22nd`, `23rd`, `24th`
- **Example 15**: Ordinals in context - `Meeting on the 15th of June`

### Custom Configuration (16-18)
- **Example 16**: Min/max validation - Day must be 10-20
- **Example 17**: Default values - Missing day defaults to 15
- **Example 18**: No-separator concatenated - `25122024` (DDMMYYYY)

### Edge Cases (19-21)
- **Example 19**: Messy input - `25  /  12 - 2024` (extra spacing, mixed separators)
- **Example 20**: Boundary dates - First/last day of year, leap day
- **Example 21**: Fuzzy month matching - Various misspellings detected via Levenshtein distance

### Component Orders (22)
- **Example 22**: All 6 component orders - DMY, MDY, YMD, YDM, MYD, DYM

## Key Features Demonstrated

### Tokenisation
- Separator handling (/, -, ., space, etc.)
- Digit↔alpha boundary detection
- Ordinal suffix recognition
- Month name recognition (full, abbreviated, fuzzy)

### Component Assignment
- Positional assignment based on `ComponentOrder`
- Unambiguous override rules (e.g., day > 31 can't be a month)
- Month-before-day swap in MDY configurations
- `IsExpected` suppression (disabled components)

### Validation
- Min/max bounds checking
- Month range (1-12)
- Day range (1-31)
- Year range (configurable, default 0-3000)

### Defaults
- `Found` - value was extracted
- `Defaulted` - value was not found but a default was configured
- `NotFound` - value was not found and no default available

### Year Expansion
- **Sliding Window**: Smart date interpretation (2024 vs 1999)
- **Always2000s**: All 2-digit years map to 2000-2099
- **Literal**: 2-digit years preserved as-is

## Configuration Reference

### Basic Config
```rust
let config = Config::default();
```

### With Component Order
```rust
let config = Config {
    component_order: ComponentOrder {
        first: DateComponent::Day,
        second: DateComponent::Month,
        third: DateComponent::Year,
    },
    ..Default::default()
};
```

### With Constraints
```rust
let config = Config {
    day: DayConfig {
        min: 10,
        max: 20,
        expected: IsExpected::Yes,
        default: Some(15),
    },
    ..Default::default()
};
```

### With Year Expansion Mode
```rust
let config = Config {
    year: YearConfig {
        two_digit_expansion: TwoDigitYearExpansion::Always2000s,
        ..Default::default()
    },
    ..Default::default()
};
```

### No-Separator Mode
```rust
let config = Config {
    no_separator: true,
    component_order: ComponentOrder {
        first: DateComponent::Day,
        second: DateComponent::Month,
        third: DateComponent::Year,
    },
    ..Default::default()
};
```

## Input Types

### Basic Input
```rust
let input = Input {
    utterance: "25/12/2024".to_string(),
    config: None,  // Uses library defaults
};
```

### With Custom Config
```rust
let input = Input {
    utterance: "25/12/2024".to_string(),
    config: Some(custom_config),
};
```

## Output Structure

All examples extract a `PartialDate` with three components:

```rust
pub struct PartialDate {
    pub day: Day {
        value: Extracted<u8>,  // Found(15), Defaulted(15), or NotFound
    },
    pub month: Month {
        number: Extracted<u8>,      // 1-12
        name: Extracted<MonthName>, // January, February, etc.
    },
    pub year: Year {
        value: Extracted<i32>,  // Any valid year
    },
}
```

### Extracted Enum
- **`Found(T)`**: Value was successfully extracted
- **`Defaulted(T)`**: Value not found; a configured default was used
- **`NotFound`**: No value extracted and no default available

## Use Cases

These examples cover real-world scenarios:

1. **Global Date Parsing** - Support multiple date conventions (DMY, MDY, YMD)
2. **Natural Language** - Extract dates from user input with month names
3. **Flexible Input** - Handle messy, misspelled, partially-formatted dates
4. **Validation** - Enforce business rules (e.g., only specific date ranges)
5. **Defaults** - Provide sensible fallbacks for missing components
6. **Fuzzy Matching** - Correct minor typos in month names automatically
7. **Legacy Data** - Support concatenated formats (no-separator dates)
8. **Partial Info** - Extract what's available, leave rest as NotFound

## Testing with Examples

Each example includes:
- Clear input description
- Configuration explanation
- Expected output
- Use case context

This makes them excellent for:
- Learning the library API
- Testing custom configurations
- Verifying expected behavior
- Debugging extraction issues
