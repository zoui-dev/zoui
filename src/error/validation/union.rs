use thiserror::Error;

use super::ValidationError;

/// 联合类型验证错误
#[derive(Debug, Clone, Error)]
pub enum UnionError {
    /// 没有匹配的分支
    #[error("no valid union member found")]
    NoMatch { errors: Vec<ValidationError> },
}
