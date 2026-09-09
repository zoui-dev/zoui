use crate::error::{NullError, Result, ValidationError};
use crate::validate::Validate;
use serde_json::Value;

/// Null schema
#[derive(Debug, Clone)]
pub struct NullSchema;

impl NullSchema {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NullSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Validate<()> for NullSchema {
    fn validate(&self, input: &Value) -> Result<()> {
        if input.is_null() {
            Ok(())
        } else {
            Err(ValidationError::Null(NullError::NotNull).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_null_basic() {
        let schema = NullSchema::new();
        assert!(schema.validate(&json!(null)).is_ok());
        assert!(schema.validate(&json!(0)).is_err());
        assert!(schema.validate(&json!("")).is_err());
        assert!(schema.validate(&json!(false)).is_err());
    }
}
