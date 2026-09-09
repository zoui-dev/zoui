mod string;
#[doc(inline)]
pub use string::StringError;
mod number;
#[doc(inline)]
pub use number::NumberError;
mod boolean;
#[doc(inline)]
pub use boolean::BooleanError;
mod array;
#[doc(inline)]
pub use array::ArrayError;
mod object;
#[doc(inline)]
pub use object::ObjectError;
mod tuple;
#[doc(inline)]
pub use tuple::TupleError;
mod union;
#[doc(inline)]
pub use union::UnionError;
mod literal;
#[doc(inline)]
pub use literal::LiteralError;
mod null;
#[doc(inline)]
pub use null::NullError;

use thiserror::Error;
use smol_str::SmolStr;

#[derive(Debug, Clone, Error)]
pub enum ValidationError {
	/// 类型不匹配
	#[error("expected {expected}, received {received}")]
	InvalidType { expected: SmolStr, received: SmolStr },

	/// 必需值缺失
	#[error("required")]
	Required,

	/// 字符串验证错误
	#[error(transparent)]
	String(#[from] StringError),

	/// 数字验证错误
	#[error(transparent)]
	Number(#[from] NumberError),

	/// 布尔验证错误
	#[error(transparent)]
	Boolean(#[from] BooleanError),

	/// 数组验证错误
	#[error(transparent)]
	Array(#[from] ArrayError),

	/// 对象验证错误
	#[error(transparent)]
	Object(#[from] ObjectError),

	/// 元组验证错误
	#[error(transparent)]
	Tuple(#[from] TupleError),

	/// 联合类型验证错误
	#[error(transparent)]
	Union(#[from] UnionError),

	/// 字面量验证错误
	#[error(transparent)]
	Literal(#[from] LiteralError),

	/// Null 验证错误
	#[error(transparent)]
	Null(#[from] NullError),

	/// 自定义验证失败
	#[error("{message}")]
	Custom { message: SmolStr },
}