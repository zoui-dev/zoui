use crate::error::{LiteralError, Result, ValidationError};
use crate::validate::Validate;
use serde_json::Value;
use smol_str::SmolStr;

/// Literal schema - 验证值是否匹配特定字面量
#[derive(Debug, Clone)]
pub struct LiteralSchema {
    value: LiteralValue,
}

#[derive(Debug, Clone)]
enum LiteralValue {
    String(SmolStr),
    Number(f64),
    Bool(bool),
}

impl LiteralSchema {
    /// 创建字符串字面量
    pub fn string(value: impl Into<SmolStr>) -> Self {
        Self {
            value: LiteralValue::String(value.into()),
        }
    }

    /// 创建数字字面量
    pub fn number(value: f64) -> Self {
        Self {
            value: LiteralValue::Number(value),
        }
    }

    /// 创建布尔字面量
    pub fn bool(value: bool) -> Self {
        Self {
            value: LiteralValue::Bool(value),
        }
    }
}

impl Validate<Value> for LiteralSchema {
    fn validate(&self, input: &Value) -> Result<Value> {
        let matches = match &self.value {
            LiteralValue::String(expected) => {
                input.as_str().map(|s| s == expected.as_str()).unwrap_or(false)
            }
            LiteralValue::Number(expected) => {
                input.as_f64().map(|n| (n - expected).abs() < f64::EPSILON).unwrap_or(false)
            }
            LiteralValue::Bool(expected) => {
                input.as_bool().map(|b| b == *expected).unwrap_or(false)
            }
        };

        if matches {
            Ok(input.clone())
        } else {
            Err(ValidationError::Literal(LiteralError::Mismatch {
                expected: format_literal(&self.value),
                actual: format_value(input),
            })
            .into())
        }
    }
}

fn format_literal(value: &LiteralValue) -> SmolStr {
    match value {
        LiteralValue::String(s) => SmolStr::new(format!("\"{}\"", s)),
        LiteralValue::Number(n) => SmolStr::new(n.to_string()),
        LiteralValue::Bool(b) => SmolStr::new(b.to_string()),
    }
}

fn format_value(value: &Value) -> SmolStr {
    match value {
        Value::String(s) => SmolStr::new(format!("\"{}\"", s)),
        Value::Number(n) => SmolStr::new(n.to_string()),
        Value::Bool(b) => SmolStr::new(b.to_string()),
        Value::Null => SmolStr::new_inline("null"),
        Value::Array(_) => SmolStr::new_inline("[array]"),
        Value::Object(_) => SmolStr::new_inline("{object}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_literal_string() {
        let schema = LiteralSchema::string("active");
        assert!(schema.validate(&json!("active")).is_ok());
        assert!(schema.validate(&json!("inactive")).is_err());
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_literal_number() {
        let schema = LiteralSchema::number(42.0);
        assert!(schema.validate(&json!(42)).is_ok());
        assert!(schema.validate(&json!(42.0)).is_ok());
        assert!(schema.validate(&json!(43)).is_err());
    }

    #[test]
    fn test_literal_bool() {
        let schema = LiteralSchema::bool(true);
        assert!(schema.validate(&json!(true)).is_ok());
        assert!(schema.validate(&json!(false)).is_err());
    }
}
