use fake::Fake;
use fake::faker::{name};
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `LastName` is an implementation of `FakeType`. It generates a last name 
/// that varies according to the specified language.
///
/// # Attributes
///
/// * `FakeType`: This provides `LastName` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `LastName` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of LastName, specifying "Japanese" as the language
/// let ln = LastName::new("lastname", "Japanese");
/// let last_name = ln.fake_apply();
/// println!("Fake last name: {}", last_name);
/// ```
#[derive(Debug)]
pub struct LastName {
    _fake_type: String,
    lang: String,
}

impl FakeType for LastName {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => name::raw::LastName(l).fake(),
            Language::En(l) => name::raw::LastName(l).fake(),
            Language::ArSa(l) => name::raw::LastName(l).fake(),
            Language::FrFr(l) => name::raw::LastName(l).fake(),
            Language::PtBr(l) => name::raw::LastName(l).fake(),
            Language::ZhCn(l) => name::raw::LastName(l).fake(),
            Language::ZhTw(l) => name::raw::LastName(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for LastName {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<LastName> for FakeDefinitionElement {
    fn from(value: LastName) -> Self {
        FakeDefinitionElement::LastName(value)
    }
}

#[cfg(test)]
mod tests {
    use super::LastName;
    use crate::fake::fake_type::{FakeType, FakeElement};

    #[test]
    fn test_last_name_fake_apply() {
        let ln = LastName::new("lastname".to_string(), "English".to_string());
        let last_name = ln.fake_apply();

        assert!(!last_name.is_empty(), "Generated last name should not be empty");
    }

    #[test]
    fn test_last_name_new() {
        let ln = LastName::new("lastname".to_string(), "English".to_string());

        assert_eq!(ln._fake_type, "lastname");
        assert_eq!(ln.lang, "English");
    }
}