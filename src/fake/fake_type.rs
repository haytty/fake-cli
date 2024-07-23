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

pub trait FakeType {
    type Response;
    fn fake_apply(&self) -> Self::Response;

    fn to_value(&self) -> Value;
}

pub trait FakeElement {
    fn new(fake_type: String, lang: String) -> Self
    where
        Self: Sized;
}

pub trait FakeWithRatioElement {
    fn new(fake_type: String, lang: String, ratio: u8) -> Self
    where
        Self: Sized;
}

pub trait FakeWithFormatElement {
    fn new(fake_type: String, lang: String, format: String) -> Self
    where
        Self: Sized;
}


pub trait FakeWithRangeElement {
    fn new(fake_type: String, lang: String, min: usize, max: usize) -> Result<Self>
    where
        Self: Sized;
}
