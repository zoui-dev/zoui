use crate::error::{Result, TupleError, ValidationError};
use crate::validate::Validate;
use serde_json::Value;
use smol_str::SmolStr;

pub struct TupleSchema<T> {
	items: T,
}

impl<T> TupleSchema<T> {
	pub fn new(items: T) -> Self {
		Self { items }
	}
}

impl<T: Clone> Clone for TupleSchema<T> {
	fn clone(&self) -> Self {
		Self { items: self.items.clone() }
	}
}

impl<T0, T1, O0, O1> Validate<(O0, O1)> for TupleSchema<(T0, T1)>
where
	T0: Validate<O0>,
	T1: Validate<O1>,
{
	fn validate(&self, input: &Value) -> Result<(O0, O1)> {
		let arr = input.as_array().ok_or_else(|| ValidationError::InvalidType {
			expected: SmolStr::new_inline("array"),
			received: type_name(input),
		})?;

		if arr.len() != 2 {
			return Err(ValidationError::Tuple(TupleError::InvalidLength {
				expected: 2,
				actual: arr.len(),
			})
			.into());
		}

		let v0 = self.items.0.validate(&arr[0]).map_err(|e| match e {
			crate::Error::Validation(ve) => ValidationError::Tuple(TupleError::InvalidElement {
				index: 0,
				source: Box::new(ve),
			})
			.into(),
			other => other,
		})?;

		let v1 = self.items.1.validate(&arr[1]).map_err(|e| match e {
			crate::Error::Validation(ve) => ValidationError::Tuple(TupleError::InvalidElement {
				index: 1,
				source: Box::new(ve),
			})
			.into(),
			other => other,
		})?;

		Ok((v0, v1))
	}
}

impl<T0, T1, T2, O0, O1, O2> Validate<(O0, O1, O2)> for TupleSchema<(T0, T1, T2)>
where
	T0: Validate<O0>,
	T1: Validate<O1>,
	T2: Validate<O2>,
{
	fn validate(&self, input: &Value) -> Result<(O0, O1, O2)> {
		let arr = input.as_array().ok_or_else(|| ValidationError::InvalidType {
			expected: SmolStr::new_inline("array"),
			received: type_name(input),
		})?;

		if arr.len() != 3 {
			return Err(ValidationError::Tuple(TupleError::InvalidLength {
				expected: 3,
				actual: arr.len(),
			})
			.into());
		}

		let v0 = self.items.0.validate(&arr[0]).map_err(|e| match e {
			crate::Error::Validation(ve) => ValidationError::Tuple(TupleError::InvalidElement {
				index: 0,
				source: Box::new(ve),
			})
			.into(),
			other => other,
		})?;

		let v1 = self.items.1.validate(&arr[1]).map_err(|e| match e {
			crate::Error::Validation(ve) => ValidationError::Tuple(TupleError::InvalidElement {
				index: 1,
				source: Box::new(ve),
			})
			.into(),
			other => other,
		})?;

		let v2 = self.items.2.validate(&arr[2]).map_err(|e| match e {
			crate::Error::Validation(ve) => ValidationError::Tuple(TupleError::InvalidElement {
				index: 2,
				source: Box::new(ve),
			})
			.into(),
			other => other,
		})?;

		Ok((v0, v1, v2))
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
	use crate::schema::{BooleanSchema, NumberSchema, StringSchema};
	use serde_json::json;

	#[test]
	fn test_tuple_2() {
		let schema = TupleSchema::new((StringSchema::new(), NumberSchema::new()));

		let result = schema.validate(&json!(["hello", 42])).unwrap();
		assert_eq!(result.0, "hello");
		assert_eq!(result.1, 42.0);

		assert!(schema.validate(&json!(["hello"])).is_err());
		assert!(schema.validate(&json!(["hello", 42, true])).is_err());

		assert!(schema.validate(&json!([123, "world"])).is_err());
	}

	#[test]
	fn test_tuple_3() {
		let schema =
			TupleSchema::new((StringSchema::new(), NumberSchema::new(), BooleanSchema::new()));

		let result = schema.validate(&json!(["test", 123, true])).unwrap();
		assert_eq!(result.0, "test");
		assert_eq!(result.1, 123.0);
		assert!(result.2);
		assert!(schema.validate(&json!(["test", 123])).is_err());
		assert!(schema.validate(&json!(["test", "wrong", true])).is_err());
	}

	#[test]
	fn test_tuple_not_array() {
		let schema = TupleSchema::new((StringSchema::new(), NumberSchema::new()));
		assert!(schema.validate(&json!("not an array")).is_err());
		assert!(schema.validate(&json!({"key": "value"})).is_err());
	}
}
