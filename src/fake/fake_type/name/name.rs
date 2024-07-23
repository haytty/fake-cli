use fake::Fake;
use fake::faker::{name};
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `Name` is an implementation of `FakeType`. It generates a name that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Name` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `Name` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Name, specifying "Japanese" as the language
/// let w = Name::new("name", "Japanese");
/// let name = w.fake_apply();
/// println!("Fake name: {}", name);
/// ```
#[derive(Debug)]
pub struct Name {
    _fake_type: String,
    lang: String,
}

impl FakeType for Name {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => name::raw::Name(l).fake(),
            Language::En(l) => name::raw::Name(l).fake(),
            Language::ArSa(l) => name::raw::Name(l).fake(),
            Language::FrFr(l) => name::raw::Name(l).fake(),
            Language::PtBr(l) => name::raw::Name(l).fake(),
            Language::ZhCn(l) => name::raw::Name(l).fake(),
            Language::ZhTw(l) => name::raw::Name(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for Name {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Name> for FakeDefinitionElement {
    fn from(value: Name) -> Self {
        FakeDefinitionElement::Name(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Name;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_name_fake_apply() {
        let w = Name::new("name".to_string(), "English".to_string());
        let name = w.fake_apply();
        assert!(!name.is_empty(), "Generated name should not be empty");
    }

    #[test]
    fn test_name_new() {
        let w = Name::new("name".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "name");
        assert_eq!(w.lang, "English");
    }
}