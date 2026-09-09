use crate::error::{StringError, ValidationError};
use crate::validate::Validate;
use serde_json::Value;
use smol_str::SmolStr;
use std::sync::Arc;

pub struct StringSchema {
    min: Option<usize>,
    max: Option<usize>,
    validators: Vec<Validator>,
    transform: Option<StringTransform>,
}

type ValidatorFn = Arc<dyn Fn(&str) -> std::result::Result<(), String> + Send + Sync>;

struct Validator {
    func: ValidatorFn,
}

#[derive(Clone, Copy)]
enum StringTransform {
    Trim,
    ToLowerCase,
    ToUpperCase,
}

impl StringSchema {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            validators: Vec::new(),
            transform: None,
        }
    }

    // Builder 方法
    pub fn min(mut self, len: usize) -> Self {
        self.min = Some(len);
        self
    }

    pub fn max(mut self, len: usize) -> Self {
        self.max = Some(len);
        self
    }

    pub fn length(self, len: usize) -> Self {
        self.min(len).max(len)
    }

    pub fn nonempty(self) -> Self {
        self.min(1)
    }

    // 通用验证方法
    pub fn refine<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> std::result::Result<(), String> + Send + Sync + 'static,
    {
        self.validators.push(Validator {
            func: Arc::new(validator),
        });
        self
    }

    // 便捷方法
    pub fn email(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_email(s) {
                Ok(())
            } else {
                Err("invalid email format".to_string())
            }
        })
    }

    pub fn url(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_url(s) {
                Ok(())
            } else {
                Err("invalid url format".to_string())
            }
        })
    }

    pub fn uuid(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_uuid(s) {
                Ok(())
            } else {
                Err("invalid uuid format".to_string())
            }
        })
    }


    pub fn datetime(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_datetime(s) {
                Ok(())
            } else {
                Err("invalid datetime format".to_string())
            }
        })
    }

    pub fn ip(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_ip(s) {
                Ok(())
            } else {
                Err("invalid ip address".to_string())
            }
        })
    }

    pub fn ipv4(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_ipv4(s) {
                Ok(())
            } else {
                Err("invalid ipv4 address".to_string())
            }
        })
    }

    pub fn ipv6(self) -> Self {
        self.refine(|s| {
            if crate::validator::is_ipv6(s) {
                Ok(())
            } else {
                Err("invalid ipv6 address".to_string())
            }
        })
    }

    pub fn starts_with(self, prefix: impl Into<String>) -> Self {
        let prefix = prefix.into();
        self.refine(move |s: &str| {
            if s.starts_with(&prefix) {
                Ok(())
            } else {
                Err(format!("must start with '{}'", prefix))
            }
        })
    }

    pub fn ends_with(self, suffix: impl Into<String>) -> Self {
        let suffix = suffix.into();
        self.refine(move |s: &str| {
            if s.ends_with(&suffix) {
                Ok(())
            } else {
                Err(format!("must end with '{}'", suffix))
            }
        })
    }

    pub fn includes(self, substring: impl Into<String>) -> Self {
        let substring = substring.into();
        self.refine(move |s: &str| {
            if s.contains(&substring) {
                Ok(())
            } else {
                Err(format!("must include '{}'", substring))
            }
        })
    }

    pub fn regex(self, pattern: regex::Regex) -> Self {
        let pattern_str = pattern.as_str().to_string();

        self.refine(move |s: &str| {
            if pattern.is_match(s) {
                Ok(())
            } else {
                Err(format!("does not match pattern: {}", pattern_str))
            }
        })
    }

    pub fn trim(mut self) -> Self {
        self.transform = Some(StringTransform::Trim);
        self
    }

    pub fn to_lowercase(mut self) -> Self {
        self.transform = Some(StringTransform::ToLowerCase);
        self
    }

    pub fn to_uppercase(mut self) -> Self {
        self.transform = Some(StringTransform::ToUpperCase);
        self
    }
}

impl Default for StringSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Validate<String> for StringSchema {
    fn validate(&self, input: &Value) -> crate::error::Result<String> {
        let mut s = input
            .as_str()
            .ok_or_else(|| ValidationError::InvalidType {
                expected: SmolStr::new_inline("string"),
                received: type_name(input),
            })?
            .to_string();

        if let Some(transform) = self.transform {
            s = match transform {
                StringTransform::Trim => s.trim().to_string(),
                StringTransform::ToLowerCase => s.to_lowercase(),
                StringTransform::ToUpperCase => s.to_uppercase(),
            };
        }

        if let Some(min) = self.min
            && s.len() < min {
                return Err(ValidationError::String(StringError::TooShort {
                    min,
                    actual: s.len(),
                })
                .into());
            }

        if let Some(max) = self.max
            && s.len() > max {
                return Err(ValidationError::String(StringError::TooLong {
                    max,
                    actual: s.len(),
                })
                .into());
            }

        for validator in &self.validators {
            if let Err(message) = (validator.func)(&s) {
                return Err(ValidationError::Custom {
                    message: SmolStr::new(message),
                }
                .into());
            }
        }

        Ok(s)
    }
}

impl Clone for StringSchema {
    fn clone(&self) -> Self {
        Self {
            min: self.min,
            max: self.max,
            validators: self
                .validators
                .iter()
                .map(|v| Validator {
                    func: Arc::clone(&v.func),
                })
                .collect(),
            transform: self.transform,
        }
    }
}

fn type_name(value: &Value) -> SmolStr {
    SmolStr::new(match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_basic() {
        let schema = StringSchema::new();
        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!(123)).is_err());
    }

    #[test]
    fn test_string_min_max() {
        let schema = StringSchema::new().min(3).max(10);
        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!("hi")).is_err());
        assert!(schema.validate(&json!("this is too long")).is_err());
    }

    #[test]
    fn test_string_email() {
        let schema = StringSchema::new().email();
        assert!(schema.validate(&json!("test@example.com")).is_ok());
        assert!(schema.validate(&json!("invalid")).is_err());
    }

    #[test]
    fn test_string_transform() {
        let schema = StringSchema::new().trim().min(3);
        assert!(schema.validate(&json!("  hello  ")).is_ok());
        assert_eq!(schema.validate(&json!("  hello  ")).unwrap(), "hello");
    }

    #[test]
    fn test_string_refine() {
        let schema = StringSchema::new().refine(|s| {
            if s.contains("test") {
                Ok(())
            } else {
                Err("must contain 'test'".to_string())
            }
        });
        assert!(schema.validate(&json!("this is a test")).is_ok());
        assert!(schema.validate(&json!("no match")).is_err());
    }
}
