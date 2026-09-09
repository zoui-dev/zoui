use thiserror::Error;
use super::ValidationError;

/// 元组验证错误
#[derive(Debug, Clone, Error)]
pub enum TupleError {
    /// 长度不匹配
    #[error("expected tuple of length {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },

    /// 元素验证失败
    #[error("element at index {index} is invalid: {source}")]
    InvalidElement {
        index: usize,
        #[source]
        source: Box<ValidationError>,
    },
}
