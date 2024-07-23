use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `Paragraphs` is an implementation of `FakeType`. It generates a paragraphs with a number of words within a certain range
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Paragraphs` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRangeElement`: This provides `Paragraphs` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Paragraphs, specifying "Japanese" as the language
/// let s = Paragraphs::new("paragraphs", "Japanese", 1, 7).unwrap();
/// let paragraphs = s.fake_apply();
/// println!("Fake paragraphs: {}", paragraphs);
/// ```
#[derive(Debug)]
pub struct Paragraphs {
    _fake_type: String,
    lang: String,
    min: usize,
    max: usize,
}

impl FakeType for Paragraphs {
    type Response = Vec<String>;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
            Language::En(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
            Language::ArSa(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
            Language::FrFr(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
            Language::PtBr(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
            Language::ZhCn(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
            Language::ZhTw(l) => lorem::raw::Paragraphs(l, self.min..self.max).fake(),
        }
    }

    fn to_value(&self) -> Value {
        let array = self.fake_apply();
        Value::Array(array.iter().map(
            |word| Value::String(word.to_string())
        ).collect())
    }
}

impl FakeWithRangeElement for Paragraphs {
    fn new(_fake_type: String, lang: String, min: usize, max: usize) -> Result<Self> {
        match (min, max) {
            (min, max) if min >= max => {
                Err(anyhow!("fake_type: paragraphs, please setting 0 <= min, 0 <= max, min < max"))
            }
            _ => {
                Ok(Self { _fake_type, lang, min, max })
            }
        }
    }
}

impl From<Paragraphs> for FakeDefinitionElement {
    fn from(value: Paragraphs) -> Self {
        FakeDefinitionElement::Paragraphs(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Paragraphs;
    use crate::fake::fake_type::{FakeType, FakeWithRangeElement};

    #[test]
    fn test_paragraphs_fake_apply() {
        let s = Paragraphs::new("paragraphs".to_string(), "English".to_string(), 1, 7).unwrap();
        let paragraphs = s.fake_apply();

        assert!(paragraphs.len() >= 1 && paragraphs.len() <= 7, "The number of paragraph in the generated paragraphs should be within the range");
    }

    #[test]
    fn test_paragraphs_new() {
        let s = Paragraphs::new("paragraphs".to_string(), "English".to_string(), 1, 7);

        assert!(s.is_ok());
    }

    #[test]
    fn test_paragraphs_new_fail() {
        let s = Paragraphs::new("paragraphs".to_string(), "English".to_string(), 7, 1);

        assert!(s.is_err());
    }
}