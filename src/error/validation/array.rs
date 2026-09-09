use thiserror::Error;
use super::ValidationError;

#[derive(Debug, Clone, Error)]
pub enum ArrayError {
	/// 长度太短
	#[error("array length must be at least {min}, but got {actual}")]
	TooShort { min: usize, actual: usize },

	/// 长度太长
	#[error("array length must be at most {max}, but got {actual}")]
	TooLong { max: usize, actual: usize },

	/// 元素验证失败
	#[error("element at index {index} is invalid: {source}")]
	InvalidElement {
		index: usize,
		#[source]
		source: Box<ValidationError>,
	},
}

