use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
	/// JSON 格式错误
	#[error("invalid json: {0}")]
	Json(#[from] serde_json::Error),

    /// IO 错误
	#[error(transparent)]
	Io(#[from] std::io::Error),

	/// 其他解析错误
	#[error("{0}")]
	Other(SmolStr),
}
