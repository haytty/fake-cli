use fake::Fake;
use fake::faker::{name};
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `Suffix` is an implementation of `FakeType`. It generates a suffix that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Suffix` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `Suffix` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Suffix, specifying "Japanese" as the language
/// let w = Suffix::new("suffix", "Japanese");
/// let suffix = w.fake_apply();
/// println!("Fake suffix: {}", suffix);
/// ```
#[derive(Debug)]
pub struct Suffix {
    _fake_type: String,
    lang: String,
}

impl FakeType for Suffix {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => name::raw::Suffix(l).fake(),
            Language::En(l) => name::raw::Suffix(l).fake(),
            Language::ArSa(l) => name::raw::Suffix(l).fake(),
            Language::FrFr(l) => name::raw::Suffix(l).fake(),
            Language::PtBr(l) => name::raw::Suffix(l).fake(),
            Language::ZhCn(l) => name::raw::Suffix(l).fake(),
            Language::ZhTw(l) => name::raw::Suffix(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for Suffix {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Suffix> for FakeDefinitionElement {
    fn from(value: Suffix) -> Self {
        FakeDefinitionElement::Suffix(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Suffix;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_suffix_fake_apply() {
        let w = Suffix::new("suffix".to_string(), "English".to_string());
        let suffix = w.fake_apply();
        assert!(!suffix.is_empty(), "Generated suffix should not be empty");
    }

    #[test]
    fn test_suffix_new() {
        let w = Suffix::new("suffix".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "suffix");
        assert_eq!(w.lang, "English");
    }
}