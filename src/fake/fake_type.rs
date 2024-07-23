use serde_json::Value;
use anyhow::{Result};

pub mod words;
pub mod word;
pub mod sentence;
pub mod number_with_format;
pub mod map;
pub mod last_name;
pub mod digit;
pub mod boolean;
pub mod array;
pub mod constant;
pub mod sentences;
pub mod paragraph;
pub mod paragraphs;
pub mod first_name;
pub mod title;
pub mod suffix;
pub mod name;
pub mod name_with_title;
pub mod free_email_provider;
pub mod domain_suffix;
pub mod free_email;
pub mod safe_email;
pub mod username;
pub mod ip_v4;
pub mod ip_v6;
pub mod ip;
pub mod mac_address;
pub mod user_agent;
pub mod password;

/// The `FakeType` trait is used for types that can behave as fake data generators.
///
/// The `fake_apply` method is where the fake data generation happens.
/// The `to_value` method is used to convert the generated fake data to a `Value` for further operations.
pub trait FakeType {
    type Response;

    /// Generates the fake data.
    fn fake_apply(&self) -> Self::Response;

    /// Converts the generated fake data to a `Value`.
    fn to_value(&self) -> Value;
}

/// The `FakeElement` trait is used for types that represent an element of fake data.
///
/// The `new` method is used to create a new instance of the implementing type with the given `fake_type` and `lang`.
pub trait FakeElement {
    fn new(fake_type: String, lang: String) -> Self
    where
        Self: Sized;
}

/// The `FakeWithRatioElement` trait is used for types that represent an element of fake data
/// that can be weighted with a given ratio.
///
/// The `new` method is used to create a new instance of the implementing type with the given `fake_type`,
/// `lang`, and `ratio`.
pub trait FakeWithRatioElement {
    fn new(fake_type: String, lang: String, ratio: u8) -> Self
    where
        Self: Sized;
}

/// The `FakeWithFormatElement` trait is used for types that represent an element of fake data
/// that can be formatted in a specific way.
///
/// The `new` method is used to create a new instance of the implementing type with the given `fake_type`,
/// `lang`, and `format`.
pub trait FakeWithFormatElement {
    fn new(fake_type: String, lang: String, format: String) -> Self
    where
        Self: Sized;
}

/// The `FakeWithRangeElement` trait is used for types that represent an element of fake data
/// that should fall within a specific range.
///
/// The `new` method is used to create a new instance of the implementing type with the given `fake_type`,
/// `lang`, `min`, and `max`. It returns a `Result` to handle cases where the provided range is invalid.
pub trait FakeWithRangeElement {
    fn new(fake_type: String, lang: String, min: usize, max: usize) -> Result<Self>
    where
        Self: Sized;
}