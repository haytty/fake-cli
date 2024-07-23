use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::FakeType;

#[derive(Debug)]
pub struct Array {
    _fake_type: String,
    count: usize,
    fake_definition_element: Box<FakeDefinitionElement>,
}

impl FakeType for Array {
    type Response = Vec<Value>;

    fn fake_apply(&self) -> Self::Response {
        let mut array = Vec::new();

        for _ in 0..self.count {
            let fake_definition_value = self.fake_definition_element.to_value();
            array.push(fake_definition_value.clone());
        }

        array
    }

    fn to_value(&self) -> Value {
        Value::Array(self.fake_apply())
    }
}

impl Array {
    pub fn new(_fake_type: String, count: usize, fake_definition_element: Box<FakeDefinitionElement>) -> Self {
        Self { _fake_type, count, fake_definition_element }
    }
}

impl From<Array> for FakeDefinitionElement {
    fn from(value: Array) -> Self {
        FakeDefinitionElement::Array(value)
    }
}
