use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::{FakeType, FakeWithRangeElement};
use crate::fake::lang::{get_language, Language};
use anyhow::{Result, anyhow};
use crate::fake::fake_definition_element::FakeDefinitionElement;

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
