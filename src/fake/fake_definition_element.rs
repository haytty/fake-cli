use std::collections::{BTreeMap, VecDeque};
use serde_json::{Value};
use crate::fake::fake_type::array::Array;
use crate::fake::fake_type::boolean::Boolean;
use crate::fake::fake_type::digit::Digit;
use crate::fake::fake_type::map::Map;
use crate::fake::fake_type::number_with_format::NumberWithFormat;
use crate::fake::fake_type::sentence::Sentence;
use crate::fake::fake_type::words::Words;
use anyhow::{anyhow, Result};
use crate::fake::fake_type::{FakeElement, FakeType, FakeWithFormatElement, FakeWithRangeElement, FakeWithRatioElement};
use crate::fake::fake_type::constant::Constant;
use crate::fake::fake_type::last_name::LastName;
use crate::fake::fake_type::word::Word;

/// `FakeDefinitionElement` is an enumeration of possible elements that can be included in a `FakeDefinition`.
/// It supports several data types and includes methods for constructing a `FakeDefinitionElement` from JSON 
/// and obtaining a `Value`.
///
/// # Example
///
/// ```
/// // Create a new instance of FakeDefinitionElement from JSON
/// let fd = FakeDefinitionElement::generate(&Value::Object(map!{
///     "fake_type" => Value::String("word".to_string()),
///     "lang" => Value::String("JA_JP".to_string()),
/// })).unwrap();
/// let fd_value = fd.to_value();
/// println!("Fake definition element value: {:?}", fd_value);
/// ```
#[derive(Debug)]
pub enum FakeDefinitionElement {
    Word(Word),
    LastName(LastName),
    Words(Words),
    Boolean(Boolean),
    Digit(Digit),
    Sentence(Sentence),
    NumberWithFormat(NumberWithFormat),
    Array(Array),
    Map(Map),
    Constant(Constant),
}

impl FakeDefinitionElement {
    pub fn to_value(&self) -> Value {
        match self {
            FakeDefinitionElement::Word(data) => data.to_value(),
            FakeDefinitionElement::LastName(data) => data.to_value(),
            FakeDefinitionElement::Sentence(data) => data.to_value(),
            FakeDefinitionElement::Words(data) => data.to_value(),
            FakeDefinitionElement::Boolean(data) => data.to_value(),
            FakeDefinitionElement::Digit(data) => data.to_value(),
            FakeDefinitionElement::NumberWithFormat(data) => data.to_value(),
            FakeDefinitionElement::Array(data) => data.to_value(),
            FakeDefinitionElement::Map(data) => data.to_value(),
            FakeDefinitionElement::Constant(data) => data.to_value(),
        }
    }
}

