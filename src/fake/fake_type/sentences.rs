use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `Sentences` is an implementation of `FakeType`. It generates a sentences with a number of words within a certain range
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Sentences` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRangeElement`: This provides `Sentences` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Sentences, specifying "Japanese" as the language
/// let s = Sentences::new("sentences", "Japanese", 1, 7).unwrap();
/// let sentences = s.fake_apply();
/// println!("Fake sentences: {}", sentences);
/// ```
#[derive(Debug)]
pub struct Sentences {
    _fake_type: String,
    lang: String,
    min: usize,
    max: usize,
}

impl FakeType for Sentences {
    type Response = Vec<String>;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
            Language::En(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
            Language::ArSa(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
            Language::FrFr(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
            Language::PtBr(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
            Language::ZhCn(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
            Language::ZhTw(l) => lorem::raw::Sentences(l, self.min..self.max).fake(),
        }
    }

    fn to_value(&self) -> Value {
        let array = self.fake_apply();
        Value::Array(array.iter().map(
            |word| Value::String(word.to_string())
        ).collect())
    }
}

impl FakeWithRangeElement for Sentences {
    fn new(_fake_type: String, lang: String, min: usize, max: usize) -> Result<Self> {
        match (min, max) {
            (min, max) if min >= max => {
                Err(anyhow!("fake_type: sentences, please setting 0 <= min, 0 <= max, min < max"))
            }
            _ => {
                Ok(Self { _fake_type, lang, min, max })
            }
        }
    }
}

impl From<Sentences> for FakeDefinitionElement {
    fn from(value: Sentences) -> Self {
        FakeDefinitionElement::Sentences(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fake_apply() {
        let sentences = Sentences::new("sentences".to_string(), "English".to_string(), 1, 7).unwrap();
        let fake_sentences = sentences.fake_apply();

        assert!(fake_sentences.len() >= 1 && fake_sentences.len() <= 7, "The number of sentence in the generated sentence should be within the range");
    }

    #[test]
    fn test_new() {
        let sentences = Sentences::new("sentences".to_string(), "English".to_string(), 1, 7);
        assert!(sentences.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_new_error() {
        let sentences = Sentences::new("sentences".to_string(), "English".to_string(), 10, 1);
        assert!(sentences.is_err(), "Should return Ok for a defined fake type");
    }
}