use fake::Fake;
use fake::faker::number;
use serde_json::{Number, Value};
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `Digit` is an implementation of `FakeType`. It generates a single digit
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Digit` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `Digit` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Digit, specifying "Japanese" as the language
/// let d = Digit::new("digit", "Japanese");
/// let digit = d.fake_apply();
/// println!("Fake digit: {}", digit);
/// ```
#[derive(Debug)]
pub struct Digit {
    _fake_type: String,
    lang: String,
}

impl FakeType for Digit {
    type Response = u8;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        let digit: &str = match lang {
            Language::JaJp(l) => number::raw::Digit(l).fake(),
            Language::En(l) => number::raw::Digit(l).fake(),
            Language::ArSa(l) => number::raw::Digit(l).fake(),
            Language::FrFr(l) => number::raw::Digit(l).fake(),
            Language::PtBr(l) => number::raw::Digit(l).fake(),
            Language::ZhCn(l) => number::raw::Digit(l).fake(),
            Language::ZhTw(l) => number::raw::Digit(l).fake(),
        };

        digit.parse::<u8>().unwrap()
    }

    fn to_value(&self) -> Value {
        let number = Number::from(self.fake_apply());
        Value::Number(number)
    }
}

impl FakeElement for Digit {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Digit> for FakeDefinitionElement {
    fn from(value: Digit) -> Self {
        FakeDefinitionElement::Digit(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Digit;
    use crate::fake::fake_type::{FakeType, FakeElement};

    #[test]
    fn test_digit_fake_apply() {
        let d = Digit::new("digit".to_string(), "English".to_string());
        let digit = d.fake_apply();

        // Since u8 is always >= 0 no need to check "digit >= 0"
        assert!(digit <= 9, "Generated digit should be less than or equal to 9");
    }

    #[test]
    fn test_digit_new() {
        let d = Digit::new("digit".to_string(), "English".to_string());

        assert_eq!(d._fake_type, "digit");
        assert_eq!(d.lang, "English");
    }
}