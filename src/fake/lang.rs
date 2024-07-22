use fake::locales::{AR_SA, EN, FR_FR, JA_JP, PT_BR, ZH_CN, ZH_TW};

pub enum Language {
    JaJp(JA_JP),
    En(EN),
    ArSa(AR_SA),
    FrFr(FR_FR),
    PtBr(PT_BR),
    ZhCn(ZH_CN),
    ZhTw(ZH_TW),
}

pub fn get_language(lang_string: &str) -> Language {
    match lang_string {
        "JA_JP" => Language::JaJp(JA_JP),
        "EN" => Language::En(EN),
        "AR_SA" => Language::ArSa(AR_SA),
        "FR_FR" => Language::FrFr(FR_FR),
        "PT_BR" => Language::PtBr(PT_BR),
        "ZH_CN" => Language::ZhCn(ZH_CN),
        "ZH_TW" => Language::ZhTw(ZH_TW),
        _ => Language::En(EN),
    }
}

