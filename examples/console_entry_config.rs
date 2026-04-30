//! A simple console example that lets the user pick from a few different
//! scenarios that have set configurations that are relevant to the scenario.
//! This is meant to be a quick way to see how different configurations can be
//! used in different contexts within the same project.
//! Run with: `cargo run --example console_entry_config`

use std::io::{self, BufRead, Write};

use partial_date::models::{
    Century, ComponentOrder, Config, DateComponent, DayConfig, IsExpected, MonthConfig,
    SlidingWindowPivot, TwoDigitYearExpansion, YearConfig,
};

fn main() {
    println!("Partial Date Extraction Examples");
    println!("===============================\n");

    println!("Welcome to this interactive console example of how the library can be used!");

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
            _ => {
                println!("Unknown option.\n");
                continue 'scenario;
            }
        };
        loop {
            print!("Enter a date (or 'back' / 'quit'): ");
            stdout.flush().unwrap();
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            match input.trim() {
                "quit" => break 'scenario,
                "back" => break,
                text => {
                    let result = partial_date::extract::extract(partial_date::models::Input {
                        utterance: text.to_string(),
                        config: Some(config_preset.get_config()),
                    });
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
    pub fn get_config(&self) -> Config {
        match self {
            // Strictly day-first numeric dates with all three components
            // required. Letter-O substitution is disabled since this scenario
            // expects clean numeric input only.
            PreDefinedConfigs::StrictDMY => Config::default()
                .with_day(
                    DayConfig::default()
                        .with_expected(IsExpected::Yes),
                )
                .with_month(
                    MonthConfig::default()
                        .with_expected(IsExpected::Yes)
                )
                .with_year(
                    YearConfig::default()
                        .with_range(1, 3000)
                        .with_expected(IsExpected::Yes)
                        .with_two_digit_expansion(TwoDigitYearExpansion::Literal),
                )
                .with_component_order(
                    ComponentOrder::new(
                        DateComponent::Day,
                        DateComponent::Month,
                        DateComponent::Year,
                    )
                    .unwrap(),
                )
                .with_letter_o_substitution(false),

            // Wide ranging historical dates that cannot use 2-digit year
            // expansion as there is no way to know which centuries the dates
            // will be from. Minimal assumptions about the data when there is
            // a lot of uncertainty.
            PreDefinedConfigs::AllHistoricalDates => Config::default().with_year(
                YearConfig::default()
                    .with_range(1, 3000)
                    .with_expected(IsExpected::Yes)
                    .with_two_digit_expansion(TwoDigitYearExpansion::Literal),
            ),

            // Constrains the year range to the Industrial Revolution period and
            // uses a sliding window for 2-digit years centred on 1800.
            // Component order matches Great Britain's common DD/MM/YYYY format.
            PreDefinedConfigs::IndustrialRevolutionDates => Config::default()
                .with_year(
                    YearConfig::default()
                        .with_range(1760, 1840)
                        .with_expected(IsExpected::Yes)
                        .with_two_digit_expansion(TwoDigitYearExpansion::SlidingWindow {
                            earliest_year: 1750,
                            pivot: SlidingWindowPivot::new(50),
                        }),
                )
                .with_component_order(
                    ComponentOrder::new(
                        DateComponent::Day,
                        DateComponent::Month,
                        DateComponent::Year,
                    )
                    .unwrap(),
                ),

            // Children under 18 — birth years must be in the 2000s and not in
            // the future. Always(Century(2000)) maps all 2-digit values to the
            // 2000s; the max of 2026 rejects future years.
            PreDefinedConfigs::ChildrenBirthdays => Config::default().with_year(
                YearConfig::default()
                    .with_range(2000, 2026)
                    .with_expected(IsExpected::Yes)
                    .with_two_digit_expansion(TwoDigitYearExpansion::Always(Century::new(2000)))
                    .with_single_digit_expansion(true),
            ),
        }
    }
}
