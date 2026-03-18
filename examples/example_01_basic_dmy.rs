//! Example 1: Basic DMY (Day-Month-Year) Format
//! Run with: `cargo run --example example_01_basic_dmy`

use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    println!("Example 1: Basic DMY Format");
    println!("==========================\n");
    
    println!("Input: '25/12/2024'");
    println!("Config: Day-Month-Year order\n");

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
    
    println!("Results:");
    println!("  Day:   {:?}", result.day.value);
    println!("  Month: {:?}", result.month.number);
    println!("  Year:  {:?}", result.year.value);
    println!("\nExplanation:");
    println!("  The input '25/12/2024' is split on '/' separators.");
    println!("  With DMY order, 25 is day, 12 is month, 2024 is year.");
}
