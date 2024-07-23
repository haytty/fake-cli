use std::collections::{BTreeMap};
use serde_json::{Value};
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::FakeType;

/// The `Map` structure is an implementation of `FakeType`. It produces a map of elements of `FakeType`.
///
/// # Attributes
///
/// * `FakeType`: This provides `Map` with the `fake_apply` and `to_value` methods.
///
/// # Example
///
/// ```
/// use std::collections::BTreeMap;
/// // Create a new instance of Map, specifying "map" as the type
/// let m = Map::new("map".to_string(), BTreeMap::new());
/// let map = m.fake_apply();
/// println!("Fake map: {:?}", map);
/// ```
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

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::fake::fake_type::FakeType;
    use std::collections::BTreeMap;

    #[test]
    fn test_map_fake_apply() {
        let m = Map::new("map".to_string(), BTreeMap::new());
        let map = m.fake_apply();

        assert!(map.is_empty(), "Generated map should be empty when no FakeDefinitionElements are provided");
    }

    #[test]
    fn test_map_new() {
        let m = Map::new("map".to_string(), BTreeMap::new());

        assert_eq!(m._fake_type, "map");
        assert_eq!(m.map.len(), 0);
    }
}