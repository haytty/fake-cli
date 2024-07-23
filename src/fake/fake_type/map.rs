use std::collections::{BTreeMap};
use serde_json::{Value};
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::FakeType;

#[derive(Debug)]
pub struct Map {
    _fake_type: String,
    map: BTreeMap<String, FakeDefinitionElement>,
}

impl FakeType for Map {
    type Response = serde_json::Map<String, Value>;

    fn fake_apply(&self) -> Self::Response {
        let m = serde_json::Map::new();

        let result = self.map.iter().fold(m, |mut acc, (key, obj)| {
            acc.insert(key.clone(), obj.to_value());
            acc
        });

        result
    }

    fn to_value(&self) -> Value {
        Value::Object(self.fake_apply())
    }
}

impl Map {
    pub fn new(_fake_type: String, map: BTreeMap<String, FakeDefinitionElement>) -> Self {
        Self { _fake_type, map }
    }
}

impl From<Map> for FakeDefinitionElement {
    fn from(value: Map) -> Self {
        FakeDefinitionElement::Map(value)
    }
}
