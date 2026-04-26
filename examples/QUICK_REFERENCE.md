# Quick Reference Guide - partial-date Examples

## The Fastest Way to Get Started

### 1. See All Examples (2 minutes)
```bash
cargo run --example all_examples
```

### 2. Pick Your Use Case

| Use Case | Example | Key Feature |
|----------|---------|------------|
| Parse dates with day first | `example_01_basic_dmy` | ComponentOrder: Day, Month, Year |
| Parse dates with month first | `example_02_basic_mdy` | ComponentOrder: Month, Day, Year |
| Extract only month & year | `example_05_partial_month_and_year` | IsExpected::No |
| Work with month names | `example_07_month_name_full` | Natural language |
| Handle typos in months | `example_09_month_name_fuzzy` | Levenshtein distance |
| Support ordinal days (1st, 2nd) | `example_13_ordinal_day_st` | Token::OrdinalDay |
| Set min/max constraints | `example_16_custom_min_max` | DayConfig.min/max |
| Use default values | `example_17_default_values` | DayConfig.default |
| Parse concatenated dates | `example_18_no_separator_concat` | no_separator=true |
| Handle messy input | `example_19_messy_input` | Robust separators |

### 3. Copy the Pattern

Every example follows this structure:

```rust
use partial_date::extract::extract;
use partial_date::models::*;

let input = Input {
    utterance: "your date string".to_string(),
    config: Some(Config {
        component_order: ComponentOrder {
            first: DateComponent::Day,
            second: DateComponent::Month,
            third: DateComponent::Year,
        },
        ..Default::default()
    }),
};

let result = extract(input);
println!("Day: {:?}", result.day.value);   // Found(25), Defaulted(15), NotFound
println!("Month: {:?}", result.month.number);
println!("Year: {:?}", result.year.value);
```

---

## Configuration Snippets

### Bare Minimum (Default Config)
```rust
let input = Input {
    utterance: "25 December 2024".to_string(),
    config: None,  // Uses library defaults
};
```

### Specific Component Order
```rust
Config {
    component_order: ComponentOrder {
        first: DateComponent::Day,
        second: DateComponent::Month,
        third: DateComponent::Year,
    },
    ..Default::default()
}
```

### Skip Year Component
```rust
Config {
    year: YearConfig {
        expected: IsExpected::No,  // Won't extract year
        ..Default::default()
    },
    ..Default::default()
}
```

### Add Min/Max Validation
```rust
DayConfig {
    min: 10,           // Days 1-9 invalid
    max: 20,           // Days 21+ invalid
    expected: IsExpected::Yes,
    ..Default::default()
}
```

### Provide Defaults
```rust
DayConfig {
    expected: IsExpected::Yes,
    default: Some(15),  // Missing day → 15
    ..Default::default()
}
```

### Handle 2-Digit Years
```rust
YearConfig {
    two_digit_expansion: TwoDigitYearExpansion::Always(Century::new(2000).unwrap()),  // 99 → 2099
    // or
    two_digit_expansion: TwoDigitYearExpansion::Literal,      // 99 → 99
    // or (default)
    two_digit_expansion: TwoDigitYearExpansion::SlidingWindow {
        earliest_year: 1950,
        pivot: SlidingWindowPivot::new(50).unwrap(),  // 99 → 1999, 24 → 2024
    },
    ..Default::default()
}
```

### Handle No-Separator Dates
```rust
Config {
    no_separator: true,  // Enable "25122024" parsing
    component_order: ComponentOrder {
        first: DateComponent::Day,
        second: DateComponent::Month,
        third: DateComponent::Year,
    },
    ..Default::default()
}
```

---

## Output Interpretation

### Extracted<T> Enum
Each component is wrapped in `Extracted<T>`:

```rust
// Successful extraction
result.day.value == Found(25)

// Missing but default configured  
result.day.value == Defaulted(15)

// Not found and no default
result.day.value == NotFound
```

### Checking Values
```rust
// Match on Found/Defaulted/NotFound
match result.day.value {
    Found(d) => println!("Day: {}", d),
    Defaulted(d) => println!("Day (default): {}", d),
    NotFound => println!("No day provided"),
}

// Or use helper methods
if result.day.value.is_found() { /* ... */ }
if result.month.number.is_not_found() { /* ... */ }
```

---

## Component Orders at a Glance

For input "01 06 24":

| Order | Code | Day | Month | Year |
|-------|------|-----|-------|------|
| DMY | Day, Month, Year | 1 | 6 | 2024 |
| MDY | Month, Day, Year | 6 | 1 | 2024 |
| YMD | Year, Month, Day | 24 | 6 | 2001 |
| YDM | Year, Day, Month | 6 | — | 2001 |
| MYD | Month, Year, Day | 24 | 1 | 2006 |
| DYM | Day, Year, Month | 1 | — | 2006 |

