use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `Username` is an implementation of `FakeType`. It generates a username that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Username` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `Username` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Username, specifying "Japanese" as the language
/// let w = Username::new("username", "Japanese");
/// let username = w.fake_apply();
/// println!("Fake username: {}", username);
/// ```
#[derive(Debug)]
pub struct Username {
    _fake_type: String,
    lang: String,
}

impl FakeType for Username {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::Username(l).fake(),
            Language::En(l) => internet::raw::Username(l).fake(),
            Language::ArSa(l) => internet::raw::Username(l).fake(),
            Language::FrFr(l) => internet::raw::Username(l).fake(),
            Language::PtBr(l) => internet::raw::Username(l).fake(),
            Language::ZhCn(l) => internet::raw::Username(l).fake(),
            Language::ZhTw(l) => internet::raw::Username(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for Username {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Username> for FakeDefinitionElement {
    fn from(value: Username) -> Self {
        FakeDefinitionElement::Username(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Username;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_username_fake_apply() {
        let w = Username::new("username".to_string(), "English".to_string());
        let username = w.fake_apply();
        assert!(!username.is_empty(), "Generated username should not be empty");
    }

    #[test]
    fn test_username_new() {
        let w = Username::new("username".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "username");
        assert_eq!(w.lang, "English");
    }
}