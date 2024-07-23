use fake::locales::{AR_SA, EN, FR_FR, JA_JP, PT_BR, ZH_CN, ZH_TW};

/// `Language` enum represents a set of supported languages.
/// Each variant of the enum corresponds to a different language.
pub enum Language {
    JaJp(JA_JP),
    En(EN),
    ArSa(AR_SA),
    FrFr(FR_FR),
    PtBr(PT_BR),
    ZhCn(ZH_CN),
    ZhTw(ZH_TW),
}

/// Takes a string as input and returns the corresponding language variant from the `Language` enum.
/// If the string does not match any of the existing variants, it defaults to English(en).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_language() {
        let lang = get_language("JA_JP");
        assert!(matches!(lang, Language::JaJp(_)), "Should return the Japanese language variant");

        let lang = get_language("EN");
        assert!(matches!(lang, Language::En(_)), "Should return the English language variant");

        let lang = get_language("NON_EXISTENT");
        assert!(matches!(lang, Language::En(_)), "Should default to English for non-existent languages");
    }
}