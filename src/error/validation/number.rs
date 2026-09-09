use smol_str::SmolStr;
use thiserror::Error;

/// 数字验证错误
#[derive(Debug, Clone, Error)]
pub enum NumberError {
	/// 数字太小
	#[error("number must be {}{bound}, but got {actual}",
            if *.inclusive { ">= " } else { "> " })]
	TooSmall { bound: f64, actual: f64, inclusive: bool },

	/// 数字太大
	#[error("number must be {}{bound}, but got {actual}",
            if *.inclusive { "<= " } else { "< " })]
	TooBig { bound: f64, actual: f64, inclusive: bool },

	/// 不是整数
	#[error("expected integer, but got float")]
	NotInteger,

	/// 不是有限数
	#[error("number must be finite")]
	NotFinite,

	/// 不满足符号要求
	#[error("number must be {sign}")]
	InvalidSign { sign: SmolStr },

	/// 不是指定倍数
	#[error("number must be a multiple of {multiple_of}")]
	NotMultipleOf { multiple_of: f64 },
}
