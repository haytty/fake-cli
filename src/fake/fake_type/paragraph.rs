use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `Paragraph` is an implementation of `FakeType`. It generates a paragraph with a number of words within a certain range
/// that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `Paragraph` with the `fake_apply` and `to_value` methods.
/// * `FakeWithRangeElement`: This provides `Paragraph` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of Paragraph, specifying "Japanese" as the language
/// let s = Paragraph::new("paragraph", "Japanese", 1, 7).unwrap();
/// let paragraph = s.fake_apply();
/// println!("Fake paragraph: {}", paragraph);
/// ```
#[derive(Debug)]
pub struct Paragraph {
    _fake_type: String,
    lang: String,
    min: usize,
    max: usize,
}

impl FakeType for Paragraph {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
            Language::En(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
            Language::ArSa(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
            Language::FrFr(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
            Language::PtBr(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
            Language::ZhCn(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
            Language::ZhTw(l) => lorem::raw::Paragraph(l, self.min..self.max).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeWithRangeElement for Paragraph {
    fn new(_fake_type: String, lang: String, min: usize, max: usize) -> Result<Self> {
        match (min, max) {
            (min, max) if min >= max => {
                Err(anyhow!("fake_type: paragraph, please setting 0 <= min, 0 <= max, min < max"))
            }
            _ => {
                Ok(Self { _fake_type, lang, min, max })
            }
        }
    }
}

impl From<Paragraph> for FakeDefinitionElement {
    fn from(value: Paragraph) -> Self {
        FakeDefinitionElement::Paragraph(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Paragraph;
    use crate::fake::fake_type::{FakeType, FakeWithRangeElement};

    #[test]
    fn test_paragraph_fake_apply() {
        let paragraph = Paragraph::new("paragraph".to_string(), "English".to_string(), 1, 7).unwrap();
        let fake_paragraph = paragraph.fake_apply();

        let sentences: Vec<_> = fake_paragraph.split("\n").collect();
        assert!(sentences.len() >= 1 && sentences.len() <= 7, "The number of paragraph in the generated paragraphs should be within the range");
    }

    #[test]
    fn test_paragraph_new() {
        let s = Paragraph::new("paragraph".to_string(), "English".to_string(), 1, 7);

        assert!(s.is_ok());
    }

    #[test]
    fn test_paragraph_new_fail() {
        let s = Paragraph::new("paragraph".to_string(), "English".to_string(), 7, 1);

        assert!(s.is_err());
    }
}