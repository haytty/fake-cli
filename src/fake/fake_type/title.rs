use fake::Fake;
use fake::faker::{name};
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `Title` is an implementation of `FakeType`. It generates a title that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Title` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `Title` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Title, specifying "Japanese" as the language
/// let w = Title::new("title", "Japanese");
/// let title = w.fake_apply();
/// println!("Fake title: {}", title);
/// ```
#[derive(Debug)]
pub struct Title {
    _fake_type: String,
    lang: String,
}

impl FakeType for Title {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => name::raw::Title(l).fake(),
            Language::En(l) => name::raw::Title(l).fake(),
            Language::ArSa(l) => name::raw::Title(l).fake(),
            Language::FrFr(l) => name::raw::Title(l).fake(),
            Language::PtBr(l) => name::raw::Title(l).fake(),
            Language::ZhCn(l) => name::raw::Title(l).fake(),
            Language::ZhTw(l) => name::raw::Title(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for Title {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Title> for FakeDefinitionElement {
    fn from(value: Title) -> Self {
        FakeDefinitionElement::Title(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Title;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_title_fake_apply() {
        let w = Title::new("title".to_string(), "English".to_string());
        let title = w.fake_apply();
        assert!(!title.is_empty(), "Generated title should not be empty");
    }

    #[test]
    fn test_title_new() {
        let w = Title::new("title".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "title");
        assert_eq!(w.lang, "English");
    }
}