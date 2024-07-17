use std::fs;
use std::path::Path;
use serde_json::Value;
use anyhow::{Result};

pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let content = fs::read_to_string(path)?;
    let json_data: Value = serde_json::from_str(&content)?;
    Ok(json_data)
}