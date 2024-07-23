use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `SafeEmail` is an implementation of `FakeType`. It generates a safe_email that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `SafeEmail` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `SafeEmail` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of SafeEmail, specifying "Japanese" as the language
/// let w = SafeEmail::new("safe_email", "Japanese");
/// let safe_email = w.fake_apply();
/// println!("Fake safe_email: {}", safe_email);
/// ```
#[derive(Debug)]
pub struct SafeEmail {
    _fake_type: String,
    lang: String,
}

impl FakeType for SafeEmail {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::SafeEmail(l).fake(),
            Language::En(l) => internet::raw::SafeEmail(l).fake(),
            Language::ArSa(l) => internet::raw::SafeEmail(l).fake(),
            Language::FrFr(l) => internet::raw::SafeEmail(l).fake(),
            Language::PtBr(l) => internet::raw::SafeEmail(l).fake(),
            Language::ZhCn(l) => internet::raw::SafeEmail(l).fake(),
            Language::ZhTw(l) => internet::raw::SafeEmail(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for SafeEmail {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<SafeEmail> for FakeDefinitionElement {
    fn from(value: SafeEmail) -> Self {
        FakeDefinitionElement::SafeEmail(value)
    }
}

#[cfg(test)]
mod tests {
    use super::SafeEmail;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_safe_email_fake_apply() {
        let w = SafeEmail::new("safe_email".to_string(), "English".to_string());
        let safe_email = w.fake_apply();
        assert!(!safe_email.is_empty(), "Generated safe_email should not be empty");
    }

    #[test]
    fn test_safe_email_new() {
        let w = SafeEmail::new("safe_email".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "safe_email");
        assert_eq!(w.lang, "English");
    }
}