// Integration tests for the partial-date library.

#[cfg(test)]
mod config;
#[cfg(test)]
mod day;
#[cfg(test)]
mod full_date;
#[cfg(test)]
mod levenshtein;
#[cfg(test)]
mod month;
#[cfg(test)]
mod month_name;
#[cfg(test)]
mod partial_date;
#[cfg(test)]
mod tokenise;
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

    /// Build a config with a specific component order.
    pub fn config_with_order(order: ComponentOrder) -> Config {
        Config {
            component_order: order,
            ..Default::default()
        }
    }

    // ---------------------------------------------------------------------------
    // Pre-built ComponentOrder constants matching the old Format variants
    // ---------------------------------------------------------------------------

    /// Day → Month → Year (formerly `Format::DDMMYY` / `Format::DDMMYYYY`).
    pub fn order_dmy() -> ComponentOrder {
        ComponentOrder {
            first: DateComponent::Day,
            second: DateComponent::Month,
            third: DateComponent::Year,
        }
    }

    /// Month → Day → Year (formerly `Format::MMDDYY` / `Format::MMDDYYYY`).
    pub fn order_mdy() -> ComponentOrder {
        ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Day,
            third: DateComponent::Year,
        }
    }

    /// Year → Month → Day (formerly `Format::YYYYMMDD` / `Format::YYMMDD`).
    pub fn order_ymd() -> ComponentOrder {
        ComponentOrder {
            first: DateComponent::Year,
            second: DateComponent::Month,
            third: DateComponent::Day,
        }
    }

    /// Year → Day → Month (formerly `Format::YYYYDDMM` / `Format::YYDDMM`).
    pub fn order_ydm() -> ComponentOrder {
        ComponentOrder {
            first: DateComponent::Year,
            second: DateComponent::Day,
            third: DateComponent::Month,
        }
    }

    /// Month → Year → Day (formerly `Format::MMYYDD` / `Format::MMYYYYDD`).
    pub fn order_myd() -> ComponentOrder {
        ComponentOrder {
            first: DateComponent::Month,
            second: DateComponent::Year,
            third: DateComponent::Day,
        }
    }
}
