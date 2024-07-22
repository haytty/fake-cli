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
use crate::fake::fake_type::FakeType;
use crate::fake::fake_type::last_name::LastName;
use crate::fake::fake_type::word::Word;

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
        }
    }
}

impl FakeDefinitionElement {
    pub fn generate_word(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: word, lang is missing"))?;
        let lang = trim_value_string(&lang_value.to_string());
        Ok(FakeDefinitionElement::Word(Word::new(fake_type.to_string(), lang)))
    }

    pub fn generate_last_name(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: last_name, lang is missing"))?;
        let lang = trim_value_string(&lang_value.to_string());
        Ok(FakeDefinitionElement::LastName(LastName::new(fake_type.to_string(), lang)))
    }

    pub fn generate_words(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: words, lang is missing"))?;
        let min_value = fake_definition_element_setting.get("min").ok_or(anyhow!("fake_type: words, min is missing"))?;
        let max_value = fake_definition_element_setting.get("max").ok_or(anyhow!("fake_type: words, max is missing"))?;
        let min = trim_value_string(&min_value.to_string()).parse::<usize>().map_err(|_| anyhow!("fake_type: words, min parse error. please 0 < min "))?;
        let max = trim_value_string(&max_value.to_string()).parse::<usize>().map_err(|_| anyhow!("fake_type: words, max parse error. please 0 < max "))?;
        let lang = trim_value_string(&lang_value.to_string());

        Ok(FakeDefinitionElement::Words(Words::new(fake_type.to_string(), lang, min, max)?))
    }

    pub fn generate_boolean(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: boolean, lang is missing"))?;
        let ratio_value = fake_definition_element_setting.get("ratio").ok_or(anyhow!("fake_type: boolean, ratio is missing"))?;
        let ratio = trim_value_string(&ratio_value.to_string()).parse::<u8>().map_err(|_| anyhow!("fake_type: boolean, ratio parse error. please input 0 to 100"))?;
        let lang = trim_value_string(&lang_value.to_string());
        Ok(FakeDefinitionElement::Boolean(Boolean::new(fake_type.to_string(), lang, ratio)))
    }

    pub fn generate_digit(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: digit, lang is missing"))?;
        let lang = trim_value_string(&lang_value.to_string());
        Ok(FakeDefinitionElement::Digit(Digit::new(fake_type.to_string(), lang)))
    }

    pub fn generate_sentence(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: sentence, lang is missing"))?;
        let min_value = fake_definition_element_setting.get("min").ok_or(anyhow!("fake_type: sentence, min is missing"))?;
        let max_value = fake_definition_element_setting.get("max").ok_or(anyhow!("fake_type: sentence, max is missing"))?;
        let min = trim_value_string(&min_value.to_string()).parse::<usize>().map_err(|_| anyhow!("fake_type: sentence, min parse error. please 0 < min "))?;
        let max = trim_value_string(&max_value.to_string()).parse::<usize>().map_err(|_| anyhow!("fake_type: sentence, max parse error. please 0 < max "))?;
        let lang = trim_value_string(&lang_value.to_string());

        Ok(FakeDefinitionElement::Sentence(Sentence::new(fake_type.to_string(), lang, min, max)?))
    }

    pub fn generate_number_with_format(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let lang_value = fake_definition_element_setting.get("lang").ok_or(anyhow!("fake_type: number_with_format, lang is missing"))?;
        let format_value = fake_definition_element_setting.get("format").ok_or(anyhow!("fake_type: number_with_format, format is missing"))?;
        let format = trim_value_string(&format_value.to_string());
        let lang = trim_value_string(&lang_value.to_string());
        Ok(FakeDefinitionElement::NumberWithFormat(NumberWithFormat::new(fake_type.to_string(), lang, format)))
    }

    pub fn generate_array(fake_definition_element_setting: &serde_json::Map<String, Value>, fake_type: &str) -> Result<FakeDefinitionElement> {
        let count_value = fake_definition_element_setting.get("count").ok_or(anyhow!("fake_type: array, count is missing"))?;
        let count = trim_value_string(&count_value.to_string()).parse::<usize>().map_err(|_| anyhow!("fake_type: array, count parse error. please 0 < count"))?;

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
}

impl FakeDefinitionElement {
    pub fn generate(fake_definition_element_value: &Value) -> Result<FakeDefinitionElement> {
        let fake_definition_element_setting = match fake_definition_element_value {
            Value::Object(map) => Some(map),
            _ => None
        }.ok_or(anyhow!("fake_definition_element_settings: undefined"))?;

        let value = fake_definition_element_setting.get("fake_type").ok_or(anyhow!("fake_definition_element_settings: fake_type is missing"))?;
        let fake_type = value.as_str().ok_or(anyhow!("fake_definition_element_settings: fake_type convert error"))?;

        let obj = match fake_type {
            "word" => FakeDefinitionElement::generate_word(fake_definition_element_setting, fake_type)?,
            "last_name" => FakeDefinitionElement::generate_last_name(fake_definition_element_setting, fake_type)?,
            "words" => FakeDefinitionElement::generate_words(fake_definition_element_setting, fake_type)?,
            "boolean" => FakeDefinitionElement::generate_boolean(fake_definition_element_setting, fake_type)?,
            "digit" => FakeDefinitionElement::generate_digit(fake_definition_element_setting, fake_type)?,
            "sentence" => FakeDefinitionElement::generate_sentence(fake_definition_element_setting, fake_type)?,
            "number_with_format" => FakeDefinitionElement::generate_number_with_format(fake_definition_element_setting, fake_type)?,
            "array" => FakeDefinitionElement::generate_array(fake_definition_element_setting, fake_type)?,
            "map" => FakeDefinitionElement::generate_map(fake_definition_element_setting, fake_type)?,
            _ => {
                Err(anyhow!("missing fake_type"))?
            }
        };

        Ok(obj)
    }
}

fn trim_value_string(s: &str) -> String {
    s.trim_matches('"').to_string()
}
