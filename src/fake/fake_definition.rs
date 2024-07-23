use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use serde_json::{Value};
use crate::fake::fake_definition_element::FakeDefinitionElement;

/// `FakeDefinition` is a container for a collection of `FakeDefinitionElement`s.
/// It can be constructed from JSON and provides the ability to convert back to a `Value`.
///
/// # Example
///
/// ```
/// // Create a new instance of FakeDefinition from JSON
/// let fd = FakeDefinition::from_json(&Value::Object(map!{
///     "name" => Value::String("John Doe".to_string()),
/// })).unwrap();
/// let fd_value = fd.to_value();
/// println!("Fake Definition value: {:?}", fd_value);
/// ```
#[derive(Debug)]
pub struct FakeDefinition(BTreeMap<String, FakeDefinitionElement>);

impl FakeDefinition {
    pub fn from_json(fake_definition_json: &Value) -> Result<Self> {
        let mut btree_map = BTreeMap::new();

        let fake_definition_element_map = match fake_definition_json {
            Value::Object(map) => Ok(map),
            Value::Array(_) => Err(anyhow!("INVALID JSON FORMAT: fake_definition_json should be map format. If you want to generate multiple data, please use the count option")),
            _ => Err(anyhow!("INVALID JSON FORMAT: undefined fake_definition_json"))
        }?;

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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::FakeDefinition;
    use crate::fake::fake_definition_element::FakeDefinitionElement;
    use serde_json::{Value};
    use crate::fake::fake_type::constant::Constant;

    #[test]
    fn test_fake_definition_from_json() {
        let mut fake_definition_json = serde_json::Map::new();

        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String("word".to_string()));
        fake_definition_element.insert("lang".to_string(), Value::String("JA_JP".to_string()));

        fake_definition_json.insert("example_word".to_string(), Value::from(fake_definition_element));
        let fd = FakeDefinition::from_json(&Value::Object(fake_definition_json));

        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_from_json_missing_fake_type() {
        let mut fake_definition_json = serde_json::Map::new();

        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String("undefined_type".to_string()));
        fake_definition_element.insert("lang".to_string(), Value::String("JA_JP".to_string()));
        fake_definition_json.insert("example_word".to_string(), Value::from(fake_definition_element));

        let fd = FakeDefinition::from_json(&Value::Object(fake_definition_json));

        assert!(fd.is_err(), "Should return an error for an undefined fake type");
    }

    #[test]
    fn test_fake_definition_to_value() {
        let mut fd = BTreeMap::new();
        fd.insert("name".to_string(), FakeDefinitionElement::Constant(Constant::new("constant".to_string(), Value::String("John Doe".to_string()))));

        let fd_value = FakeDefinition(fd).to_value();

        assert!(fd_value.is_object(), "Generated value should be an object");
    }
}