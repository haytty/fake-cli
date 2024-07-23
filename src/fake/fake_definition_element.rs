use std::collections::{BTreeMap, VecDeque};
use serde_json::{Value};
use crate::fake::fake_type::array::Array;
use crate::fake::fake_type::map::Map;
use anyhow::{anyhow, Result};
use crate::fake::fake_type::{FakeElement, FakeType, FakeWithFormatElement, FakeWithRangeElement, FakeWithRatioElement};
use crate::fake::fake_type::boolean::boolean::Boolean;
use crate::fake::fake_type::constant::Constant;
use crate::fake::fake_type::internet::domain_suffix::DomainSuffix;
use crate::fake::fake_type::internet::free_email::FreeEmail;
use crate::fake::fake_type::internet::free_email_provider::FreeEmailProvider;
use crate::fake::fake_type::internet::ip::IP;
use crate::fake::fake_type::internet::ip_v4::IPv4;
use crate::fake::fake_type::internet::ip_v6::IPv6;
use crate::fake::fake_type::internet::mac_address::MACAddress;
use crate::fake::fake_type::internet::password::Password;
use crate::fake::fake_type::internet::safe_email::SafeEmail;
use crate::fake::fake_type::internet::user_agent::UserAgent;
use crate::fake::fake_type::internet::username::Username;
use crate::fake::fake_type::lorem::paragraph::Paragraph;
use crate::fake::fake_type::lorem::paragraphs::Paragraphs;
use crate::fake::fake_type::lorem::sentence::Sentence;
use crate::fake::fake_type::lorem::sentences::Sentences;
use crate::fake::fake_type::lorem::word::Word;
use crate::fake::fake_type::lorem::words::Words;
use crate::fake::fake_type::name::first_name::FirstName;
use crate::fake::fake_type::name::last_name::LastName;
use crate::fake::fake_type::name::name::Name;
use crate::fake::fake_type::name::name_with_title::NameWithTitle;
use crate::fake::fake_type::name::suffix::Suffix;
use crate::fake::fake_type::name::title::Title;
use crate::fake::fake_type::number::digit::Digit;
use crate::fake::fake_type::number::number_with_format::NumberWithFormat;

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
    // Lorem
    Word(Word),
    Words(Words),
    Sentence(Sentence),
    Sentences(Sentences),
    Paragraph(Paragraph),
    Paragraphs(Paragraphs),

    // Name
    FirstName(FirstName),
    LastName(LastName),
    Title(Title),
    Suffix(Suffix),
    Name(Name),
    NameWithTitle(NameWithTitle),

    // Number
    Digit(Digit),
    NumberWithFormat(NumberWithFormat),

    // Boolean
    Boolean(Boolean),

    // Internet
    FreeEmailProvider(FreeEmailProvider),
    DomainSuffix(DomainSuffix),
    FreeEmail(FreeEmail),
    SafeEmail(SafeEmail),
    Username(Username),
    Password(Password),
    IPv4(IPv4),
    IPv6(IPv6),
    IP(IP),
    MACAddress(MACAddress),
    UserAgent(UserAgent),

    // FakeCliOriginal
    Array(Array),
    Map(Map),
    Constant(Constant),
}

