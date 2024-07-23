use serde_json::Value;
use crate::fake::fake_definition_element::FakeDefinitionElement;
use crate::fake::fake_type::FakeType;

/// `Array` is an implementation of `FakeType`. It generates an array of fake elements.
/// The length of the array and the type of elements it contains are specified.
///
/// # Attributes
///
/// * `FakeType`: This provides `Array` with the `fake_apply` and `to_value` methods.
///
/// # Example
///
/// ```
/// // Create a new instance of Array with FakeDefinitionElement instances
/// let a = Array::new("array", 5, Box::new(FakeDefinitionElement::Constant(Constant::new("constant".to_string(), Value::String("Hello, world!".to_string())))));
/// let array = a.fake_apply();
/// println!("Fake array: {:?}", array);
/// ```
#[derive(Debug)]
pub struct Array {
    _fake_type: String,
    count: usize,
    fake_definition_element: Box<FakeDefinitionElement>,
}

impl FakeType for Array {
    type Response = Vec<Value>;

    fn fake_apply(&self) -> Self::Response {
        let mut array = Vec::new();

        for _ in 0..self.count {
            let fake_definition_value = self.fake_definition_element.to_value();
            array.push(fake_definition_value.clone());
        }

        array
    }

    fn to_value(&self) -> Value {
        Value::Array(self.fake_apply())
    }
}

impl Array {
    pub fn new(_fake_type: String, count: usize, fake_definition_element: Box<FakeDefinitionElement>) -> Self {
        Self { _fake_type, count, fake_definition_element }
    }
}

impl From<Array> for FakeDefinitionElement {
    fn from(value: Array) -> Self {
        FakeDefinitionElement::Array(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::fake::fake_type::FakeType;
    use crate::fake::fake_definition_element::FakeDefinitionElement;
    use crate::fake::fake_type::constant::Constant;
    use serde_json::Value;

    #[test]
    fn test_array_fake_apply() {
        let a = Array::new(
            "array".to_string(),
            3,
            Box::new(
                FakeDefinitionElement::Constant(
                    Constant::new("constant".to_string(), Value::String("Hello, world!".to_string()))
                )
            ),
        );
        let array_value = a.fake_apply();

        assert_eq!(array_value.len(), 3, "Generated array should have 3 elements");
    }

    #[test]
    fn test_array_new() {
        let a = Array::new(
            "array".to_string(),
            3,
            Box::new(
                FakeDefinitionElement::Constant(
                    Constant::new("constant".to_string(), Value::String("Hello, world!".to_string()))
                )
            ),
        );

        assert_eq!(a._fake_type, "array");
        assert_eq!(a.count, 3);
    }
}