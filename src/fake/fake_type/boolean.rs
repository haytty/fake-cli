use fake::Fake;
use fake::faker::boolean::raw;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeType, FakeWithRatioElement};
use crate::fake::lang::{get_language, Language};

/// `Boolean` is an implementation of `FakeType`. It generates a boolean value 
/// that varies according to the specified language and ratio.
///
/// # Attributes
///
/// * `FakeType`: This provides `Boolean` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRatioElement`: This provides `Boolean` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Boolean, specifying "English" as the language and a ratio of 30
/// let b = Boolean::new("boolean", "English", 30);
/// let boolean = b.fake_apply();
/// println!("Fake boolean: {}", boolean);
/// ```
#[derive(Debug)]
pub struct Boolean {
    _fake_type: String,
    lang: String,
    ratio: u8,
}

impl FakeType for Boolean {
    type Response = bool;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => raw::Boolean(l, self.ratio).fake(),
            Language::En(l) => raw::Boolean(l, self.ratio).fake(),
            Language::ArSa(l) => raw::Boolean(l, self.ratio).fake(),
            Language::FrFr(l) => raw::Boolean(l, self.ratio).fake(),
            Language::PtBr(l) => raw::Boolean(l, self.ratio).fake(),
            Language::ZhCn(l) => raw::Boolean(l, self.ratio).fake(),
            Language::ZhTw(l) => raw::Boolean(l, self.ratio).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::Bool(self.fake_apply())
    }
}

impl FakeWithRatioElement for Boolean {
    fn new(_fake_type: String, lang: String, ratio: u8) -> Self {
        Self { _fake_type, lang, ratio }
    }
}

impl From<Boolean> for FakeDefinitionElement {
    fn from(value: Boolean) -> Self {
        FakeDefinitionElement::Boolean(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;
    use crate::fake::fake_type::{FakeType, FakeWithRatioElement};

    #[test]
    fn test_boolean_fake_apply() {
        let b = Boolean::new("boolean".to_string(), "English".to_string(), 30);
        let boolean_value = b.fake_apply();

        assert!(boolean_value == true || boolean_value == false, "Generated value should be a boolean");
    }

    #[test]
    fn test_boolean_new() {
        let b = Boolean::new("boolean".to_string(), "English".to_string(), 30);

        assert_eq!(b._fake_type, "boolean");
        assert_eq!(b.lang, "English");
        assert_eq!(b.ratio, 30);
    }
}