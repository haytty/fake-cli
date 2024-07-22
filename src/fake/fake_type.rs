use serde_json::Value;

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