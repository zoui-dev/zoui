use crate::error::Result;
use serde_json::Value;

pub trait Validate<T = Value>: Clone {
	/// 验证输入值
	///
	/// 只负责验证逻辑，成功时返回类型 T
	fn validate(&self, value: &Value) -> Result<T>;

	/// 仅检查是否有效
	fn is_valid(&self, value: &Value) -> bool {
		self.validate(value).is_ok()
	}
}
