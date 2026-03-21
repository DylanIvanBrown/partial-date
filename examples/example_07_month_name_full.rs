//! Example 7: Natural Language - Full Month Name
//! Run with: `cargo run --example example_07_month_name_full`

use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    println!("Example 7: Natural Language Month Names");
    println!("======================================\n");

    println!("Input: '25 December 2024'");
    println!("Config: Default (auto-detects month name)\n");

    let input = Input {
        utterance: "25 December 2024".to_string(),
        config: None,
    };

    let result = extract(input);

    println!("Results:");
    println!("  Day:        {:?}", result.day.value);
    println!("  Month:      {:?}", result.month.number);
    println!("  Month Name: {:?}", result.month.name);
    println!("  Year:       {:?}", result.year.value);
    println!("\nExplanation:");
    println!("  The library recognizes 'December' as a month name.");
    println!("  It extracts both the numeric value (12) and MonthName enum.");
    println!("  ComponentOrder is not needed when month name is present!");
}
