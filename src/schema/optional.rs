use crate::validate::Validate;
use crate::Result;
use serde_json::Value;
use std::marker::PhantomData;

pub struct Optional<S, T> {
    inner: S,
    _phantom: PhantomData<T>,
}

impl<S, T> Optional<S, T> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}


impl<S, T> Validate<Option<T>> for Optional<S, T>
where
    S: Validate<T>,
{
    fn validate(&self, input: &Value) -> Result<Option<T>> {
        if input.is_null() {
            Ok(None)
        } else {
            self.inner.validate(input).map(Some)
        }
    }
}

impl<S: Clone, T> Clone for Optional<S, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::string::StringSchema;
    use crate::schema::optional;
    use serde_json::json;

    #[test]
    fn test_optional_null() {
        let schema = optional(StringSchema::new());
        assert_eq!(schema.validate(&json!(null)).unwrap(), None);
    }

    #[test]
    fn test_optional_value() {
        let schema = optional(StringSchema::new());
        assert_eq!(
            schema.validate(&json!("hello")).unwrap(),
            Some("hello".to_string())
        );
    }

    #[test]
    fn test_optional_invalid() {
        let schema = optional(StringSchema::new().min(5));
        assert!(schema.validate(&json!("hi")).is_err());
    }
}
