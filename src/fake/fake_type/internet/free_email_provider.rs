use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `FreeEmailProvider` is an implementation of `FakeType`. It generates a free_email_provider that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `FreeEmailProvider` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `FreeEmailProvider` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of FreeEmailProvider, specifying "Japanese" as the language
/// let w = FreeEmailProvider::new("free_email_provider", "Japanese");
/// let free_email_provider = w.fake_apply();
/// println!("Fake free_email_provider: {}", free_email_provider);
/// ```
#[derive(Debug)]
pub struct FreeEmailProvider {
    _fake_type: String,
    lang: String,
}

impl FakeType for FreeEmailProvider {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::FreeEmailProvider(l).fake(),
            Language::En(l) => internet::raw::FreeEmailProvider(l).fake(),
            Language::ArSa(l) => internet::raw::FreeEmailProvider(l).fake(),
            Language::FrFr(l) => internet::raw::FreeEmailProvider(l).fake(),
            Language::PtBr(l) => internet::raw::FreeEmailProvider(l).fake(),
            Language::ZhCn(l) => internet::raw::FreeEmailProvider(l).fake(),
            Language::ZhTw(l) => internet::raw::FreeEmailProvider(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for FreeEmailProvider {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<FreeEmailProvider> for FakeDefinitionElement {
    fn from(value: FreeEmailProvider) -> Self {
        FakeDefinitionElement::FreeEmailProvider(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FreeEmailProvider;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_free_email_provider_fake_apply() {
        let w = FreeEmailProvider::new("free_email_provider".to_string(), "English".to_string());
        let free_email_provider = w.fake_apply();
        assert!(!free_email_provider.is_empty(), "Generated free_email_provider should not be empty");
    }

    #[test]
    fn test_free_email_provider_new() {
        let w = FreeEmailProvider::new("free_email_provider".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "free_email_provider");
        assert_eq!(w.lang, "English");
    }
}