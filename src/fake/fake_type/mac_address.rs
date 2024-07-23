use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `MACAddress` is an implementation of `FakeType`. It generates a mac_address that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `MACAddress` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `MACAddress` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of MACAddress, specifying "Japanese" as the language
/// let w = MACAddress::new("mac_address", "Japanese");
/// let mac_address = w.fake_apply();
/// println!("Fake mac_address: {}", mac_address);
/// ```
#[derive(Debug)]
pub struct MACAddress {
    _fake_type: String,
    lang: String,
}

impl FakeType for MACAddress {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::MACAddress(l).fake(),
            Language::En(l) => internet::raw::MACAddress(l).fake(),
            Language::ArSa(l) => internet::raw::MACAddress(l).fake(),
            Language::FrFr(l) => internet::raw::MACAddress(l).fake(),
            Language::PtBr(l) => internet::raw::MACAddress(l).fake(),
            Language::ZhCn(l) => internet::raw::MACAddress(l).fake(),
            Language::ZhTw(l) => internet::raw::MACAddress(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for MACAddress {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<MACAddress> for FakeDefinitionElement {
    fn from(value: MACAddress) -> Self {
        FakeDefinitionElement::MACAddress(value)
    }
}

#[cfg(test)]
mod tests {
    use super::MACAddress;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_mac_address_fake_apply() {
        let w = MACAddress::new("mac_address".to_string(), "English".to_string());
        let mac_address = w.fake_apply();
        assert!(!mac_address.is_empty(), "Generated mac_address should not be empty");
    }

    #[test]
    fn test_mac_address_new() {
        let w = MACAddress::new("mac_address".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "mac_address");
        assert_eq!(w.lang, "English");
    }
}