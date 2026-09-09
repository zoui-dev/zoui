use crate::{NumberError, Result, Validate, ValidationError};
use serde_json::Value;
use smol_str::SmolStr;
use std::sync::Arc;

pub struct NumberSchema {
	min: Option<f64>,
	max: Option<f64>,
	min_inclusive: bool,
	max_inclusive: bool,
	int_only: bool,
	positive: bool,
	negative: bool,
	nonnegative: bool,
	nonpositive: bool,
	finite: bool,
	validators: Vec<Validator>,
}

type ValidatorFn = Arc<dyn Fn(f64) -> std::result::Result<(), String> + Send + Sync>;

struct Validator {
	func: ValidatorFn,
}

impl NumberSchema {
	pub fn new() -> Self {
		Self {
			min: None,
			max: None,
			min_inclusive: true,
			max_inclusive: true,
			int_only: false,
			positive: false,
			negative: false,
			nonnegative: false,
			nonpositive: false,
			finite: false,
			validators: Vec::new(),
		}
	}

	// Builder 方法
	pub fn min(mut self, value: f64) -> Self {
		self.min = Some(value);
		self.min_inclusive = true;
		self
	}

	pub fn max(mut self, value: f64) -> Self {
		self.max = Some(value);
		self.max_inclusive = true;
		self
	}

	pub fn gt(mut self, value: f64) -> Self {
		self.min = Some(value);
		self.min_inclusive = false;
		self
	}

	pub fn gte(self, value: f64) -> Self {
		self.min(value)
	}

	pub fn lt(mut self, value: f64) -> Self {
		self.max = Some(value);
		self.max_inclusive = false;
		self
	}

	pub fn lte(self, value: f64) -> Self {
		self.max(value)
	}

	pub fn int(mut self) -> Self {
		self.int_only = true;
		self
	}

	pub fn positive(mut self) -> Self {
		self.positive = true;
		self
	}

	pub fn negative(mut self) -> Self {
		self.negative = true;
		self
	}

	pub fn nonnegative(mut self) -> Self {
		self.nonnegative = true;
		self
	}

	pub fn nonpositive(mut self) -> Self {
		self.nonpositive = true;
		self
	}

	pub fn finite(mut self) -> Self {
		self.finite = true;
		self
	}

	pub fn safe(self) -> Self {
		self.min(-(2f64.powi(53) - 1.0)).max(2f64.powi(53) - 1.0)
	}

	pub fn multiple_of(self, value: f64) -> Self {
		self.refine(move |n: f64| {
			if (n / value).fract().abs() < f64::EPSILON {
				Ok(())
			} else {
				Err(format!("must be multiple of {}", value))
			}
		})
	}

	pub fn refine<F>(mut self, validator: F) -> Self
	where
		F: Fn(f64) -> std::result::Result<(), String> + Send + Sync + 'static,
	{
		self.validators.push(Validator { func: Arc::new(validator) });
		self
	}
}

impl Default for NumberSchema {
	fn default() -> Self {
		Self::new()
	}
}

impl Validate<f64> for NumberSchema {
	fn validate(&self, input: &Value) -> Result<f64> {
		let num = input.as_f64().ok_or_else(|| ValidationError::InvalidType {
			expected: SmolStr::new_inline("number"),
			received: type_name(input),
		})?;

		// 2. 有限性检查
		if self.finite && !num.is_finite() {
			return Err(ValidationError::Number(NumberError::NotFinite).into());
		}

		// 3. 整数检查
		if self.int_only && num.fract().abs() > f64::EPSILON {
			return Err(ValidationError::Number(NumberError::NotInteger).into());
		}

		// 4. 正负数检查
		if self.positive && num <= 0.0 {
			return Err(ValidationError::Number(NumberError::InvalidSign {
				sign: SmolStr::new_inline("positive"),
			})
			.into());
		}

		if self.negative && num >= 0.0 {
			return Err(ValidationError::Number(NumberError::InvalidSign {
				sign: SmolStr::new_inline("negative"),
			})
			.into());
		}

		if self.nonnegative && num < 0.0 {
			return Err(ValidationError::Number(NumberError::InvalidSign {
				sign: SmolStr::new_inline("non-negative"),
			})
			.into());
		}

		if self.nonpositive && num > 0.0 {
			return Err(ValidationError::Number(NumberError::InvalidSign {
				sign: SmolStr::new_inline("non-positive"),
			})
			.into());
		}

		// 5. 范围检查
		if let Some(min) = self.min {
			let valid = if self.min_inclusive { num >= min } else { num > min };
			if !valid {
				return Err(ValidationError::Number(NumberError::TooSmall {
					bound: min,
					actual: num,
					inclusive: self.min_inclusive,
				})
				.into());
			}
		}

		if let Some(max) = self.max {
			let valid = if self.max_inclusive { num <= max } else { num < max };
			if !valid {
				return Err(ValidationError::Number(NumberError::TooBig {
					bound: max,
					actual: num,
					inclusive: self.max_inclusive,
				})
				.into());
			}
		}

		// 6. 运行所有自定义验证器
		for validator in &self.validators {
			if let Err(message) = (validator.func)(num) {
				return Err(ValidationError::Custom { message: SmolStr::new(message) }.into());
			}
		}

		Ok(num)
	}
}

impl Clone for NumberSchema {
	fn clone(&self) -> Self {
		Self {
			min: self.min,
			max: self.max,
			min_inclusive: self.min_inclusive,
			max_inclusive: self.max_inclusive,
			int_only: self.int_only,
			positive: self.positive,
			negative: self.negative,
			nonnegative: self.nonnegative,
			nonpositive: self.nonpositive,
			finite: self.finite,
			validators: self
				.validators
				.iter()
				.map(|v| Validator { func: Arc::clone(&v.func) })
				.collect(),
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
	use serde_json::json;

	#[test]
	fn test_number_basic() {
		let schema = NumberSchema::new();
		assert!(schema.validate(&json!(42)).is_ok());
		assert!(schema.validate(&json!(3.5)).is_ok());
		assert!(schema.validate(&json!("123")).is_err());
	}

	#[test]
	fn test_number_min_max() {
		let schema = NumberSchema::new().min(0.0).max(100.0);
		assert!(schema.validate(&json!(50)).is_ok());
		assert!(schema.validate(&json!(-1)).is_err());
		assert!(schema.validate(&json!(101)).is_err());
	}

	#[test]
	fn test_number_int() {
		let schema = NumberSchema::new().int();
		assert!(schema.validate(&json!(42)).is_ok());
		assert!(schema.validate(&json!(3.5)).is_err());
	}

	#[test]
	fn test_number_positive() {
		let schema = NumberSchema::new().positive();
		assert!(schema.validate(&json!(1)).is_ok());
		assert!(schema.validate(&json!(0)).is_err());
		assert!(schema.validate(&json!(-1)).is_err());
	}

	#[test]
	fn test_number_refine() {
		let schema = NumberSchema::new().refine(|n| {
			if n % 2.0 == 0.0 {
				Ok(())
			} else {
				Err("must be even".to_string())
			}
		});
		assert!(schema.validate(&json!(4)).is_ok());
		assert!(schema.validate(&json!(5)).is_err());
	}
}
