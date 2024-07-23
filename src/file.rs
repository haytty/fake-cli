use std::fs;
use std::path::Path;
use serde_json::Value;
use anyhow::{Result};

/// Takes a file path as input and returns the parsed JSON as a `Value`.
///
/// The `path` argument can be any type that implements the `AsRef<Path>` trait,
/// which means it can be a `String`, a `&str`, or a `PathBuf`.
///
/// Initially, the file content is read as a string. Then the string is parsed
/// into a `serde_json::Value` using the `serde_json::from_str` function.
///
/// If the file cannot be read or the content cannot be parsed as JSON, an error is returned
/// wrapped in the `anyhow::Result` type.
pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let content = fs::read_to_string(path)?;
    let json_data: Value = serde_json::from_str(&content)?;
    Ok(json_data)
}