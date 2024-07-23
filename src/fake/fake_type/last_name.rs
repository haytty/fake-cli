use fake::Fake;
use fake::faker::{name};
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

#[derive(Debug)]
pub struct LastName {
    _fake_type: String,
    lang: String,
}

impl FakeType for LastName {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => name::raw::LastName(l).fake(),
            Language::En(l) => name::raw::LastName(l).fake(),
            Language::ArSa(l) => name::raw::LastName(l).fake(),
            Language::FrFr(l) => name::raw::LastName(l).fake(),
            Language::PtBr(l) => name::raw::LastName(l).fake(),
            Language::ZhCn(l) => name::raw::LastName(l).fake(),
            Language::ZhTw(l) => name::raw::LastName(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for LastName {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<LastName> for FakeDefinitionElement {
    fn from(value: LastName) -> Self {
        FakeDefinitionElement::LastName(value)
    }
}
