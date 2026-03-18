//! Example 9: Fuzzy Month Name Matching (Misspellings)
//! Run with: `cargo run --example example_09_month_name_fuzzy`

use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    println!("Example 9: Fuzzy Month Name Matching");
    println!("===================================\n");
    
    println!("Input: '31 Decmber 2024' (notice typo: 'Decmber')");
    println!("Config: Default (uses Levenshtein distance)\n");

    let input = Input {
        utterance: "31 Decmber 2024".to_string(),
        config: None,
    };

    let result = extract(input);
    
    println!("Results:");
    println!("  Day:        {:?}", result.day.value);
    println!("  Month:      {:?}", result.month.number);
    println!("  Month Name: {:?}", result.month.name);
    println!("  Year:       {:?}", result.year.value);
    println!("\nExplanation:");
    println!("  Despite the typo 'Decmber', the library correctly");
    println!("  identifies it as December using Levenshtein distance.");
    println!("  This fuzzy matching helps with OCR errors and typos!");
}
