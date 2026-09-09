use thiserror::Error;

/// 布尔验证错误
#[derive(Debug, Clone, Error)]
pub enum BooleanError {
    /// 必须为真
    #[error("value must be true")]
    MustBeTrue,

    /// 必须为假
    #[error("value must be false")]
    MustBeFalse,
}
