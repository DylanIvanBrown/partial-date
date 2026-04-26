//! A simple console example that lets the user pick from a few different
//! scenarios that have set configurations that are relevant to the scenario.
//! This is meant to be a quick way to see how different configurations can be
//! used in different contexts within the same project.
//! Run with: `cargo run --example console_entry_config`

use std::io::{self, BufRead, Write};

use partial_date::models::{IsExpected, Range, WindowRange};

fn main() {
    println!("Partial Date Extraction Examples");
    println!("===============================\n");

    println!("Welcome to this interactive console example of how the library can be used!");

    //TODO: Create a loop in the console that allows the user to select the scenario they want and then enter dates and see what the partial date library parses them as. Then allows the user to exit that loop by typing exit and selecting another date format to test. If they type quit then it ends the program.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    'scenario: loop {
        println!("Select a scenario:");
        println!("  1) StrictDMY");
        println!("  2) AllHistoricalDates");
        println!("  3) IndustrialRevolutionDates");
        println!("  4) ChildrenBirthdays");
        println!("  (quit to exit)");
        print!("> ");
        stdout.flush().unwrap();
        let mut choice = String::new();
        stdin.lock().read_line(&mut choice).unwrap();
        let config_preset = match choice.trim() {
            "1" => PreDefinedConfigs::StrictDMY,
            "2" => PreDefinedConfigs::AllHistoricalDates,
            "3" => PreDefinedConfigs::IndustrialRevolutionDates,
            "4" => PreDefinedConfigs::ChildrenBirthdays,
            "quit" => break 'scenario,
            _ => { println!("Unknown option.\n"); continue 'scenario; }
        };
        loop {
            print!("Enter a date (or 'back' / 'quit'): ");
            stdout.flush().unwrap();
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            match input.trim() {
                "quit" => break 'scenario,
                "back" => break,          // back to scenario selection
                text => {
                    let result = partial_date::extract::extract(
                        partial_date::models::Input {
                            utterance: text.to_string(),
                            config: Some(config_preset.get_config()),
                        }
                    );
                    println!("  Day:   {:?}", result.day.value);
                    println!("  Month: {:?}", result.month.number);
                    println!("  Year:  {:?}\n", result.year.value);
                }
            }
        }
    }
    println!("Goodbye!");
}

pub enum PreDefinedConfigs {
    StrictDMY,
    AllHistoricalDates,
    IndustrialRevolutionDates,
    ChildrenBirthdays,
}

impl PreDefinedConfigs {
    pub fn get_config(&self) -> partial_date::models::Config {
        match self {
            PreDefinedConfigs::StrictDMY => partial_date::models::Config {
                component_order: partial_date::models::ComponentOrder {
                    first: partial_date::models::DateComponent::Day,
                    second: partial_date::models::DateComponent::Month,
                    third: partial_date::models::DateComponent::Year,
                },
                day: partial_date::models::DayConfig {
                    min: 1,
                    max: 31,
                    default: None,
                    expected: IsExpected::Yes,
                },
                month: partial_date::models::MonthConfig {
                    default: None,
                    expected: IsExpected::Yes,
                    min: 1,
                    max: 12,
                },
                year: partial_date::models::YearConfig {
                    min: 1,
                    max: 3000,
                    default: None,
                    expected: IsExpected::Yes,
                    two_digit_expansion: partial_date::models::TwoDigitYearExpansion::Literal,
                    single_digit_year_expansion: false,
                },
                letter_o_substitution: false,
                ..Default::default()
            },
            // Wide ranging historical dates that cannot use 2 digit year
            // expansion as there is no way to know which centuries the dates
            // will be from. This is an example of how the config can be used to
            // make minimal assumptions about the data when there is a lot of
            // uncertainty.
            PreDefinedConfigs::AllHistoricalDates => partial_date::models::Config {
                year: partial_date::models::YearConfig {
                    min: 1,
                    max: 3000,
                    default: None,
                    expected: IsExpected::Yes,
                    two_digit_expansion: partial_date::models::TwoDigitYearExpansion::Literal,
                    single_digit_year_expansion: false,
                },
                ..Default::default()
            },
            PreDefinedConfigs::IndustrialRevolutionDates => partial_date::models::Config {
                // Constraining the year range to dates in that period and
                // allowing a sliding window for 2-digit years to select years
                // within that range. This is an example of how the config can
                // be used to make assumptions about the data based on the
                // context of the project.
                year: partial_date::models::YearConfig {
                    min: 1760,
                    max: 1840,
                    default: None,
                    two_digit_expansion: partial_date::models::TwoDigitYearExpansion::SlidingWindow(WindowRange {
                        lower_range:  Range{min: 1800, max: 1850},
                        upper_range: Range{min: 1750, max: 1800},
                    }),
                    single_digit_year_expansion: false,
                    expected: partial_date::models::IsExpected::Yes,
                },
                // A component order that matches the common formats for Great
                // Britain where the industrial revolution largely took place
                component_order: partial_date::models::ComponentOrder {
                    first: partial_date::models::DateComponent::Day,
                    second: partial_date::models::DateComponent::Month,
                    third: partial_date::models::DateComponent::Year,
                },
                ..Default::default()
            },
            // Config for children under 18, which requires that their birth
            // years are in the 2000s and not in the future. Uses the shorthand
            // for always 2000s years that the library provides.
            PreDefinedConfigs::ChildrenBirthdays => partial_date::models::Config {
                year: partial_date::models::YearConfig {
                    min: 2000,
                    max: 2026,
                    default: None,
                    single_digit_year_expansion: true,
                    two_digit_expansion: partial_date::models::TwoDigitYearExpansion::Always2000s,
                    expected: partial_date::models::IsExpected::Yes,
                },
                ..Default::default()
            },
        }
    }
}