use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `IP` is an implementation of `FakeType`. It generates a ip that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `IP` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `IP` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of IP, specifying "Japanese" as the language
/// let w = IP::new("ip", "Japanese");
/// let ip = w.fake_apply();
/// println!("Fake ip: {}", ip);
/// ```
#[derive(Debug)]
pub struct IP {
    _fake_type: String,
    lang: String,
}

impl FakeType for IP {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::IP(l).fake(),
            Language::En(l) => internet::raw::IP(l).fake(),
            Language::ArSa(l) => internet::raw::IP(l).fake(),
            Language::FrFr(l) => internet::raw::IP(l).fake(),
            Language::PtBr(l) => internet::raw::IP(l).fake(),
            Language::ZhCn(l) => internet::raw::IP(l).fake(),
            Language::ZhTw(l) => internet::raw::IP(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for IP {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<IP> for FakeDefinitionElement {
    fn from(value: IP) -> Self {
        FakeDefinitionElement::IP(value)
    }
}

#[cfg(test)]
mod tests {
    use super::IP;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_ip_fake_apply() {
        let w = IP::new("ip".to_string(), "English".to_string());
        let ip = w.fake_apply();
        assert!(!ip.is_empty(), "Generated ip should not be empty");
    }

    #[test]
    fn test_ip_new() {
        let w = IP::new("ip".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "ip");
        assert_eq!(w.lang, "English");
    }
}