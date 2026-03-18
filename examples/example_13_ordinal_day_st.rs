//! Example 13: Ordinal Day Suffix 'st'
//! Run with: `cargo run --example example_13_ordinal_day_st`

use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    println!("Example 13: Ordinal Day Suffix");
    println!("=============================\n");
    
    println!("Input: '1st December 2024'");
    println!("Config: Default\n");

    let input = Input {
        utterance: "1st December 2024".to_string(),
        config: None,
    };

    let result = extract(input);
    
    println!("Results:");
    println!("  Day:   {:?}", result.day.value);
    println!("  Month: {:?}", result.month.number);
    println!("  Year:  {:?}", result.year.value);
    println!("\nExplanation:");
    println!("  The library recognizes '1st' as an ordinal day.");
    println!("  It extracts the numeric value (1) and ignores the suffix.");
    println!("  Also works with 'nd', 'rd', 'th' suffixes.");
}
