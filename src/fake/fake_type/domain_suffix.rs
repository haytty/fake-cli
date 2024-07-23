use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `DomainSuffix` is an implementation of `FakeType`. It generates a domain_suffix that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `DomainSuffix` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `DomainSuffix` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of DomainSuffix, specifying "Japanese" as the language
/// let w = DomainSuffix::new("domain_suffix", "Japanese");
/// let domain_suffix = w.fake_apply();
/// println!("Fake domain_suffix: {}", domain_suffix);
/// ```
#[derive(Debug)]
pub struct DomainSuffix {
    _fake_type: String,
    lang: String,
}

impl FakeType for DomainSuffix {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::DomainSuffix(l).fake(),
            Language::En(l) => internet::raw::DomainSuffix(l).fake(),
            Language::ArSa(l) => internet::raw::DomainSuffix(l).fake(),
            Language::FrFr(l) => internet::raw::DomainSuffix(l).fake(),
            Language::PtBr(l) => internet::raw::DomainSuffix(l).fake(),
            Language::ZhCn(l) => internet::raw::DomainSuffix(l).fake(),
            Language::ZhTw(l) => internet::raw::DomainSuffix(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for DomainSuffix {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<DomainSuffix> for FakeDefinitionElement {
    fn from(value: DomainSuffix) -> Self {
        FakeDefinitionElement::DomainSuffix(value)
    }
}

#[cfg(test)]
mod tests {
    use super::DomainSuffix;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_domain_suffix_fake_apply() {
        let w = DomainSuffix::new("domain_suffix".to_string(), "English".to_string());
        let domain_suffix = w.fake_apply();
        assert!(!domain_suffix.is_empty(), "Generated domain_suffix should not be empty");
    }

    #[test]
    fn test_domain_suffix_new() {
        let w = DomainSuffix::new("domain_suffix".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "domain_suffix");
        assert_eq!(w.lang, "English");
    }
}