---

## Tokenisation: What Gets Extracted

The `tokenise()` function recognizes:

### Numeric Tokens
```
"25"     → Token::Numeric(25, 2)     // value, digit count
"2024"   → Token::Numeric(2024, 4)
```

### Ordinal Tokens
```
"1st"    → Token::OrdinalDay(1)
"22nd"   → Token::OrdinalDay(22)
"31st"   → Token::OrdinalDay(31)
```

### Month Name Tokens
```
"December"  → Token::MonthName(MonthName::December)
"Dec"       → Token::MonthName(MonthName::December)
"Decmber"   → Token::MonthName(MonthName::December)  // Fuzzy!
```

### Ignored
```
"the"       → Ignored (noise)
"on"        → Ignored (noise)
"2024abc"   → Token::Numeric(2024, 4) + Ignored("abc")
```

---

## Year Expansion Modes

### Sliding Window (DEFAULT)
```
Input: 24, 99, 00, 50
Range: 00-49 → 2000-2049, 50-99 → 1950-1999
Output: 2024, 1999, 2000, 1950
Best for: Recent historical data (1950s-2050s)
```

### Always(Century)
```
Always(Century::new(2000).unwrap())
Input: 24, 99, 00, 50
Output: 2024, 2099, 2000, 2050
Best for: Systems with guaranteed single-century dates (e.g. all 2000s)
```

### Literal
```
Input: 24, 99, 00, 50
Output: 24, 99, 0, 50
Best for: Legacy systems or already-expanded years
```

---

## Separator Support

The library handles these separators automatically:
- Space: `25 12 2024`
- Slash: `25/12/2024`
- Dash: `25-12-2024`
- Dot: `25.12.2024`
- Comma: `25,12,2024`
- Backslash: `25\12\2024`
- Multiple/Mixed: `25  /  12 - 2024`

Use `config.extra_separators` for custom ones!

---

## Common Patterns

### Pattern 1: EU Dates (DMY)
```rust
Config {
    component_order: ComponentOrder::dmy(),
    ..Default::default()
}
```

### Pattern 2: US Dates (MDY)
```rust
Config {
    component_order: ComponentOrder::mdy(),
    ..Default::default()
}
```

### Pattern 3: ISO Dates (YMD)
```rust
Config {
    component_order: ComponentOrder::ymd(),
    ..Default::default()
}
```

### Pattern 4: Natural Language Only
```rust
Input {
    utterance: "25 December 2024".to_string(),
    config: None,  // Month name provides clarity
}
```

### Pattern 5: Partial Date
```rust
Config {
    year: YearConfig {
        expected: IsExpected::No,
        ..Default::default()
    },
    ..Default::default()
}
```

---

## Validation Rules

All values are bounds-checked:

### Day
- Default min: 1
- Default max: 31
- Customizable via `DayConfig.min/max`

### Month
- Default min: 1
- Default max: 12
- Fixed by definition (can't customize)

### Year
- Default min: 0
- Default max: 3000
- Customizable via `YearConfig.min/max`

---

## Execution Flow

```
Input: "25/12/2024"
   ↓
tokenise() → [Numeric(25,2), Numeric(12,2), Numeric(2024,4)]
   ↓
interpret_tokens() → (day_raw=Some((25,2)), month_raw=Some((12,2)), year_raw=Some(2024))
   ↓
validate_day/month/year() → Check min/max bounds
   ↓
apply_default() → Convert to Found/Defaulted/NotFound
   ↓
Result: PartialDate { day: Found(25), month: Found(12), year: Found(2024) }
```

---

## Debugging Tips

### What was tokenised?
```rust
let tokens = tokenise("25 December 2024", &config);
println!("{:?}", tokens);
// Output: [Numeric(25, 2), MonthName(December), Numeric(2024, 4)]
```

### What's the component order?
```rust
let order = config.component_order;
println!("First: {:?}, Second: {:?}, Third: {:?}", 
    order.first, order.second, order.third);
```

### Is a value found or defaulted?
```rust
match result.day.value {
    Found(d) => println!("Extracted: {}", d),
    Defaulted(d) => println!("Used default: {}", d),
    NotFound => println!("Neither found nor default"),
}
```

---

## Performance Notes

- Tokenisation: O(n) in input length
- Interpretation: O(1) for fixed 3-token limit
- Validation: O(1) bounds checks
- Fuzzy matching: O(m*n) Levenshtein where m,n = month name lengths

**Practical**: All operations complete in microseconds for typical input.

---

## See Also

- `README.md` - Detailed documentation
- `EXAMPLES_SUMMARY.md` - Full explanation of all 22 examples
- Source: Run `cargo run --example all_examples`