impl FakeDefinitionElement {
    pub fn generate_element<T>(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement>
    where
        T: FakeElement + Into<FakeDefinitionElement>,
    {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: {}, lang is missing", fake_type))?;
        let lang = lang_value.as_str().ok_or(anyhow!("fake_type: {}, lang should be a string", fake_type))?;

        Ok(T::new(fake_type.to_string(), lang.to_string()).into())
    }

    pub fn generate_with_range_element<T>(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement>
    where
        T: FakeWithRangeElement + Into<FakeDefinitionElement>,
    {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: {}, lang is missing", fake_type))?;
        let min_value = fake_definition_element_setting.get("min").ok_or(anyhow!("fake_type: {}, min is missing", fake_type))?;
        let max_value = fake_definition_element_setting.get("max").ok_or(anyhow!("fake_type: {}, max is missing", fake_type))?;
        let min = min_value.as_u64().ok_or(anyhow!("fake_type: {}, min parse error. please 0 < min ", fake_type))? as usize;
        let max = max_value.as_u64().ok_or(anyhow!("fake_type: {}, max parse error. please 0 < max ", fake_type))? as usize;
        let lang = lang_value.as_str().ok_or(anyhow!("fake_type: {}, lang should be a string",fake_type))?;

        Ok(T::new(fake_type.to_string(), lang.to_string(), min, max)?.into())
    }

    pub fn generate_with_ratio_element<T>(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement>
    where
        T: FakeWithRatioElement + Into<FakeDefinitionElement>,
    {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: {}, lang is missing", fake_type))?;
        let ratio_value = fake_definition_element_setting.get("ratio").ok_or(anyhow!("fake_type: {}, ratio is missing", fake_type))?;
        let ratio = ratio_value.as_u64().ok_or(anyhow!("fake_type: boolean, ratio parse error. please input 0 to 100"))? as u8;
        let lang = lang_value.as_str().ok_or(anyhow!("fake_type: boolean, lang should be a string"))?;

        Ok(T::new(fake_type.to_string(), lang.to_string(), ratio).into())
    }

    pub fn generate_with_format_element<T>(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement>
    where
        T: FakeWithFormatElement + Into<FakeDefinitionElement>,
    {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: {}, lang is missing", fake_type))?;
        let format_value = fake_definition_element_setting.get("format").ok_or(anyhow!("fake_type: {}, format is missing", fake_type))?;
        let format = format_value.as_str().ok_or(anyhow!("fake_type: {}, format should be a string", fake_type))?;
        let lang = lang_value.as_str().ok_or(anyhow!("fake_type: {}, lang should be a string", fake_type))?;

        Ok(T::new(fake_type.to_string(), lang.to_string(), format.to_string()).into())
    }

    pub fn generate_array(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let count_value = fake_definition_element_setting.get("count").ok_or(anyhow!("fake_type: array, count is missing"))?;
        let count = count_value.as_u64().ok_or(anyhow!("fake_type: array, count parse error. please 0 < count "))? as usize;

        let mut deque = VecDeque::new();
        let exclude_conditions = vec!["count", "fake_type"];
        for (k, v) in fake_definition_element_setting {
            if !exclude_conditions.contains(&k.as_str()) {
                deque.push_back(FakeDefinitionElement::generate(v)?);
            }
        }

        let fake_definition_element = deque.pop_front().ok_or(anyhow!("fake_type: array, undefined fake_definition_element"))?;
        Ok(FakeDefinitionElement::Array(Array::new(fake_type.to_string(), count, Box::new(fake_definition_element))))
    }

    pub fn generate_map(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let mut btree_map = BTreeMap::new();

        for (k, v) in fake_definition_element_setting {
            if k.as_str().ne("fake_type") {
                btree_map.insert(k.clone(), FakeDefinitionElement::generate(v)?);
            }
        }

        if btree_map.len() > 0 {
            let m = Map::new(fake_type.to_string(), btree_map);
            Ok(FakeDefinitionElement::Map(m))
        } else {
            Err(anyhow!("fake_type: map, undefined fake_definition_element"))
        }
    }

    pub fn generate_constant(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let value = fake_definition_element_setting.get("value").ok_or(anyhow!("fake_type: {}, value is missing", fake_type))?;
        Ok(FakeDefinitionElement::Constant(Constant::new(fake_type.to_string(), value.clone())))
    }
}

impl FakeDefinitionElement {
    pub fn generate(fake_definition_element_value: &Value) -> Result<FakeDefinitionElement> {
        let fake_definition_element_setting = match fake_definition_element_value {
            Value::Object(map) => Ok(map),
            _ => Err(anyhow!("INVALID JSON FORMAT: fake_definition_element_settings is undefined"))
        }?;

        let value = fake_definition_element_setting.get("fake_type").ok_or(anyhow!("fake_definition_element_settings: fake_type is missing"))?;
        let fake_type = value.as_str().ok_or(anyhow!("fake_definition_element_settings: fake_type convert error"))?;

        let obj = match fake_type {
            "word" => FakeDefinitionElement::generate_element::<Word>(fake_definition_element_setting, fake_type)?,
            "last_name" => FakeDefinitionElement::generate_element::<LastName>(fake_definition_element_setting, fake_type)?,
            "words" => FakeDefinitionElement::generate_with_range_element::<Words>(fake_definition_element_setting, fake_type)?,
            "boolean" => FakeDefinitionElement::generate_with_ratio_element::<Boolean>(fake_definition_element_setting, fake_type)?,
            "digit" => FakeDefinitionElement::generate_element::<Digit>(fake_definition_element_setting, fake_type)?,
            "sentence" => FakeDefinitionElement::generate_with_range_element::<Sentence>(fake_definition_element_setting, fake_type)?,
            "number_with_format" => FakeDefinitionElement::generate_with_format_element::<NumberWithFormat>(fake_definition_element_setting, fake_type)?,
            "array" => FakeDefinitionElement::generate_array(fake_definition_element_setting, fake_type)?,
            "map" => FakeDefinitionElement::generate_map(fake_definition_element_setting, fake_type)?,
            "constant" => FakeDefinitionElement::generate_constant(fake_definition_element_setting, fake_type)?,
            _ => {
                Err(anyhow!("{} is missing fake_type", fake_type))?
            }
        };

        Ok(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::FakeDefinitionElement;
    use serde_json::{Value};
    use anyhow::Result;

    fn generate_element(fake_type: &str, lang: &str) -> Result<FakeDefinitionElement> {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String(fake_type.to_string()));
        fake_definition_element.insert("lang".to_string(), Value::String(lang.to_string()));
        FakeDefinitionElement::generate(&Value::from(fake_definition_element))
    }

    fn generate_element_with_range(fake_type: &str, lang: &str, min: usize, max: usize) -> Result<FakeDefinitionElement> {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String(fake_type.to_string()));
        fake_definition_element.insert("lang".to_string(), Value::String(lang.to_string()));
        fake_definition_element.insert("min".to_string(), Value::Number(serde_json::Number::from(min)));
        fake_definition_element.insert("max".to_string(), Value::Number(serde_json::Number::from(max)));
        FakeDefinitionElement::generate(&Value::from(fake_definition_element))
    }

    fn generate_element_with_ratio(fake_type: &str, lang: &str, ratio: u8) -> Result<FakeDefinitionElement> {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String(fake_type.to_string()));
        fake_definition_element.insert("lang".to_string(), Value::String(lang.to_string()));
        fake_definition_element.insert("ratio".to_string(), Value::Number(serde_json::Number::from(ratio)));
        FakeDefinitionElement::generate(&Value::from(fake_definition_element))
    }

    fn generate_element_with_format(fake_type: &str, lang: &str, format: &str) -> Result<FakeDefinitionElement> {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String(fake_type.to_string()));
        fake_definition_element.insert("lang".to_string(), Value::String(lang.to_string()));
        fake_definition_element.insert("format".to_string(), Value::String(format.to_string()));
        FakeDefinitionElement::generate(&Value::from(fake_definition_element))
    }

    fn generate_element_for_constant(fake_type: &str, value: &str) -> Result<FakeDefinitionElement> {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String(fake_type.to_string()));
        fake_definition_element.insert("value".to_string(), Value::String(value.to_string()));
        FakeDefinitionElement::generate(&Value::from(fake_definition_element))
    }

    #[test]
    fn test_fake_definition_element_generate_for_word() {
        let fd = generate_element("word", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_last_name() {
        let fd = generate_element("last_name", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_words() {
        let fd = generate_element_with_range("words", "EN", 1, 5);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_boolean() {
        let fd = generate_element_with_ratio("boolean", "EN", 50);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_digit() {
        let fd = generate_element("digit", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_sentence() {
        let fd = generate_element_with_range("sentence", "EN", 1, 3);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_number_with_format() {
        let fd = generate_element_with_format("number_with_format", "EN", "TEST ^#####");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_constant() {
        let fd = generate_element_for_constant("constant", "aaaaa");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_array() {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String("array".to_string()));
        fake_definition_element.insert("count".to_string(), Value::Number(serde_json::Number::from(1)));

        let mut child_fake_definition_element = serde_json::Map::new();
        child_fake_definition_element.insert("fake_type".to_string(), Value::String("word".to_string()));
        child_fake_definition_element.insert("lang".to_string(), Value::String("JA_JP".to_string()));

        fake_definition_element.insert("example_word".to_string(), Value::from(child_fake_definition_element));

        let fd = FakeDefinitionElement::generate(&Value::from(fake_definition_element));
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_map() {
        let mut fake_definition_element = serde_json::Map::new();
        fake_definition_element.insert("fake_type".to_string(), Value::String("map".to_string()));

        let mut child_fake_definition_element = serde_json::Map::new();
        child_fake_definition_element.insert("fake_type".to_string(), Value::String("word".to_string()));
        child_fake_definition_element.insert("lang".to_string(), Value::String("JA_JP".to_string()));

        fake_definition_element.insert("example_word".to_string(), Value::from(child_fake_definition_element));

        let fd = FakeDefinitionElement::generate(&Value::from(fake_definition_element));
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_missing_fake_type() {
        let fd = generate_element("undefined_type", "EN");
        assert!(fd.is_err(), "Should return an error for an undefined fake type");
    }
}