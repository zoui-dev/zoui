use crate::error::{ArrayError, ValidationError};
use crate::validate::Validate;
use crate::Result;
use serde_json::Value;
use smol_str::SmolStr;

pub struct ArraySchema<S> {
    element: S,
    min: Option<usize>,
    max: Option<usize>,
}

impl<S> ArraySchema<S> {
    pub fn new(element: S) -> Self {
        Self {
            element,
            min: None,
            max: None,
        }
    }

    pub fn min(mut self, len: usize) -> Self {
        self.min = Some(len);
        self
    }

    pub fn max(mut self, len: usize) -> Self {
        self.max = Some(len);
        self
    }

    pub fn length(self, len: usize) -> Self {
        self.min(len).max(len)
    }

    pub fn nonempty(self) -> Self {
        self.min(1)
    }
}

impl<S, T> Validate<Vec<T>> for ArraySchema<S>
where
    S: Validate<T>,
{
    fn validate(&self, input: &Value) -> Result<Vec<T>> {
        let arr = input.as_array().ok_or_else(|| ValidationError::InvalidType {
            expected: SmolStr::new_inline("array"),
            received: type_name(input),
        })?;

        if let Some(min) = self.min
            && arr.len() < min {
                return Err(ValidationError::Array(ArrayError::TooShort {
                    min,
                    actual: arr.len(),
                }).into());
            }

        if let Some(max) = self.max
            && arr.len() > max {
                return Err(ValidationError::Array(ArrayError::TooLong {
                    max,
                    actual: arr.len(),
                }).into());
            }

        let mut results = Vec::with_capacity(arr.len());
        for (i, v) in arr.iter().enumerate() {
            match self.element.validate(v) {
                Ok(val) => results.push(val),
                Err(e) => {
                    let validation_err = match e {
                        crate::Error::Validation(ve) => ve,
                        crate::Error::Parse(pe) => return Err(crate::Error::Parse(pe)),
                        crate::Error::Io(io) => return Err(crate::Error::Io(io)),
                    };
                    return Err(ValidationError::Array(ArrayError::InvalidElement {
                        index: i,
                        source: Box::new(validation_err),
                    }).into());
                }
            }
        }

        Ok(results)
    }
}

impl<S: Clone> Clone for ArraySchema<S> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            min: self.min,
            max: self.max,
        }
    }
}

fn type_name(value: &Value) -> SmolStr {
    SmolStr::new(match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::number::NumberSchema;
    use crate::schema::string::StringSchema;
    use crate::schema::array;
    use serde_json::json;

    #[test]
    fn test_array_basic() {
        let schema = array(StringSchema::new());
        assert!(schema.validate(&json!(["a", "b", "c"])).is_ok());
        assert!(schema.validate(&json!([1, 2, 3])).is_err());
    }

    #[test]
    fn test_array_min_max() {
        let schema = array(NumberSchema::new()).min(2).max(5);
        assert!(schema.validate(&json!([1, 2, 3])).is_ok());
        assert!(schema.validate(&json!([1])).is_err());
        assert!(schema.validate(&json!([1, 2, 3, 4, 5, 6])).is_err());
    }

    #[test]
    fn test_array_element_validation() {
        let schema = array(NumberSchema::new());
        let result = schema.validate(&json!([1, "invalid", 3]));
        assert!(result.is_err());
    }

    #[test]
    fn test_array_nonempty() {
        let schema = array(StringSchema::new()).nonempty();
        assert!(schema.validate(&json!(["a"])).is_ok());
        assert!(schema.validate(&json!([])).is_err());
    }
}
