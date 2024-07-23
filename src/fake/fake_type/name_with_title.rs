use fake::Fake;
use fake::faker::name;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `NameWithTitle` is an implementation of `FakeType`. It generates a name_with_title that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `NameWithTitle` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `NameWithTitle` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of NameWithTitle, specifying "Japanese" as the language
/// let w = NameWithTitle::new("name_with_title", "Japanese");
/// let name_with_title = w.fake_apply();
/// println!("Fake name_with_title: {}", name_with_title);
/// ```
#[derive(Debug)]
pub struct NameWithTitle {
    _fake_type: String,
    lang: String,
}

impl FakeType for NameWithTitle {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => name::raw::NameWithTitle(l).fake(),
            Language::En(l) => name::raw::NameWithTitle(l).fake(),
            Language::ArSa(l) => name::raw::NameWithTitle(l).fake(),
            Language::FrFr(l) => name::raw::NameWithTitle(l).fake(),
            Language::PtBr(l) => name::raw::NameWithTitle(l).fake(),
            Language::ZhCn(l) => name::raw::NameWithTitle(l).fake(),
            Language::ZhTw(l) => name::raw::NameWithTitle(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for NameWithTitle {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<NameWithTitle> for FakeDefinitionElement {
    fn from(value: NameWithTitle) -> Self {
        FakeDefinitionElement::NameWithTitle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::NameWithTitle;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_name_with_title_fake_apply() {
        let w = NameWithTitle::new("name_with_title".to_string(), "English".to_string());
        let name_with_title = w.fake_apply();
        assert!(!name_with_title.is_empty(), "Generated name_with_title should not be empty");
    }

    #[test]
    fn test_name_with_title_new() {
        let w = NameWithTitle::new("name_with_title".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "name_with_title");
        assert_eq!(w.lang, "English");
    }
}