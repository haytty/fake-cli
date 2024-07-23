use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `Words` is an implementation of `FakeType`. It generates a vector of words within a certain range
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Words` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRangeElement`: This provides `Words` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Words, specifying "Japanese" as the language
/// let w = Words::new("words", "Japanese", 1, 5).unwrap();
/// let words = w.fake_apply();
/// println!("Fake words: {:?}", words);
/// ```
#[derive(Debug)]
pub struct Words {
    _fake_type: String,
    lang: String,
    min: usize,
    max: usize,
}

impl FakeType for Words {
    type Response = Vec<String>;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => lorem::raw::Words(l, self.min..self.max).fake(),
            Language::En(l) => lorem::raw::Words(l, self.min..self.max).fake(),
            Language::ArSa(l) => lorem::raw::Words(l, self.min..self.max).fake(),
            Language::FrFr(l) => lorem::raw::Words(l, self.min..self.max).fake(),
            Language::PtBr(l) => lorem::raw::Words(l, self.min..self.max).fake(),
            Language::ZhCn(l) => lorem::raw::Words(l, self.min..self.max).fake(),
            Language::ZhTw(l) => lorem::raw::Words(l, self.min..self.max).fake(),
        }
    }

    fn to_value(&self) -> Value {
        let array = self.fake_apply();
        Value::Array(array.iter().map(
            |word| Value::String(word.to_string())
        ).collect())
    }
}

impl FakeWithRangeElement for Words {
    fn new(_fake_type: String, lang: String, min: usize, max: usize) -> Result<Self> {
        match (min, max) {
            (min, max) if min >= max => {
                Err(anyhow!("fake_type: words, please setting 0 <= min, 0 <= max, min < max"))
            }
            _ => {
                Ok(Self { _fake_type, lang, min, max })
            }
        }
    }
}

impl From<Words> for FakeDefinitionElement {
    fn from(value: Words) -> Self {
        FakeDefinitionElement::Words(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Words;
    use crate::fake::fake_type::{FakeType, FakeWithRangeElement};

    #[test]
    fn test_words_fake_apply() {
        let w = Words::new("words".to_string(), "English".to_string(), 1, 5).unwrap();
        let words = w.fake_apply();

        assert!(words.len() >= 1 && words.len() <= 5, "The number of generated words should be within the range");
        assert!(!words.iter().any(|word| word.is_empty()), "No generated word should be empty");
    }

    #[test]
    fn test_words_new() {
        let w = Words::new("words".to_string(), "English".to_string(), 1, 5);

        assert!(w.is_ok());
    }

    #[test]
    fn test_words_new_fail() {
        let w = Words::new("words".to_string(), "English".to_string(), 5, 1);

        assert!(w.is_err());
    }
}