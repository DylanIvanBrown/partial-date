//! Comprehensive examples showcasing the partial-date library with 22+ different use cases.
//!
//! Run with: `cargo run --example all_examples`

use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    println!("======================================================================");
    println!("partial-date: 22+ Examples");
    println!("======================================================================\n");

    example_1_basic_dmy();
    example_2_basic_mdy();
    example_3_basic_ymd();
    example_4_partial_day_and_month();
    example_5_partial_month_and_year();
    example_6_day_only();
    example_7_month_name_full();
    example_8_month_name_abbreviated();
    example_9_month_name_fuzzy();
    example_10_year_two_digit_sliding_window();
    example_11_year_two_digit_always_2000s();
    example_12_year_two_digit_literal();
    example_13_ordinal_day_st();
    example_14_ordinal_day_varied();
    example_15_ordinal_day_in_context();
    example_16_custom_min_max();
    example_17_default_values();
    example_18_no_separator_concat();
    example_19_messy_input();
    example_20_boundary_dates();
    example_21_fuzzy_month_name();
    example_22_all_component_orders();
}

// ============================================================================
// EXAMPLE 1: Basic DMY (Day-Month-Year) Format
// ============================================================================
fn example_1_basic_dmy() {
    println!("1️⃣  BASIC DMY FORMAT");
    println!("   Input: '25/12/2024'");
    println!("   Config: Day-Month-Year order\n");

    let input = Input {
        utterance: "25/12/2024".to_string(),
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
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 2: Basic MDY (Month-Day-Year) Format
// ============================================================================
fn example_2_basic_mdy() {
    println!("2️⃣  BASIC MDY FORMAT");
    println!("   Input: '12/25/2024'");
    println!("   Config: Month-Day-Year order\n");

    let input = Input {
        utterance: "12/25/2024".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Month,
                second: DateComponent::Day,
                third: DateComponent::Year,
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 3: Basic YMD (Year-Month-Day) Format
// ============================================================================
fn example_3_basic_ymd() {
    println!("3️⃣  BASIC YMD FORMAT");
    println!("   Input: '2024/12/25'");
    println!("   Config: Year-Month-Day order\n");

    let input = Input {
        utterance: "2024/12/25".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Year,
                second: DateComponent::Month,
                third: DateComponent::Day,
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 4: Partial Date - Day + Month Only
// ============================================================================
fn example_4_partial_day_and_month() {
    println!("4️⃣  PARTIAL DATE: Day + Month");
    println!("   Input: '15/06'");
    println!("   Config: DMY order, Day=Yes, Month=Yes, Year=No\n");

    let input = Input {
        utterance: "15/06".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Month,
                third: DateComponent::Year,
            },
            year: YearConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?} (not expected)\n", result.year.value);
}

// ============================================================================
// EXAMPLE 5: Partial Date - Month + Year Only
// ============================================================================
fn example_5_partial_month_and_year() {
    println!("5️⃣  PARTIAL DATE: Month + Year");
    println!("   Input: 'June 2025'");
    println!("   Config: Day=No, Month=Yes, Year=Yes\n");

    let input = Input {
        utterance: "June 2025".to_string(),
        config: Some(Config {
            day: DayConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?} (not expected)", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 6: Day Only
// ============================================================================
fn example_6_day_only() {
    println!("6️⃣  PARTIAL DATE: Day Only");
    println!("   Input: 'the 15th'");
    println!("   Config: Day=Yes, Month=No, Year=No\n");

    let input = Input {
        utterance: "the 15th".to_string(),
        config: Some(Config {
            month: MonthConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            year: YearConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?} (not expected)", result.month.number);
    println!("   ✓ Year:  {:?} (not expected)\n", result.year.value);
}

// ============================================================================
// EXAMPLE 7: Month Name - Full Name
// ============================================================================
fn example_7_month_name_full() {
    println!("7️⃣  NATURAL LANGUAGE: Full Month Name");
    println!("   Input: '25 December 2024'");
    println!("   Config: Default (accepts month names)\n");

    let input = Input {
        utterance: "25 December 2024".to_string(),
        config: None, // Uses default config
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Month Name: {:?}", result.month.name);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 8: Month Name - Abbreviated (3-letter)
// ============================================================================
fn example_8_month_name_abbreviated() {
    println!("8️⃣  NATURAL LANGUAGE: Abbreviated Month Name");
    println!("   Input: '15 Dec 2025'");
    println!("   Config: Default\n");

    let input = Input {
        utterance: "15 Dec 2025".to_string(),
        config: None,
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Month Name: {:?}", result.month.name);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 9: Month Name - Fuzzy Matching (Misspelled)
// ============================================================================
fn example_9_month_name_fuzzy() {
    println!("9️⃣  NATURAL LANGUAGE: Fuzzy Misspelled Month");
    println!("   Input: '31 Decmber 2024' (typo: 'Decmber')");
    println!("   Config: Default (uses Levenshtein fuzzy matching)\n");

    let input = Input {
        utterance: "31 Decmber 2024".to_string(),
        config: None,
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!(
        "   ✓ Month Name: {:?} (correctly matched!)",
        result.month.name
    );
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 10: Two-Digit Year - Sliding Window (Default)
// ============================================================================
fn example_10_year_two_digit_sliding_window() {
    println!("🔟 TWO-DIGIT YEAR: Sliding Window Expansion");
    println!("   Input: '25/12/24'");
    println!("   Config: Default (00-49→2000-2049, 50-99→1950-1999)\n");

    let cases = vec![
        ("25/12/24", 2024),
        ("15/06/99", 1999),
        ("01/01/00", 2000),
        ("31/12/50", 1950),
    ];

    for (input_str, _expected_year) in cases {
        let input = Input {
            utterance: input_str.to_string(),
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
        println!("   '{:12}' → Year: {:?}", input_str, result.year.value);
    }
    println!();
}

// ============================================================================
// EXAMPLE 11: Two-Digit Year - Always(Century)
// ============================================================================
fn example_11_year_two_digit_always_2000s() {
    println!("1️⃣1️⃣  TWO-DIGIT YEAR: Always(Century(2000))");
    println!("   Input: '25/12/99'");
    println!("   Config: Two-digit expansion = Always(Century(2000)) (all become 2000-2099)\n");

    let input = Input {
        utterance: "25/12/99".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Month,
                third: DateComponent::Year,
            },
            year: YearConfig {
                expected: IsExpected::Yes,
                two_digit_expansion: TwoDigitYearExpansion::Always(Century::new(2000).unwrap()),
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Input '99' expands to: {:?}", result.year.value);
    println!("   (Note: normally 99 → 1999, here → 2099)\n");
}

// ============================================================================
// EXAMPLE 12: Two-Digit Year - Literal (No Expansion)
// ============================================================================
fn example_12_year_two_digit_literal() {
    println!("1️⃣2️⃣  TWO-DIGIT YEAR: Literal (No Expansion)");
    println!("   Input: '25/12/24'");
    println!("   Config: Two-digit expansion = Literal (keep as-is)\n");

    let input = Input {
        utterance: "25/12/24".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Month,
                third: DateComponent::Year,
            },
            year: YearConfig {
                expected: IsExpected::Yes,
                two_digit_expansion: TwoDigitYearExpansion::Literal,
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Input '24' stays as: {:?}", result.year.value);
    println!("   (Normally 24 → 2024, here → 24)\n");
}

// ============================================================================
// EXAMPLE 13: Ordinal Day - 'st' Suffix
// ============================================================================
fn example_13_ordinal_day_st() {
    println!("1️⃣3️⃣  ORDINAL DAY: 'st' Suffix");
    println!("   Input: '1st December 2024'");
    println!("   Config: Default\n");

    let input = Input {
        utterance: "1st December 2024".to_string(),
        config: None,
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 14: Ordinal Day - Varied Suffixes
// ============================================================================
fn example_14_ordinal_day_varied() {
    println!("1️⃣4️⃣  ORDINAL DAY: Various Suffixes");
    println!("   Inputs: '21st', '22nd', '23rd', '24th'\n");

    let cases = vec![
        "21st June 2024",
        "22nd June 2024",
        "23rd June 2024",
        "24th June 2024",
    ];

    for input_str in cases {
        let input = Input {
            utterance: input_str.to_string(),
            config: None,
        };
        let result = extract(input);
        println!("   '{}' → Day: {:?}", input_str, result.day.value);
    }
    println!();
}

// ============================================================================
// EXAMPLE 15: Ordinal Day in Natural Language Context
// ============================================================================
fn example_15_ordinal_day_in_context() {
    println!("1️⃣5️⃣  ORDINAL DAY: In Context");
    println!("   Input: 'Meeting on the 15th of June'");
    println!("   Config: Default\n");

    let input = Input {
        utterance: "Meeting on the 15th of June".to_string(),
        config: None,
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 16: Custom Min/Max Validation
// ============================================================================
fn example_16_custom_min_max() {
    println!("1️⃣6️⃣  CUSTOM CONFIG: Min/Max Validation");
    println!("   Input: '15/06/2024'");
    println!("   Config: Day min=10, max=20 (only days 10-20 valid)\n");

    let input = Input {
        utterance: "15/06/2024".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Month,
                third: DateComponent::Year,
            },
            day: DayConfig {
                min: 10,
                max: 20,
                expected: IsExpected::Yes,
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day 15 (in range 10-20): {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 17: Default Values
// ============================================================================
fn example_17_default_values() {
    println!("1️⃣7️⃣  DEFAULT VALUES");
    println!("   Input: 'June 2024' (no day)");
    println!("   Config: Default day = 15\n");

    let input = Input {
        utterance: "June 2024".to_string(),
        config: Some(Config {
            day: DayConfig {
                expected: IsExpected::Yes,
                default: Some(15),
                ..Default::default()
            },
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?} (from default)", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 18: No-Separator Concatenated Dates
// ============================================================================
fn example_18_no_separator_concat() {
    println!("1️⃣8️⃣  NO-SEPARATOR CONCATENATED DATES");
    println!("   Input: '25122024' (DDMMYYYY with no separators)");
    println!("   Config: no_separator=true, DMY order\n");

    let input = Input {
        utterance: "25122024".to_string(),
        config: Some(Config {
            component_order: ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Month,
                third: DateComponent::Year,
            },
            no_separator: true,
            ..Default::default()
        }),
    };

    let result = extract(input);
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 19: Messy Input with Extra Spacing & Mixed Separators
// ============================================================================
fn example_19_messy_input() {
    println!("1️⃣9️⃣  MESSY INPUT: Extra spacing & mixed separators");
    println!("   Input: '25  /  12 - 2024'");
    println!("   Config: Default (handles various separators)\n");

    let input = Input {
        utterance: "25  /  12 - 2024".to_string(),
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
    println!("   ✓ Day:   {:?}", result.day.value);
    println!("   ✓ Month: {:?}", result.month.number);
    println!("   ✓ Year:  {:?}\n", result.year.value);
}

// ============================================================================
// EXAMPLE 20: Boundary Dates
// ============================================================================
fn example_20_boundary_dates() {
    println!("2️⃣0️⃣  BOUNDARY DATES");
    println!("   Testing edge cases: first/last day of month/year\n");

    let cases = vec![
        ("01/01/2024", "First day of year"),
        ("31/12/2024", "Last day of year"),
        ("29/02/2024", "Leap day"),
    ];

    for (input_str, description) in cases {
        let input = Input {
            utterance: input_str.to_string(),
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
        println!(
            "   {} ({}): Day={:?}, Month={:?}, Year={:?}",
            input_str, description, result.day.value, result.month.number, result.year.value
        );
    }
    println!();
}

// ============================================================================
// EXAMPLE 21: Fuzzy Month Matching with Various Misspellings
// ============================================================================
fn example_21_fuzzy_month_name() {
    println!("2️⃣1️⃣  FUZZY MONTH MATCHING: Misspellings");
    println!("   Testing Levenshtein distance fuzzy matching\n");

    let cases = vec![
        ("25 Januray 2024", "Januray (transposition)"),
        ("15 Decmber 2024", "Decmber (missing letter)"),
        ("31 Ocotber 2024", "Ocotber (swap)"),
    ];

    for (input_str, description) in cases {
        let input = Input {
            utterance: input_str.to_string(),
            config: None,
        };
        let result = extract(input);
        println!(
            "   {} → {:?} {}",
            description,
            result.month.name,
            if result.month.number.is_found() {
                "✓"
            } else {
                "✗"
            }
        );
    }
    println!();
}

// ============================================================================
// EXAMPLE 22: All Component Orders (DMY, MDY, YMD, YDM, MYD, DYM)
// ============================================================================
fn example_22_all_component_orders() {
    println!("2️⃣2️⃣  ALL COMPONENT ORDERS");
    println!("   Same input '01 06 24', different interpretations based on order\n");

    let input_str = "01 06 24";
    let orders = vec![
        (
            "DMY",
            ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Month,
                third: DateComponent::Year,
            },
        ),
        (
            "MDY",
            ComponentOrder {
                first: DateComponent::Month,
                second: DateComponent::Day,
                third: DateComponent::Year,
            },
        ),
        (
            "YMD",
            ComponentOrder {
                first: DateComponent::Year,
                second: DateComponent::Month,
                third: DateComponent::Day,
            },
        ),
        (
            "YDM",
            ComponentOrder {
                first: DateComponent::Year,
                second: DateComponent::Day,
                third: DateComponent::Month,
            },
        ),
        (
            "MYD",
            ComponentOrder {
                first: DateComponent::Month,
                second: DateComponent::Year,
                third: DateComponent::Day,
            },
        ),
        (
            "DYM",
            ComponentOrder {
                first: DateComponent::Day,
                second: DateComponent::Year,
                third: DateComponent::Month,
            },
        ),
    ];

    for (order_name, order) in orders {
        let input = Input {
            utterance: input_str.to_string(),
            config: Some(Config {
                component_order: order,
                year: YearConfig {
                    expected: IsExpected::Yes,
                    two_digit_expansion: TwoDigitYearExpansion::SlidingWindow {
                        earliest_year: 1950,
                        pivot: SlidingWindowPivot::new(50).unwrap(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            }),
        };

        let result = extract(input);
        println!(
            "   {} → Day: {:?}, Month: {:?}, Year: {:?}",
            order_name, result.day.value, result.month.number, result.year.value
        );
    }
    println!();
}
