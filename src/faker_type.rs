use fake::Fake;
use fake::faker::boolean::raw::Boolean;
use fake::faker::color::raw::*;
use fake::faker::company::raw::*;
use fake::faker::http::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::lorem::raw::*;
use fake::faker::name::raw::*;
use fake::faker::number::raw::*;
use fake::locales::{EN, JA_JP};
use serde_json::{Number, Value};

use crate::error::FakerTypeError;

pub enum FakerType {
    Word,
    Words,
    Sentence,
    Sentences,
    Paragraph,
    Paragraphs,
    FirstName,
    LastName,
    Title,
    Suffix,
    Name,
    NameWithTitle,
    Digit,
    NumberWithFormat,
    Boolean,
    FreeEmailProvider,
    DomainSuffix,
    FreeEmail,
    SafeEmail,
    Username,
    Password,
    IPv4,
    IPv6,
    IP,
    MACAddress,
    UserAgent,
    RfcStatusCode,
    ValidStatusCode,
    HexColor,
    RgbColor,
    RgbaColor,
    HslColor,
    HslaColor,
    Color,
    CompanySuffix,
    CompanyName,
    Buzzword,
    BuzzwordMiddle,
    BuzzwordTail,
    CatchPhrase,
    BsVerb,
    BsAdj,
    BsNoun,
    Bs,
    Profession,
    Industry,
}

impl FakerType {
    pub fn fake(&self) -> Value {
        match self {
            FakerType::Word => {
                Value::String(Word(JA_JP).fake())
            }
            FakerType::Words => {
                let vs: Vec<String> = Words(JA_JP, 3..5).fake();
                Value::Array(vs.iter().map(|word| Value::String(word.to_string())).collect())
            }
            FakerType::Sentence => {
                Value::String(Sentence(JA_JP, 3..5).fake())
            }
            FakerType::Sentences => {
                let vs: Vec<String> = Sentences(JA_JP, 3..5).fake();
                Value::Array(vs.iter().map(|word| Value::String(word.to_string())).collect())
            }
            FakerType::Paragraph => {
                Value::String(Paragraph(JA_JP, 3..5).fake())
            }
            FakerType::Paragraphs => {
                let vs: Vec<String> = Paragraphs(JA_JP, 3..5).fake();
                Value::Array(vs.iter().map(|word| Value::String(word.to_string())).collect())
            }
            FakerType::FirstName => {
                Value::String(FirstName(JA_JP).fake())
            }
            FakerType::LastName => {
                Value::String(LastName(JA_JP).fake())
            }
            FakerType::Title => {
                Value::String(Title(JA_JP).fake())
            }
            FakerType::Suffix => {
                Value::String(Suffix(JA_JP).fake())
            }
            FakerType::Name => {
                Value::String(Name(JA_JP).fake())
            }
            FakerType::NameWithTitle => {
                Value::String(NameWithTitle(JA_JP).fake())
            }
            FakerType::Digit => {
                let digit: &str = Digit(JA_JP).fake();
                let number = Number::from(digit.parse::<u8>().unwrap());
                Value::Number(number)
            }
            FakerType::NumberWithFormat => {
                let n: String = NumberWithFormat(JA_JP, "FLAT 0# ^#/F").fake();
                Value::String(n)
                // let number = Number::from(n.parse::<u32>().unwrap());
                // Value::Number(number)
            }
            FakerType::Boolean => {
                // 50%の割合
                Value::Bool(Boolean(JA_JP, 50).fake())
            }
            FakerType::FreeEmailProvider => {
                Value::String(FreeEmailProvider(EN).fake())
            }
            FakerType::DomainSuffix => {
                Value::String(DomainSuffix(JA_JP).fake())
            }
            FakerType::FreeEmail => {
                Value::String(FreeEmail(EN).fake())
            }
            FakerType::SafeEmail => {
                Value::String(SafeEmail(EN).fake())
            }
            FakerType::Username => {
                Value::String(Username(JA_JP).fake())
            }
            FakerType::Password => {
                Value::String(Password(JA_JP, 8..20).fake())
            }
            FakerType::IPv4 => {
                Value::String(IPv4(JA_JP).fake())
            }
            FakerType::IPv6 => {
                Value::String(IPv6(JA_JP).fake())
            }
            FakerType::IP => {
                Value::String(IP(JA_JP).fake())
            }
            FakerType::MACAddress => {
                Value::String(MACAddress(JA_JP).fake())
            }
            FakerType::UserAgent => {
                Value::String(UserAgent(JA_JP).fake())
            }
            FakerType::RfcStatusCode => {
                Value::String(RfcStatusCode(JA_JP).fake())
            }
            FakerType::ValidStatusCode => {
                Value::String(ValidStatusCode(JA_JP).fake())
            }
            FakerType::HexColor => {
                Value::String(HexColor(JA_JP).fake())
            }
            FakerType::RgbColor => {
                Value::String(RgbColor(JA_JP).fake())
            }
            FakerType::RgbaColor => {
                Value::String(RgbaColor(JA_JP).fake())
            }
            FakerType::HslColor => {
                Value::String(HslColor(JA_JP).fake())
            }
            FakerType::HslaColor => {
                Value::String(HslaColor(JA_JP).fake())
            }
            FakerType::Color => {
                Value::String(Color(JA_JP).fake())
            }
            FakerType::CompanySuffix => {
                Value::String(CompanySuffix(JA_JP).fake())
            }
            FakerType::CompanyName => {
                Value::String(CompanyName(JA_JP).fake())
            }
            FakerType::Buzzword => {
                Value::String(Buzzword(JA_JP).fake())
            }
            FakerType::BuzzwordMiddle => {
                Value::String(BuzzwordMiddle(JA_JP).fake())
            }
            FakerType::BuzzwordTail => {
                Value::String(BuzzwordTail(JA_JP).fake())
            }
            FakerType::CatchPhrase => {
                Value::String(CatchPhase(JA_JP).fake())
            }
            FakerType::BsVerb => {
                Value::String(BsVerb(JA_JP).fake())
            }
            FakerType::BsAdj => {
                Value::String(BsAdj(JA_JP).fake())
            }
            FakerType::BsNoun => {
                Value::String(BsNoun(JA_JP).fake())
            }
            FakerType::Bs => {
                Value::String(Bs(JA_JP).fake())
            }
            FakerType::Profession => {
                Value::String(Profession(JA_JP).fake())
            }
            FakerType::Industry => {
                Value::String(Industry(JA_JP).fake())
            }
            _ => {
                Value::String("Undefined".to_string())
            }
        }
    }
}

