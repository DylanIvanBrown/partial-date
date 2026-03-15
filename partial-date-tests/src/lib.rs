// Integration tests for the partial-date library.

#[cfg(test)]
mod config;
#[cfg(test)]
mod day;
#[cfg(test)]
mod full_date;
#[cfg(test)]
mod month;
#[cfg(test)]
mod partial_date;
#[cfg(test)]
mod year;

/// Helper module for common test utilities.
#[cfg(test)]
mod helpers {
    use partial_date::models::*;

    /// Build an [`Input`] with the given utterance and no config (all defaults).
    pub fn input(utterance: &str) -> Input {
        Input {
            utterance: utterance.to_string(),
            config: None,
        }
    }

    /// Build an [`Input`] with the given utterance and a custom config.
    pub fn input_with_config(utterance: &str, config: Config) -> Input {
        Input {
            utterance: utterance.to_string(),
            config: Some(config),
        }
    }

    /// Build a config where only the day is expected.
    pub fn day_only_config() -> Config {
        Config {
            day: DayConfig {
                expected: IsExpected::Yes,
                ..Default::default()
            },
            month: MonthConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            year: YearConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Build a config where only the month is expected.
    pub fn month_only_config() -> Config {
        Config {
            day: DayConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            month: MonthConfig {
                expected: IsExpected::Yes,
                ..Default::default()
            },
            year: YearConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Build a config where only the year is expected.
    pub fn year_only_config() -> Config {
        Config {
            day: DayConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            month: MonthConfig {
                expected: IsExpected::No,
                ..Default::default()
            },
            year: YearConfig {
                expected: IsExpected::Yes,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Build a config with a specific format.
    pub fn config_with_format(format: Format) -> Config {
        Config {
            primary_format: format,
            ..Default::default()
        }
    }
}
