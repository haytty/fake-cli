use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `Word` is an implementation of `FakeType`. It generates a word that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Word` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `Word` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Word, specifying "Japanese" as the language
/// let w = Word::new("word", "Japanese");
/// let word = w.fake_apply();
/// println!("Fake word: {}", word);
/// ```
#[derive(Debug)]
pub struct Word {
    _fake_type: String,
    lang: String,
}

impl FakeType for Word {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => lorem::raw::Word(l).fake(),
            Language::En(l) => lorem::raw::Word(l).fake(),
            Language::ArSa(l) => lorem::raw::Word(l).fake(),
            Language::FrFr(l) => lorem::raw::Word(l).fake(),
            Language::PtBr(l) => lorem::raw::Word(l).fake(),
            Language::ZhCn(l) => lorem::raw::Word(l).fake(),
            Language::ZhTw(l) => lorem::raw::Word(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for Word {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Word> for FakeDefinitionElement {
    fn from(value: Word) -> Self {
        FakeDefinitionElement::Word(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Word;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_word_fake_apply() {
        let w = Word::new("word".to_string(), "English".to_string());
        let word = w.fake_apply();
        assert!(!word.is_empty(), "Generated word should not be empty");
    }

    #[test]
    fn test_word_new() {
        let w = Word::new("word".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "word");
        assert_eq!(w.lang, "English");
    }
}