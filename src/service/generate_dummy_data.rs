use std::collections::BTreeMap;
use anyhow::{Context, Result};
use serde_json::{Value};
use crate::faker_type::FakerType;

pub fn generate_dummy_data(json_data: &Value) -> Result<()> {
    let mut btree_map = BTreeMap::new();

    if let Value::Object(map) = &json_data {
        for (key, value) in map {
            let trimed_value = value.to_string().trim_matches('"').to_string();

            let command: FakerType = trimed_value.try_into()
                .context("Failed to convert string to Commands enum")?;

            let v = command.fake();
            btree_map.insert(key, v);
        }
    }

    let pretty_json = serde_json::to_string_pretty(&btree_map)?;
    println!("{}", pretty_json);
   
    Ok(())
}