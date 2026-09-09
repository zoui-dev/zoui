use thiserror::Error;

/// 字符串验证错误
#[derive(Debug, Clone, Error)]
pub enum StringError {
	/// 长度太短
	#[error("string length must be at least {min}, but got {actual}")]
	TooShort { min: usize, actual: usize },

	/// 长度太长
	#[error("string length must be at most {max}, but got {actual}")]
	TooLong { max: usize, actual: usize },

	/// Email 格式无效
	#[error("invalid email format")]
	InvalidEmail,

	/// URL 格式无效
	#[error("invalid url format")]
	InvalidUrl,

	/// UUID 格式无效
	#[error("invalid uuid format")]
	InvalidUuid,

	/// 正则表达式不匹配
	#[error("string does not match pattern")]
	RegexMismatch,

	/// 日期时间格式无效
	#[error("invalid datetime format")]
	InvalidDatetime,

	/// IP 地址格式无效
	#[error("invalid ip format")]
	InvalidIp,
}
