use fake::Fake;
use fake::faker::boolean::raw;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeType, FakeWithRatioElement};
use crate::fake::lang::{get_language, Language};


#[derive(Debug)]
pub struct Boolean {
    _fake_type: String,
    lang: String,
    ratio: u8,
}

impl FakeType for Boolean {
    type Response = bool;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => raw::Boolean(l, self.ratio).fake(),
            Language::En(l) => raw::Boolean(l, self.ratio).fake(),
            Language::ArSa(l) => raw::Boolean(l, self.ratio).fake(),
            Language::FrFr(l) => raw::Boolean(l, self.ratio).fake(),
            Language::PtBr(l) => raw::Boolean(l, self.ratio).fake(),
            Language::ZhCn(l) => raw::Boolean(l, self.ratio).fake(),
            Language::ZhTw(l) => raw::Boolean(l, self.ratio).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::Bool(self.fake_apply())
    }
}

impl FakeWithRatioElement for Boolean {
    fn new(_fake_type: String, lang: String, ratio: u8) -> Self {
        Self { _fake_type, lang, ratio }
    }
}

impl From<Boolean> for FakeDefinitionElement {
    fn from(value: Boolean) -> Self {
        FakeDefinitionElement::Boolean(value)
    }
}
