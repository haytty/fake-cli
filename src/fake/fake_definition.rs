use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use serde_json::{Value};
use crate::fake::fake_definition_element::FakeDefinitionElement;

#[derive(Debug)]
pub struct FakeDefinition(BTreeMap<String, FakeDefinitionElement>);

impl FakeDefinition {
    pub fn from_json(fake_definition_json: &Value) -> Result<Self> {
        let mut btree_map = BTreeMap::new();

        let fake_definition_element_map = match fake_definition_json {
            Value::Object(map) => Some(map),
            _ => None
        }.ok_or(anyhow!("undefined fake_definition_json"))?;

        for (fake_definition_element_key, fake_definition_element_value) in fake_definition_element_map {
            let fake_definition_element = FakeDefinitionElement::generate(fake_definition_element_value)?;
            btree_map.insert(fake_definition_element_key.clone(), fake_definition_element);
        }

        Ok(Self(btree_map))
    }

    pub fn to_value(&self) -> Value {
        let mut map = serde_json::Map::new();
        for (k, fake_definition_element) in &self.0 {
            map.insert(k.clone(), fake_definition_element.to_value());
        }

        Value::Object(map)
    }
}
