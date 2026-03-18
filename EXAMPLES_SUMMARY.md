# Examples Summary - partial-date Library

## Overview

The `examples/` directory contains **22+ comprehensive examples** demonstrating all major features of the partial-date library with various configurations and real-world use cases.

## Quick Start

Run all examples at once:
```bash
cargo run --example all_examples
```

Run a specific individual example:
```bash
cargo run --example example_01_basic_dmy
cargo run --example example_07_month_name_full
cargo run --example example_13_ordinal_day_st
```

## Files Created

### Main Examples File
- **`all_examples.rs`** (23 KB) - Comprehensive runner with 22 examples

### Individual Example Files (for quick reference)
- `example_01_basic_dmy.rs` - DMY format
- `example_02_basic_mdy.rs` - MDY format
- `example_07_month_name_full.rs` - Full month names
- `example_09_month_name_fuzzy.rs` - Fuzzy month matching
- `example_13_ordinal_day_st.rs` - Ordinal days

### Documentation
- **`README.md`** - Complete examples guide with configuration reference
- **`EXAMPLES_SUMMARY.md`** - This file

## Example Categories

### 1. Basic Date Formats (Examples 1-3)
Demonstrates fundamental date parsing with different component orders:

```rust
// Example 1: DMY Format
Input: "25/12/2024"
Result: Day=25, Month=12, Year=2024

// Example 2: MDY Format  
Input: "12/25/2024"
Result: Day=25, Month=12, Year=2024 (different order!)

// Example 3: YMD Format
Input: "2024/12/25"
Result: Day=25, Month=12, Year=2024
```

**Key Concept**: `ComponentOrder` tells the parser how to interpret numeric components.

---

### 2. Partial Dates (Examples 4-6)
Extracts only available components, leaving others as `NotFound` or `Defaulted`:

```rust
// Example 4: Day + Month only
Input: "15/06"
Config: Year=No (not expected)
Result: Day=15, Month=6, Year=NotFound

// Example 5: Month + Year only
Input: "June 2025"
Config: Day=No (not expected)
Result: Day=NotFound, Month=6, Year=2025

// Example 6: Day only
Input: "the 15th"
Result: Day=15, Month=NotFound, Year=NotFound
```

**Key Concept**: Control which components are extracted via `IsExpected` flags.

---

### 3. Natural Language Month Names (Examples 7-9)
Recognizes month names in full, abbreviated, or misspelled form:

```rust
// Example 7: Full month name
Input: "25 December 2024"
Result: Month=12, Name=December

// Example 8: Abbreviated (3-letter)
Input: "15 Dec 2025"
Result: Month=12, Name=December

// Example 9: Fuzzy matching (typo)
Input: "31 Decmber 2024" ← typo!
Result: Month=12, Name=December ✓ (corrected!)
```

**Key Concept**: Levenshtein distance fuzzy matching handles OCR errors and typos.

---

### 4. Year Expansion Modes (Examples 10-12)
Three strategies for converting 2-digit years to full 4-digit years:

```rust
// Example 10: Sliding Window (DEFAULT)
Input: "25/12/24" and "25/12/99"
Config: 00-49→2000-2049, 50-99→1950-1999
Result: 24→2024, 99→1999 (sensible defaults)

// Example 11: Always2000s
Input: "25/12/99"
Result: 99→2099 (all become 2000-2099)

// Example 12: Literal
Input: "25/12/24"
Result: 24→24 (no expansion, keep as-is)
```

**Key Concept**: Choose expansion mode based on your data domain.

---

### 5. Ordinal Days (Examples 13-15)
Handles day numbers with ordinal suffixes (st, nd, rd, th):

```rust
// Example 13: 'st' suffix
Input: "1st December 2024"
Result: Day=1

// Example 14: Various ordinals
Input: "21st", "22nd", "23rd", "24th"
Result: Correctly extracted regardless of suffix

// Example 15: In natural language
Input: "Meeting on the 15th of June"
Result: Day=15, Month=6
```

**Key Concept**: Ordinals are unambiguous indicators of days.

---

### 6. Custom Configuration (Examples 16-18)
Demonstrates config options: min/max bounds, defaults, concatenated dates:

```rust
// Example 16: Min/Max validation
Config: Day min=10, max=20
Input: "15/06/2024"
Result: Day=15 ✓ (in range)

// Example 17: Default values
Config: Default day = 15
Input: "June 2024"
Result: Day=Defaulted(15), Month=6

// Example 18: No-separator mode
Config: no_separator=true, DMY order
Input: "25122024"
Result: Day=25, Month=12, Year=2024
```

**Key Concept**: Config enables validation and sensible fallbacks.

---

### 7. Edge Cases (Examples 19-21)
Real-world messy input, boundaries, and fuzzy matching:

```rust
// Example 19: Messy input
Input: "25  /  12 - 2024" (extra spaces, mixed separators)
Result: Day=25, Month=12, Year=2024 ✓

// Example 20: Boundary dates
Input: "01/01/2024" (first day)
Input: "31/12/2024" (last day)
Input: "29/02/2024" (leap day)
Result: All correctly extracted

// Example 21: Fuzzy matching
Input: "Januray" (transposition)
Input: "Decmber" (missing letter)
Input: "Ocotber" (character swap)
Result: All correctly identified via Levenshtein distance
```

