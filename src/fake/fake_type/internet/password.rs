use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `Password` is an implementation of `FakeType`. It generates a password with a number of words within a certain range
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Password` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRangeElement`: This provides `Password` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Password, specifying "Japanese" as the language
/// let s = Password::new("password", "Japanese", 1, 7).unwrap();
/// let password = s.fake_apply();
/// println!("Fake password: {}", password);
/// ```
#[derive(Debug)]
pub struct Password {
    _fake_type: String,
    lang: String,
    min: usize,
    max: usize,
}

impl FakeType for Password {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::Password(l, self.min..self.max).fake(),
            Language::En(l) => internet::raw::Password(l, self.min..self.max).fake(),
            Language::ArSa(l) => internet::raw::Password(l, self.min..self.max).fake(),
            Language::FrFr(l) => internet::raw::Password(l, self.min..self.max).fake(),
            Language::PtBr(l) => internet::raw::Password(l, self.min..self.max).fake(),
            Language::ZhCn(l) => internet::raw::Password(l, self.min..self.max).fake(),
            Language::ZhTw(l) => internet::raw::Password(l, self.min..self.max).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeWithRangeElement for Password {
    fn new(_fake_type: String, lang: String, min: usize, max: usize) -> Result<Self> {
        match (min, max) {
            (min, max) if min >= max => {
                Err(anyhow!("fake_type: password, please setting 0 <= min, 0 <= max, min < max"))
            }
            _ => {
                Ok(Self { _fake_type, lang, min, max })
            }
        }
    }
}

impl From<Password> for FakeDefinitionElement {
    fn from(value: Password) -> Self {
        FakeDefinitionElement::Password(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Password;
    use crate::fake::fake_type::{FakeType, FakeWithRangeElement};

    #[test]
    fn test_password_fake_apply() {
        let s = Password::new("password".to_string(), "English".to_string(), 1, 7).unwrap();
        let password = s.fake_apply();

        assert!(password.split_whitespace().count() >= 1 && password.split_whitespace().count() <= 7, "The number of words in the generated password should be within the range");
        assert!(!password.is_empty(), "Generated password should not be empty");
    }

    #[test]
    fn test_password_new() {
        let s = Password::new("password".to_string(), "English".to_string(), 1, 7);

        assert!(s.is_ok());
    }

    #[test]
    fn test_password_new_fail() {
        let s = Password::new("password".to_string(), "English".to_string(), 7, 1);

        assert!(s.is_err());
    }
}