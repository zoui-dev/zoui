use smol_str::SmolStr;
use thiserror::Error;
use super::ValidationError;

#[derive(Debug, Clone, Error)]
pub enum ObjectError {
	/// 未识别的键
	#[error("unrecognized keys: {}", .keys.join(", "))]
	UnrecognizedKeys { keys: Vec<SmolStr> },

	/// 字段验证失败
	#[error("field '{field}' is invalid: {source}")]
	InvalidField {
		field: SmolStr,
		#[source]
		source: Box<ValidationError>,
	},
}