**Key Concept**: Robust parsing handles real-world messiness.

---

### 8. Component Orders (Example 22)
Same input, different interpretations based on `ComponentOrder`:

```rust
Input: "01 06 24"

DMY → Day=1, Month=6, Year=2024
MDY → Day=6, Month=1, Year=2024 (swapped!)
YMD → Day=24, Month=6, Year=2001
YDM → Day=6, Month=NotFound, Year=2001
MYD → Day=24, Month=1, Year=2006
DYM → Day=1, Month=NotFound, Year=2006
```

**Key Concept**: ComponentOrder is crucial for ambiguous numeric dates.

---

## Configuration Pattern Reference

### Minimal Config (Uses Defaults)
```rust
Input {
    utterance: "25 December 2024".to_string(),
    config: None,
}
```

### With Custom Component Order
```rust
Input {
    utterance: "25/12/2024".to_string(),
    config: Some(Config {
        component_order: ComponentOrder {
            first: DateComponent::Day,
            second: DateComponent::Month,
            third: DateComponent::Year,
        },
        ..Default::default()
    }),
}
```

### With Constraints
```rust
Input {
    utterance: "15/06/2024".to_string(),
    config: Some(Config {
        day: DayConfig {
            min: 10,
            max: 20,
            expected: IsExpected::Yes,
            default: Some(15),
        },
        ..Default::default()
    }),
}
```

### With Year Expansion
```rust
Input {
    utterance: "25/12/24".to_string(),
    config: Some(Config {
        year: YearConfig {
            two_digit_expansion: TwoDigitYearExpansion::Always2000s,
            ..Default::default()
        },
        ..Default::default()
    }),
}
```

### No-Separator Mode
```rust
Input {
    utterance: "25122024".to_string(),
    config: Some(Config {
        no_separator: true,
        component_order: ComponentOrder {
            first: DateComponent::Day,
            second: DateComponent::Month,
            third: DateComponent::Year,
        },
        ..Default::default()
    }),
}
```

---

## Output Structure

Every `extract()` call returns a `PartialDate`:

```rust
pub struct PartialDate {
    pub day: Day {
        value: Extracted<u8>,  // Found, Defaulted, or NotFound
    },
    pub month: Month {
        number: Extracted<u8>,      // 1-12
        name: Extracted<MonthName>, // January, February, etc.
    },
    pub year: Year {
        value: Extracted<i32>,  // Any valid year
    },
}

pub enum Extracted<T> {
    Found(T),        // Value was extracted from input
    Defaulted(T),    // Value not found; default was used
    NotFound,        // No value and no default
}
```

---

## Use Cases Covered

### 1. Global Date Parsing
Support multiple conventions (DMY, MDY, YMD) based on user region.

### 2. Natural Language Input
Extract dates from conversational text with month names, ordinals, etc.

### 3. Data Import
Handle legacy or messy date formats from databases/spreadsheets.

### 4. Form Validation
Enforce date ranges and provide sensible defaults.

### 5. OCR Processing
Correct minor misspellings in automatically scanned documents.

### 6. Partial Information
Accept incomplete dates and indicate what was missing.

### 7. Two-Digit Year Handling
Choose appropriate expansion strategy for your dataset.

### 8. Fuzzy Matching
Robustly handle user input with typos and variations.

---

## Testing Each Feature

Every example demonstrates:
- ✅ Clear input description
- ✅ Configuration explanation  
- ✅ Expected output
- ✅ Use case context
- ✅ Key concepts

This makes them ideal for:
- Learning the library API
- Testing custom configurations
- Verifying expected behavior
- Debugging extraction issues
- Understanding tradeoffs

---

## Running Examples

### View all examples at once:
```bash
cargo run --example all_examples
```

Expected output includes 22 clearly labeled sections with input/output for each example.

### Run specific examples:
```bash
cargo run --example example_01_basic_dmy
cargo run --example example_02_basic_mdy
cargo run --example example_07_month_name_full
cargo run --example example_09_month_name_fuzzy
cargo run --example example_13_ordinal_day_st
```

Each individual example is self-contained and can be run independently.

---

## Example Statistics

- **Total Examples**: 22+
- **Categories**: 8 (formats, partial, names, years, ordinals, config, edge cases, orders)
- **Configuration Types**: 5+ (basic, order, constraints, expansion, concatenated)
- **Features Demonstrated**: 15+
- **Lines of Code**: ~850 in all_examples.rs

---

## Next Steps

1. **Run `cargo run --example all_examples`** to see all examples
2. **Pick an example** that matches your use case
3. **Copy the configuration** into your code
4. **Adjust settings** as needed for your data
5. **Test thoroughly** with your real-world inputs

---

## Integration with Your Code

Each example shows the exact pattern to follow:

```rust
use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    let input = Input {
        utterance: "your input here".to_string(),
        config: Some(/* your config */),
    };
    
    let result = extract(input);
    
    println!("Day: {:?}", result.day.value);
    println!("Month: {:?}", result.month.number);
    println!("Year: {:?}", result.year.value);
}
```

Copy this structure and customize the utterance and config!
