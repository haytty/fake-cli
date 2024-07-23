use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `Sentence` is an implementation of `FakeType`. It generates a sentence with a number of words within a certain range
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Sentence` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRangeElement`: This provides `Sentence` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Sentence, specifying "Japanese" as the language
/// let s = Sentence::new("sentence", "Japanese", 1, 7).unwrap();
/// let sentence = s.fake_apply();
/// println!("Fake sentence: {}", sentence);
/// ```
#[derive(Debug)]
pub struct Sentence {
    _fake_type: String,
    lang: String,
    min: usize,
    max: usize,
}

impl FakeType for Sentence {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
            Language::En(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
            Language::ArSa(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
            Language::FrFr(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
            Language::PtBr(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
            Language::ZhCn(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
            Language::ZhTw(l) => lorem::raw::Sentence(l, self.min..self.max).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeWithRangeElement for Sentence {
    fn new(_fake_type: String, lang: String, min: usize, max: usize) -> Result<Self> {
        match (min, max) {
            (min, max) if min >= max => {
                Err(anyhow!("fake_type: sentence, please setting 0 <= min, 0 <= max, min < max"))
            }
            _ => {
                Ok(Self { _fake_type, lang, min, max })
            }
        }
    }
}

impl From<Sentence> for FakeDefinitionElement {
    fn from(value: Sentence) -> Self {
        FakeDefinitionElement::Sentence(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Sentence;
    use crate::fake::fake_type::{FakeType, FakeWithRangeElement};

    #[test]
    fn test_sentence_fake_apply() {
        let s = Sentence::new("sentence".to_string(), "English".to_string(), 1, 7).unwrap();
        let sentence = s.fake_apply();

        assert!(sentence.split_whitespace().count() >= 1 && sentence.split_whitespace().count() <= 7, "The number of words in the generated sentence should be within the range");
        assert!(!sentence.is_empty(), "Generated sentence should not be empty");
    }

    #[test]
    fn test_sentence_new() {
        let s = Sentence::new("sentence".to_string(), "English".to_string(), 1, 7);

        assert!(s.is_ok());
    }

    #[test]
    fn test_sentence_new_fail() {
        let s = Sentence::new("sentence".to_string(), "English".to_string(), 7, 1);

        assert!(s.is_err());
    }
}