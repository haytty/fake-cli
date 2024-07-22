use fake::Fake;
use fake::faker::number;
use serde_json::Value;
use crate::fake::fake_type::FakeType;
use crate::fake::lang::{get_language, Language};

#[derive(Debug)]
pub struct NumberWithFormat {
    _fake_type: String,
    lang: String,
    format: String,
}

impl FakeType for NumberWithFormat {
    type Response = String;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::En(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::ArSa(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::FrFr(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::PtBr(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::ZhCn(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
            Language::ZhTw(l) => number::raw::NumberWithFormat(l, self.format.as_str()).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl NumberWithFormat {
    pub fn new(_fake_type: String, lang: String, format: String) -> Self {
        Self { _fake_type, lang, format }
    }
}

