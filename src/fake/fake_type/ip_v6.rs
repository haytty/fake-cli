use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `IPv6` is an implementation of `FakeType`. It generates a ip_v6 that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `IPv6` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `IPv6` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of IPv6, specifying "Japanese" as the language
/// let w = IPv6::new("ip_v6", "Japanese");
/// let ip_v6 = w.fake_apply();
/// println!("Fake ip_v6: {}", ip_v6);
/// ```
#[derive(Debug)]
pub struct IPv6 {
    _fake_type: String,
    lang: String,
}

impl FakeType for IPv6 {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::IPv6(l).fake(),
            Language::En(l) => internet::raw::IPv6(l).fake(),
            Language::ArSa(l) => internet::raw::IPv6(l).fake(),
            Language::FrFr(l) => internet::raw::IPv6(l).fake(),
            Language::PtBr(l) => internet::raw::IPv6(l).fake(),
            Language::ZhCn(l) => internet::raw::IPv6(l).fake(),
            Language::ZhTw(l) => internet::raw::IPv6(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for IPv6 {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<IPv6> for FakeDefinitionElement {
    fn from(value: IPv6) -> Self {
        FakeDefinitionElement::IPv6(value)
    }
}

#[cfg(test)]
mod tests {
    use super::IPv6;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_ip_v6_fake_apply() {
        let w = IPv6::new("ip_v6".to_string(), "English".to_string());
        let ip_v6 = w.fake_apply();
        assert!(!ip_v6.is_empty(), "Generated ip_v6 should not be empty");
    }

    #[test]
    fn test_ip_v6_new() {
        let w = IPv6::new("ip_v6".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "ip_v6");
        assert_eq!(w.lang, "English");
    }
}