use fake::Fake;
use fake::faker::number;
use serde_json::{Number, Value};
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

#[derive(Debug)]
pub struct Digit {
    _fake_type: String,
    lang: String,
}

impl FakeType for Digit {
    type Response = u8;

    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        let digit: &str = match lang {
            Language::JaJp(l) => number::raw::Digit(l).fake(),
            Language::En(l) => number::raw::Digit(l).fake(),
            Language::ArSa(l) => number::raw::Digit(l).fake(),
            Language::FrFr(l) => number::raw::Digit(l).fake(),
            Language::PtBr(l) => number::raw::Digit(l).fake(),
            Language::ZhCn(l) => number::raw::Digit(l).fake(),
            Language::ZhTw(l) => number::raw::Digit(l).fake(),
        };

        digit.parse::<u8>().unwrap()
    }

    fn to_value(&self) -> Value {
        let number = Number::from(self.fake_apply());
        Value::Number(number)
    }
}

impl FakeElement for Digit {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<Digit> for FakeDefinitionElement {
    fn from(value: Digit) -> Self {
        FakeDefinitionElement::Digit(value)
    }
}
