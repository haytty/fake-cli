use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::{FakeType};

/// `Constant` is an implementation of `FakeType`. It doesn't generate a fake value but instead returns
/// a constant value that was supplied to it.
///
/// # Attributes
///
/// * `FakeType`: This provides `Constant` with the `fake_apply` and `to_value` methods.
///
/// # Example
///
/// ```
/// // Create a new instance of Constant, specifying "constant" as the type and a constant value
/// let c = Constant::new("constant", Value::String("Hello, world!".to_string()));
/// let constant = c.fake_apply();
/// println!("Constant value: {}", constant);
/// ```
#[derive(Debug)]
pub struct Constant {
    _fake_type: String,
    value: Value,
}

impl FakeType for Constant {
    type Response = Value;

    fn fake_apply(&self) -> Self::Response {
        self.value.clone()
    }

    fn to_value(&self) -> Value {
        self.fake_apply()
    }
}

impl Constant {
    pub fn new(_fake_type: String, value: Value) -> Self {
        Self { _fake_type, value }
    }
}

impl From<Constant> for FakeDefinitionElement {
    fn from(value: Constant) -> Self {
        FakeDefinitionElement::Constant(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Constant;
    use crate::fake::fake_type::FakeType;
    use serde_json::Value;

    #[test]
    fn test_constant_fake_apply() {
        let c = Constant::new("constant".to_string(), Value::String("Hello, world!".to_string()));
        let constant_value = c.fake_apply();

        assert_eq!(constant_value, Value::String("Hello, world!".to_string()));
    }

    #[test]
    fn test_constant_new() {
        let c = Constant::new("constant".to_string(), Value::String("Hello, world!".to_string()));

        assert_eq!(c._fake_type, "constant");
        assert_eq!(c.value, Value::String("Hello, world!".to_string()));
    }
}