use fake::Fake;
use fake::faker::{name};
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `FirstName` is an implementation of `FakeType`. It generates a last name 
/// that varies according to the specified language.
///
/// # Attributes
///
/// * `FakeType`: This provides `FirstName` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `FirstName` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of FirstName, specifying "Japanese" as the language
/// let n = FirstName::new("lastname", "Japanese");
/// let first_name = n.fake_apply();
/// println!("Fake first name: {}", first_name);
/// ```
#[derive(Debug)]
pub struct FirstName {
    _fake_type: String,
    lang: String,
}

impl FakeType for FirstName {
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

impl FakeElement for FirstName {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<FirstName> for FakeDefinitionElement {
    fn from(value: FirstName) -> Self {
        FakeDefinitionElement::FirstName(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FirstName;
    use crate::fake::fake_type::{FakeType, FakeElement};

    #[test]
    fn test_last_name_fake_apply() {
        let n = FirstName::new("firstname".to_string(), "EN".to_string());
        let first_name = n.fake_apply();

        assert!(!first_name.is_empty(), "Generated last name should not be empty");
    }

    #[test]
    fn test_first_name_new() {
        let n = FirstName::new("firstname".to_string(), "English".to_string());

        assert_eq!(n._fake_type, "firstname");
        assert_eq!(n.lang, "English");
    }
}