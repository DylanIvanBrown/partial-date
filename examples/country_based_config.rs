//! An example with country based config to use the correct date formatting for users from different countries.
//! Run with: `cargo run --example country_based_config`

use partial_date::models::{self, ComponentOrder, Config, DateComponent};

fn main() {
    println!("Example: Country-Based Config");
    println!("============================\n");

    let south_african_user_1 = User {
        name: "Albie Sachs".to_string(),
        date_input: "13/10/94".to_string(),
        country: Country::SouthAfrica,
    };

    let south_african_user_2 = User {
        name: "Robert Sobukwe".to_string(),
        date_input: "02/06".to_string(),
        country: Country::SouthAfrica,
    };

    let liberian_user = User {
        name: "Leymah Gbowee".to_string(),
        date_input: "02/12/24".to_string(),
        country: Country::Liberia,
    };

    let liberian_user_2 = User {
        name: "George Weah".to_string(),
        date_input: "02/06".to_string(),
        country: Country::Liberia,
    };

    let all_users = vec![
        south_african_user_1,
        south_african_user_2,
        liberian_user,
        liberian_user_2,
    ];

    for user in all_users {
        // Matching country to config allows us to cleanly parse the different
        // users with their appropriate date configs
        let config = user.country.get_config();

        let input = models::Input {
            utterance: user.date_input.clone(),
            config: Some(config),
        };

        let result = partial_date::extract::extract(input);

        println!(
            "User: {} from {:?} input: '{}'",
            user.name, user.country, user.date_input
        );
        println!("  Parsed Day:   {:?}", result.day.value);
        println!("  Parsed Month: {:?}", result.month.number);
        println!("  Parsed Year:  {:?}\n", result.year.value);
    }
}

// A simple user struct to hold the date input and country information
struct User {
    name: String,
    date_input: String,
    country: Country,
}

// Enum to represent different countries with different date formats
#[derive(Debug)]
pub enum Country {
    SouthAfrica,
    Liberia,
}

// Implement a method to get the appropriate config for each country
impl Country {
    pub fn get_config(&self) -> Config {
        match self {
            Country::SouthAfrica => Config {
                component_order: ComponentOrder {
                    first: DateComponent::Day,
                    second: DateComponent::Month,
                    third: DateComponent::Year,
                },
                ..Default::default()
            },
            Country::Liberia => Config {
                component_order: ComponentOrder {
                    first: DateComponent::Month,
                    second: DateComponent::Day,
                    third: DateComponent::Year,
                },
                ..Default::default()
            },
        }
    }
}