impl FakeDefinitionElement {
    pub fn to_value(&self) -> Value {
        match self {
            // Lorem
            FakeDefinitionElement::Word(data) => data.to_value(),
            FakeDefinitionElement::Words(data) => data.to_value(),
            FakeDefinitionElement::Sentence(data) => data.to_value(),
            FakeDefinitionElement::Sentences(data) => data.to_value(),
            FakeDefinitionElement::Paragraph(data) => data.to_value(),
            FakeDefinitionElement::Paragraphs(data) => data.to_value(),

            // Name
            FakeDefinitionElement::FirstName(data) => data.to_value(),
            FakeDefinitionElement::LastName(data) => data.to_value(),
            FakeDefinitionElement::Title(data) => data.to_value(),
            FakeDefinitionElement::Suffix(data) => data.to_value(),
            FakeDefinitionElement::Name(data) => data.to_value(),
            FakeDefinitionElement::NameWithTitle(data) => data.to_value(),

            // Number
            FakeDefinitionElement::Digit(data) => data.to_value(),
            FakeDefinitionElement::NumberWithFormat(data) => data.to_value(),

            // Boolean
            FakeDefinitionElement::Boolean(data) => data.to_value(),

            // Internet
            FakeDefinitionElement::FreeEmailProvider(data) => data.to_value(),
            FakeDefinitionElement::DomainSuffix(data) => data.to_value(),
            FakeDefinitionElement::FreeEmail(data) => data.to_value(),
            FakeDefinitionElement::SafeEmail(data) => data.to_value(),
            FakeDefinitionElement::Username(data) => data.to_value(),
            FakeDefinitionElement::Password(data) => data.to_value(),
            FakeDefinitionElement::IPv4(data) => data.to_value(),
            FakeDefinitionElement::IPv6(data) => data.to_value(),
            FakeDefinitionElement::IP(data) => data.to_value(),
            FakeDefinitionElement::MACAddress(data) => data.to_value(),
            FakeDefinitionElement::UserAgent(data) => data.to_value(),

            // FakeCliOriginal
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
            // Lorem
            "word" => FakeDefinitionElement::generate_element::<Word>(fake_definition_element_setting, fake_type)?,
            "words" => FakeDefinitionElement::generate_with_range_element::<Words>(fake_definition_element_setting, fake_type)?,
            "sentence" => FakeDefinitionElement::generate_with_range_element::<Sentence>(fake_definition_element_setting, fake_type)?,
            "sentences" => FakeDefinitionElement::generate_with_range_element::<Sentences>(fake_definition_element_setting, fake_type)?,
            "paragraph" => FakeDefinitionElement::generate_with_range_element::<Paragraph>(fake_definition_element_setting, fake_type)?,
            "paragraphs" => FakeDefinitionElement::generate_with_range_element::<Paragraphs>(fake_definition_element_setting, fake_type)?,

            // Name
            "first_name" => FakeDefinitionElement::generate_element::<FirstName>(fake_definition_element_setting, fake_type)?,
            "last_name" => FakeDefinitionElement::generate_element::<LastName>(fake_definition_element_setting, fake_type)?,
            "title" => FakeDefinitionElement::generate_element::<Title>(fake_definition_element_setting, fake_type)?,
            "suffix" => FakeDefinitionElement::generate_element::<Suffix>(fake_definition_element_setting, fake_type)?,
            "name" => FakeDefinitionElement::generate_element::<Name>(fake_definition_element_setting, fake_type)?,
            "name_with_title" => FakeDefinitionElement::generate_element::<NameWithTitle>(fake_definition_element_setting, fake_type)?,

            // Number
            "digit" => FakeDefinitionElement::generate_element::<Digit>(fake_definition_element_setting, fake_type)?,
            "number_with_format" => FakeDefinitionElement::generate_with_format_element::<NumberWithFormat>(fake_definition_element_setting, fake_type)?,

            // Boolean
            "boolean" => FakeDefinitionElement::generate_with_ratio_element::<Boolean>(fake_definition_element_setting, fake_type)?,

            // Internet
            "free_email_provider" => FakeDefinitionElement::generate_element::<FreeEmailProvider>(fake_definition_element_setting, fake_type)?,
            "domain_suffix" => FakeDefinitionElement::generate_element::<DomainSuffix>(fake_definition_element_setting, fake_type)?,
            "free_email" => FakeDefinitionElement::generate_element::<FreeEmail>(fake_definition_element_setting, fake_type)?,
            "safe_email" => FakeDefinitionElement::generate_element::<SafeEmail>(fake_definition_element_setting, fake_type)?,
            "username" => FakeDefinitionElement::generate_element::<Username>(fake_definition_element_setting, fake_type)?,
            "password" => FakeDefinitionElement::generate_with_range_element::<Password>(fake_definition_element_setting, fake_type)?,
            "ip_v4" => FakeDefinitionElement::generate_element::<IPv4>(fake_definition_element_setting, fake_type)?,
            "ip_v6" => FakeDefinitionElement::generate_element::<IPv6>(fake_definition_element_setting, fake_type)?,
            "ip" => FakeDefinitionElement::generate_element::<IP>(fake_definition_element_setting, fake_type)?,
            "mac_address" => FakeDefinitionElement::generate_element::<MACAddress>(fake_definition_element_setting, fake_type)?,
            "user_agent" => FakeDefinitionElement::generate_element::<UserAgent>(fake_definition_element_setting, fake_type)?,

            // FakeCliOriginal
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

    // Lorem
    #[test]
    fn test_fake_definition_element_generate_for_word() {
        let fd = generate_element("word", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_words() {
        let fd = generate_element_with_range("words", "EN", 1, 5);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_sentence() {
        let fd = generate_element_with_range("sentence", "EN", 1, 3);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_sentences() {
        let fd = generate_element_with_range("sentences", "EN", 1, 3);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_paragraph() {
        let fd = generate_element_with_range("paragraph", "EN", 1, 3);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_paragraphs() {
        let fd = generate_element_with_range("paragraphs", "EN", 1, 3);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    // Name
    #[test]
    fn test_fake_definition_element_generate_for_first_name() {
        let fd = generate_element("first_name", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_last_name() {
        let fd = generate_element("last_name", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_title() {
        let fd = generate_element("title", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_suffix() {
        let fd = generate_element("suffix", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_name() {
        let fd = generate_element("name", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_name_with_title() {
        let fd = generate_element("name_with_title", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    // Number
    #[test]
    fn test_fake_definition_element_generate_for_digit() {
        let fd = generate_element("digit", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_number_with_format() {
        let fd = generate_element_with_format("number_with_format", "EN", "TEST ^#####");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    // Boolean
    #[test]
    fn test_fake_definition_element_generate_for_boolean() {
        let fd = generate_element_with_ratio("boolean", "EN", 50);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    // Internet
    #[test]
    fn test_fake_definition_element_generate_for_free_email_provider() {
        let fd = generate_element("free_email_provider", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_domain_suffix() {
        let fd = generate_element("domain_suffix", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_free_email() {
        let fd = generate_element("free_email", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_safe_email() {
        let fd = generate_element("safe_email", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_username() {
        let fd = generate_element("username", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_password() {
        let fd = generate_element_with_range("username", "EN", 1, 5);
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_ip_v4() {
        let fd = generate_element("ip_v4", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_ip_v6() {
        let fd = generate_element("ip_v6", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_ip() {
        let fd = generate_element("ip", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_mac_address() {
        let fd = generate_element("mac_address", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    #[test]
    fn test_fake_definition_element_generate_for_user_agent() {
        let fd = generate_element("user_agent", "EN");
        assert!(fd.is_ok(), "Should return Ok for a defined fake type");
    }

    // FakeCliOriginal
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