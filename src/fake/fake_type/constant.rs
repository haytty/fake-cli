use fake::Fake;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeType};

#[derive(Debug)]
pub struct Constant {
    _fake_type: String,
    value: Value,
}

impl FakeType for Constant {
    type Response = Value;

    fn fake_apply(&self) -> Self::Response {
        self.value.clone()
    }

    fn to_value(&self) -> Value {
        self.fake_apply()
    }
}

impl Constant {
    pub fn new(_fake_type: String, value: Value) -> Self {
        Self { _fake_type, value }
    }
}

impl From<Constant> for FakeDefinitionElement {
    fn from(value: Constant) -> Self {
        FakeDefinitionElement::Constant(value)
    }
}