impl TryFrom<String> for FakerType {
    type Error = FakerTypeError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "first_name" => Ok(FakerType::FirstName),
            "last_name" => Ok(FakerType::LastName),
            "title" => Ok(FakerType::Title),
            "suffix" => Ok(FakerType::Suffix),
            "name" => Ok(FakerType::Name),
            "name_with_title" => Ok(FakerType::NameWithTitle),
            "digit" => Ok(FakerType::Digit),
            "number_with_format" => Ok(FakerType::NumberWithFormat),
            "boolean" => Ok(FakerType::Boolean),
            "free_email_provider" => Ok(FakerType::FreeEmailProvider),
            "domain_suffix" => Ok(FakerType::DomainSuffix),
            "free_email" => Ok(FakerType::FreeEmail),
            "safe_email" => Ok(FakerType::SafeEmail),
            "username" => Ok(FakerType::Username),
            "password" => Ok(FakerType::Password),
            "ip_v4" => Ok(FakerType::IPv4),
            "ip_v6" => Ok(FakerType::IPv6),
            "ip" => Ok(FakerType::IP),
            "mac_address" => Ok(FakerType::MACAddress),
            "user_agent" => Ok(FakerType::UserAgent),
            "rfc_status_code" => Ok(FakerType::RfcStatusCode),
            "valid_status_code" => Ok(FakerType::ValidStatusCode),
            "word" => Ok(FakerType::Word),
            "words" => Ok(FakerType::Words),
            "sentence" => Ok(FakerType::Sentence),
            "sentences" => Ok(FakerType::Sentences),
            "paragraph" => Ok(FakerType::Paragraph),
            "paragraphs" => Ok(FakerType::Paragraphs),
            "hex_color" => Ok(FakerType::HexColor),
            "rgb_color" => Ok(FakerType::RgbColor),
            "rgba_color" => Ok(FakerType::RgbaColor),
            "hsl_color" => Ok(FakerType::HslColor),
            "hsla_color" => Ok(FakerType::HslaColor),
            "color" => Ok(FakerType::Color),
            "company_suffix" => Ok(FakerType::CompanySuffix),
            "company_name" => Ok(FakerType::CompanyName),
            "buzzword" => Ok(FakerType::Buzzword),
            "buzzword_middle" => Ok(FakerType::BuzzwordMiddle),
            "buzzword_tail" => Ok(FakerType::BuzzwordTail),
            "catch_phrase" => Ok(FakerType::CatchPhrase),
            "bs_verb" => Ok(FakerType::BsVerb),
            "bs_adj" => Ok(FakerType::BsAdj),
            "bs_noun" => Ok(FakerType::BsNoun),
            "bs" => Ok(FakerType::Bs),
            "profession" => Ok(FakerType::Profession),
            "industry" => Ok(FakerType::Industry),
            _ => Err(FakerTypeError::InvalidType),
        }
    }
}
