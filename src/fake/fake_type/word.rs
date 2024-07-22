use fake::Fake;
use fake::faker::lorem;
use serde_json::Value;
use crate::fake::fake_type::FakeType;
use crate::fake::lang::{get_language, Language};

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

impl Word {
    pub fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

