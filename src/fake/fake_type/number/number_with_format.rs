use fake::Fake;
use fake::faker::number;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeType, FakeWithFormatElement};
use crate::fake::lang::{get_language, Language};

/// `NumberWithFormat` is an implementation of `FakeType`. It generates a number with a specific format
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `NumberWithFormat` with the `fake_apply` and `to_value` methods.
/// * `FakeWithFormatElement`: This provides `NumberWithFormat` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of NumberWithFormat, specifying "Japanese" as the language and a specific format
/// let n = NumberWithFormat::new("number_with_format", "Japanese", "###-###".to_string());
/// let number = n.fake_apply();
/// println!("Fake number with format: {}", number);
/// ```
#[derive(Debug)]
pub struct NumberWithFormat {
    _fake_type: String,
    lang: String,
    format: String,
}

impl FakeType for NumberWithFormat {
    type Response = String;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::En(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::ArSa(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::FrFr(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::PtBr(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::ZhCn(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::ZhTw(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeWithFormatElement for NumberWithFormat {
    fn new(_fake_type: String, lang: String, format: String) -> Self {
        Self { _fake_type, lang, format }
    }
}

impl From<NumberWithFormat> for FakeDefinitionElement {
    fn from(value: NumberWithFormat) -> Self {
        FakeDefinitionElement::NumberWithFormat(value)
    }
}

#[cfg(test)]
mod tests {
    use super::NumberWithFormat;
    use crate::fake::fake_type::{FakeType, FakeWithFormatElement};

    #[test]
    fn test_number_with_format_fake_apply() {
        let n = NumberWithFormat::new("number_with_format".to_string(), "English".to_string(), "###-###".to_string());
        let number = n.fake_apply();

        assert!(!number.is_empty(), "Generated number with format should not be empty");
    }

    #[test]
    fn test_number_with_format_new() {
        let n = NumberWithFormat::new("number_with_format".to_string(), "English".to_string(), "###-###".to_string());
        assert_eq!(n._fake_type, "number_with_format");
        assert_eq!(n.lang, "English");
        assert_eq!(n.format, "###-###");
    }
}