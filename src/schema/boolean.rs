use crate::error::{Result, ValidationError};
use crate::validate::Validate;
use serde_json::Value;
use smol_str::SmolStr;

pub struct BooleanSchema {
	_private: (),
}

impl BooleanSchema {
	pub fn new() -> Self {
		Self { _private: () }
	}
}

impl Default for BooleanSchema {
	fn default() -> Self {
		Self::new()
	}
}

impl Validate<bool> for BooleanSchema {
	fn validate(&self, input: &Value) -> Result<bool> {
		let value = input.as_bool().ok_or_else(|| ValidationError::InvalidType {
			expected: SmolStr::new_inline("boolean"),
			received: type_name(input),
		})?;

		Ok(value)
	}
}

impl Clone for BooleanSchema {
	fn clone(&self) -> Self {
		Self::new()
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
	use serde_json::json;

	#[test]
	fn test_boolean_basic() {
		let schema = BooleanSchema::new();
		assert!(schema.validate(&json!(true)).is_ok());
		assert!(schema.validate(&json!(false)).is_ok());
		assert!(schema.validate(&json!(1)).is_err());
		assert!(schema.validate(&json!("true")).is_err());
	}
}
