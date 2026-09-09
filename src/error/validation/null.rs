use thiserror::Error;

/// Null 验证错误
#[derive(Debug, Clone, Error)]
pub enum NullError {
    /// 不是 null
    #[error("expected null")]
    NotNull,
}
