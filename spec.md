# Partial Date library

This library is intended to allow users to extract a partial date from a given user text. The difference with this library vs other date extractors is that we will support partial dates, with some config to allow users to default the other missing date values if required.

The intention of this library is to be entirely deterministic, with no ML/AI model use.

We will allow for extensability of the extraction fns to allow users to provide ML/AI or other implementations of the extractions functions if that makes sense for their use case.

This library is intended to be performant and have an absolute minimum (ideally almost none) of dependencies.

## Configurability

This library is intended to support configurability for different use cases for partial date extraction. We are taking the principle of "sensible defaults with configurability" as a cornerstone of this library to make the library easier to use across a wide range of contexts.

One common difference of context is the date format, be that DD/MM/YY, MM/DD/YYYY or so on. We want to allow the user to supply their expected format, which will determine the default format to expect in an ambiguous case like 01/01/12.

For cases like 31/12/19 we should still be able to return DD = 31, MM = 12, and YY = 19 no matter what the default format is.

We should also allow the user to provide a config for the default year. This is useful in cases where the year is obvious, based on some other known fact about the user or about the context of the question. For example if a user is asked "When did the accident happen?" and they provide " 12 June" then we should be able to provide a default date (eg. 2025) and return a partial date with that default date value, which would be 12/06/2026.

Our struct for the partial date should also allow configuration for max and min values for validation, particularly for items like year, which might have a min or max reasonable year value in different contexts. The default for year min in the library could be 0 and the max 3000 to account for cases where the dates could be historical AD dates, or dates in the future for some kind of planning case. These should be configurable by the user though.

We should also allow users to determine if they want the values to be defaulted at all in the config. Allowing `default = false` would mean we would instead return an indication that the value was not found instead.

We should allow config to exist for both the entire use of the library in the project, and on a specific use of the query to extract the partial date. In other words we will have a project config that will be used as the default config if a config for extraction is not provided in the query to extract a partial date.

## Structs

We want to maximise our type safety by having structs that avoid the use of too many primitives directly.

### Partial Date Result

For example our PartialDate result would be something like this:

```rust
pub struct PartialDate{
    day : Day,
    month: Month,
    year: Year 
}

pub struct Day{
    value: Extracted,
}

// This enum could be used for each value, Day, Month and Year
pub enum Extracted(
    Found(u8), // The value was found in the input
    NotFound, // No value could be found and no default was provided
    Defaulted(u8), // The value was not found and the default was provided
)

```

### Config

For configuration we should allow the user to provide a config, which should be in the form of a type safe struct(s).

```rust
pub struct Config{
    // Day config
    day: DayConfig,
    // Month config
    month: MonthConfig,
    // Year config
    year: YearConfig,
    /// Default format DD-MM-YYYY etc. 
    primary_format: Format,
    /// Primary separator to use for extracting date. All separators will be attempted.
    primary_separator: Separator,

}

/// Is the value expected to be provided.
/// This is used to make it clear when the user expects a value to be there or not.
/// This is important when parsing a value like 12/06 as we can use this to weight 
/// our decision about whether a day, month or year is expected.
pub enum IsExpected
{
    Yes,
    No,
    // This should be the default for IsExpected
    Maybe,
}

pub struct DayConfig {
    min: u8,
    max u8,
}

// Sensible defaults for all config structs
impl Default for DayConfig {
    fn default() -> Self{
        DayConfig{
            min: 1,
            max: 31,
        }
    }
}

pub struct MonthConfig {
    min: u8,
    max u8,
    // Do we add options for string vs not string values like June etc?
    // Generally we should handle as many formats as we can
}

// Sensible defaults for all config structs
impl Default for MonthConfig {
    fn default() -> Self{
        MonthConfig{
            min: 1,
            max: 12,
        }
    }
}

pub struct YearConfig {
    min: u8,
    max u8,
}

// Open question here as to whether we should expand these options 
// to also include formats where some of the parts are not provided.
// e.g MMYY or DDMM
pub enum Format(
    DDMMYY,
    DDMMYYYY,
    MMDDYY,
    MMDDYYYY,
    MMYYDD,
    MMYYYYDD,
    YYMMDD,
    YYDDMM,
    YYYYMMDD,
    YYYYDDMM,
    /// Provide a custom format, using the D Y and M character to represent Days, Months and Years respectively.
    Other(String),
)

/// An enum for the different separators we should expect. We will iterate through all separator types to try and extract the date
pub enum Separator {
    Dash, // -
    Space, // white space
    ForwardSlash // /
    BackSlash,// \
    Dot, // .
    Comma, // ,
    NoSeparator,// No separator provided, e.g 011224 or 1994
    Other(String)
}
```

### Input

As input we should allow the user to pass a string, as well as an optional config.

```rust
pub struct Input {
    utterance: String,
    config: Option<Config>,
    //Perhaps we can provide options for the primary data to extract, such as Day, Month or Year or any combination of them
}
```

## Result

The Result of our partial date extraction should almost always return a PartialDate. Even in the case where no date is found we should end up returning a PartialDate where all the values are `Extracted::NotFound` rather than returning an error. The only error cases should be as a result of internal errors, and perhaps we can even remove those by having a variant `Extracted::Error` that has different error types, such as `Invalid` or `Empty`.

## Trait

Should we use a trait for a PartialDateExtractor or parts of it that users can implement to use custom logic? For example having a trait for MonthExtractor that supports english and numerical representations, but allowing the user to then configure the project to use a custom implementation of the MonthExtractor that allows for different languages and representations of months to be extracted?

## Tests

We will keep tests for the library separate, so that any dependencies for tests are not required by the user of the library.

It would be ideal for us to document all the test cases we have used, so the user of the library could see the test cases that the library covers and the expected inputs and outputs in those cases.

We will create mapping for all common english versions of a month, eg "Apr" -> April.

We will also want to handle misspellings, perhaps using the levenshtein distance or ratio algorithm. We can either use a small library for this or implement the calculation ourselves in this library.

## Documentation

We should thoroughly document the library, so that the created cargo docs are easy to read, and provide clear examples on how to use the library, with different examples for different kinds of configurations and values.