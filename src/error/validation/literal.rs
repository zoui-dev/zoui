use smol_str::SmolStr;
use thiserror::Error;

/// 字面量验证错误
#[derive(Debug, Clone, Error)]
pub enum LiteralError {
    /// 值不匹配字面量
    #[error("expected literal value '{expected}', but got '{actual}'")]
    Mismatch { expected: SmolStr, actual: SmolStr },
}
