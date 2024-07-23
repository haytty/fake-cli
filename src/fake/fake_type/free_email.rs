use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `FreeEmail` is an implementation of `FakeType`. It generates a free_email that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `FreeEmail` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `FreeEmail` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of FreeEmail, specifying "Japanese" as the language
/// let w = FreeEmail::new("free_email", "Japanese");
/// let free_email = w.fake_apply();
/// println!("Fake free_email: {}", free_email);
/// ```
#[derive(Debug)]
pub struct FreeEmail {
    _fake_type: String,
    lang: String,
}

impl FakeType for FreeEmail {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::FreeEmail(l).fake(),
            Language::En(l) => internet::raw::FreeEmail(l).fake(),
            Language::ArSa(l) => internet::raw::FreeEmail(l).fake(),
            Language::FrFr(l) => internet::raw::FreeEmail(l).fake(),
            Language::PtBr(l) => internet::raw::FreeEmail(l).fake(),
            Language::ZhCn(l) => internet::raw::FreeEmail(l).fake(),
            Language::ZhTw(l) => internet::raw::FreeEmail(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for FreeEmail {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<FreeEmail> for FakeDefinitionElement {
    fn from(value: FreeEmail) -> Self {
        FakeDefinitionElement::FreeEmail(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FreeEmail;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_free_email_fake_apply() {
        let w = FreeEmail::new("free_email".to_string(), "English".to_string());
        let free_email = w.fake_apply();
        assert!(!free_email.is_empty(), "Generated free_email should not be empty");
    }

    #[test]
    fn test_free_email_new() {
        let w = FreeEmail::new("free_email".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "free_email");
        assert_eq!(w.lang, "English");
    }
}