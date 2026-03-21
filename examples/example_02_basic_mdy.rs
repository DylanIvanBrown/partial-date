//! Example 2: Basic MDY (Month-Day-Year) Format
//! Run with: `cargo run --example example_02_basic_mdy`

use partial_date::extract::extract;
use partial_date::models::*;

fn main() {
    println!("Example 2: Basic MDY Format");
    println!("==========================\n");

    println!("Input: '12/25/2024'");
    println!("Config: Month-Day-Year order\n");

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

    println!("Results:");
    println!("  Day:   {:?}", result.day.value);
    println!("  Month: {:?}", result.month.number);
    println!("  Year:  {:?}", result.year.value);
    println!("\nExplanation:");
    println!("  Same input as Example 1, but different interpretation!");
    println!("  With MDY order, 12 is month, 25 is day, 2024 is year.");
    println!("  This demonstrates how ComponentOrder changes the parsing.");
}
