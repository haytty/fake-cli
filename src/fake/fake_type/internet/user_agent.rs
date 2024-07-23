use fake::Fake;
use fake::faker::internet;
use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeElement, FakeType};
use crate::fake::lang::{get_language, Language};

/// `UserAgent` is an implementation of `FakeType`. It generates a user_agent that varies according to language.
///
/// # Attributes
///
/// * `FakeType`: This provides `UserAgent` with the `fake_apply` and `to_value` methods.
/// * `FakeElement`: This provides `UserAgent` with the `new` method.
///
/// # Example
///
/// ```
/// // Create a new instance of UserAgent, specifying "Japanese" as the language
/// let w = UserAgent::new("user_agent", "Japanese");
/// let user_agent = w.fake_apply();
/// println!("Fake user_agent: {}", user_agent);
/// ```
#[derive(Debug)]
pub struct UserAgent {
    _fake_type: String,
    lang: String,
}

impl FakeType for UserAgent {
    type Response = String;
    fn fake_apply(&self) -> Self::Response {
        let lang = get_language(&self.lang.as_str());
        match lang {
            Language::JaJp(l) => internet::raw::UserAgent(l).fake(),
            Language::En(l) => internet::raw::UserAgent(l).fake(),
            Language::ArSa(l) => internet::raw::UserAgent(l).fake(),
            Language::FrFr(l) => internet::raw::UserAgent(l).fake(),
            Language::PtBr(l) => internet::raw::UserAgent(l).fake(),
            Language::ZhCn(l) => internet::raw::UserAgent(l).fake(),
            Language::ZhTw(l) => internet::raw::UserAgent(l).fake(),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.fake_apply())
    }
}

impl FakeElement for UserAgent {
    fn new(_fake_type: String, lang: String) -> Self {
        Self { _fake_type, lang }
    }
}

impl From<UserAgent> for FakeDefinitionElement {
    fn from(value: UserAgent) -> Self {
        FakeDefinitionElement::UserAgent(value)
    }
}

#[cfg(test)]
mod tests {
    use super::UserAgent;
    use crate::fake::fake_type::{FakeElement, FakeType};

    #[test]
    fn test_user_agent_fake_apply() {
        let w = UserAgent::new("user_agent".to_string(), "English".to_string());
        let user_agent = w.fake_apply();
        assert!(!user_agent.is_empty(), "Generated user_agent should not be empty");
    }

    #[test]
    fn test_user_agent_new() {
        let w = UserAgent::new("user_agent".to_string(), "English".to_string());
        assert_eq!(w._fake_type, "user_agent");
        assert_eq!(w.lang, "English");
    }
}