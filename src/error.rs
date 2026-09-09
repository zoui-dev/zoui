mod validation;
#[doc(inline)]
pub use validation::*;
mod parse;
#[doc(inline)]
pub use parse::ParseError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 验证错误
	#[error(transparent)]
	Validation(#[from] ValidationError),

	/// 解析错误
	#[error(transparent)]
	Parse(#[from] ParseError),

	/// IO 错误
	#[error(transparent)]
	Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
