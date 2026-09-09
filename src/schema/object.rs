use crate::error::{ObjectError, Result, ValidationError};
use crate::validate::Validate;
use serde_json::{Map, Value};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ObjectSchema {
    fields: HashMap<SmolStr, Arc<dyn ObjectFieldValidator>>,
    strict: bool,
}

impl ObjectSchema {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            strict: false,
        }
    }

    /// 添加必需字段
    pub fn field<S, T>(mut self, name: impl Into<SmolStr>, schema: S) -> Self
    where
        S: Validate<T> + Send + Sync + 'static,
        T: serde::Serialize + Send + Sync + Debug + 'static,
    {
        self.fields.insert(
            name.into(),
            Arc::new(RequiredFieldValidator::new(schema)),
        );
        self
    }

    /// 添加可选字段
    pub fn optional_field<S, T>(mut self, name: impl Into<SmolStr>, schema: S) -> Self
    where
        S: Validate<T> + Send + Sync + 'static,
        T: serde::Serialize + Send + Sync + Debug + 'static,
    {
        self.fields.insert(
            name.into(),
            Arc::new(OptionalFieldValidator::new(schema)),
        );
        self
    }

    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }
}

impl Default for ObjectSchema {
    fn default() -> Self {
        Self::new()
    }
}

trait ObjectFieldValidator: Send + Sync {
    fn validate_field(&self, value: Option<&Value>) -> Result<Value>;
    fn clone_box(&self) -> Box<dyn ObjectFieldValidator>;
}

#[derive(Debug)]
struct RequiredFieldValidator<S, T> {
    schema: S,
    _phantom: PhantomData<T>,
}

impl<S, T> RequiredFieldValidator<S, T> {
    fn new(schema: S) -> Self {
        Self {
            schema,
            _phantom: PhantomData,
        }
    }
}

impl<S, T> ObjectFieldValidator for RequiredFieldValidator<S, T>
where
    S: Validate<T> + Send + Sync + Clone + 'static,
    T: serde::Serialize + Send + Sync + Debug + 'static,
{
    fn validate_field(&self, value: Option<&Value>) -> Result<Value> {
        match value {
            Some(v) => {
                let validated = self.schema.validate(v)?;
                serde_json::to_value(validated).map_err(|e| {
                    ValidationError::Custom {
                        message: SmolStr::new(format!("Failed to serialize validated value: {}", e)),
                    }
                    .into()
                })
            }
            None => Err(ValidationError::Required.into()),
        }
    }

    fn clone_box(&self) -> Box<dyn ObjectFieldValidator> {
        Box::new(Self {
            schema: self.schema.clone(),
            _phantom: PhantomData,
        })
    }
}

#[derive(Debug)]
struct OptionalFieldValidator<S, T> {
    schema: S,
    _phantom: PhantomData<T>,
}

impl<S, T> OptionalFieldValidator<S, T> {
    fn new(schema: S) -> Self {
        Self {
            schema,
            _phantom: PhantomData,
        }
    }
}

impl<S, T> ObjectFieldValidator for OptionalFieldValidator<S, T>
where
    S: Validate<T> + Send + Sync + Clone + 'static,
    T: serde::Serialize + Send + Sync + Debug + 'static,
{
    fn validate_field(&self, value: Option<&Value>) -> Result<Value> {
        match value {
            Some(v) if !v.is_null() => {
                let validated = self.schema.validate(v)?;
                serde_json::to_value(validated).map_err(|e| {
                    ValidationError::Custom {
                        message: SmolStr::new(format!("Failed to serialize validated value: {}", e)),
                    }
                    .into()
                })
            }
            _ => Ok(Value::Null),
        }
    }

    fn clone_box(&self) -> Box<dyn ObjectFieldValidator> {
        Box::new(Self {
            schema: self.schema.clone(),
            _phantom: PhantomData,
        })
    }
}

impl Clone for ObjectSchema {
    fn clone(&self) -> Self {
        Self {
            fields: self
                .fields
                .iter()
                .map(|(k, v)| (k.clone(), Arc::from(v.clone_box())))
                .collect(),
            strict: self.strict,
        }
    }
}

impl Validate<Map<String, Value>> for ObjectSchema {
    fn validate(&self, input: &Value) -> Result<Map<String, Value>> {
        let obj = input.as_object().ok_or_else(|| ValidationError::InvalidType {
            expected: SmolStr::new_inline("object"),
            received: type_name(input),
        })?;

        let mut result = Map::new();

        for (field_name, validator) in &self.fields {
            let field_value = obj.get(field_name.as_str());

            let validated = validator.validate_field(field_value).map_err(|e| match e {
                crate::Error::Validation(ve) => {
                    ValidationError::Object(ObjectError::InvalidField {
                        field: field_name.clone(),
                        source: Box::new(ve),
                    })
                    .into()
                }
                other => other,
            })?;

            result.insert(field_name.to_string(), validated);
        }

        if self.strict {
            let unrecognized: Vec<SmolStr> = obj
                .keys()
                .filter(|k| !self.fields.contains_key(k.as_str()))
                .map(SmolStr::new)
                .collect();

            if !unrecognized.is_empty() {
                return Err(
                    ValidationError::Object(ObjectError::UnrecognizedKeys { keys: unrecognized })
                        .into(),
                );
            }
        }

        Ok(result)
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
    use crate::schema::{NumberSchema, StringSchema};
    use serde_json::json;

    #[test]
    fn test_object_basic() {
        let schema = ObjectSchema::new()
            .field("name", StringSchema::new())
            .field("age", NumberSchema::new());

        let valid = json!({"name": "Alice", "age": 30});
        assert!(schema.validate(&valid).is_ok());

        let invalid = json!({"name": "Alice"});
        assert!(schema.validate(&invalid).is_err());
    }

    #[test]
    fn test_object_optional_field() {
        let schema = ObjectSchema::new()
            .field("name", StringSchema::new())
            .optional_field("age", NumberSchema::new());

        let valid = json!({"name": "Alice"});
        assert!(schema.validate(&valid).is_ok());

        let with_age = json!({"name": "Alice", "age": 30});
        assert!(schema.validate(&with_age).is_ok());
    }

    #[test]
    fn test_object_strict() {
        let schema = ObjectSchema::new()
            .field("name", StringSchema::new())
            .strict();

        let valid = json!({"name": "Alice"});
        assert!(schema.validate(&valid).is_ok());

        let extra = json!({"name": "Alice", "extra": "field"});
        assert!(schema.validate(&extra).is_err());
    }

    #[test]
    fn test_object_not_strict() {
        let schema = ObjectSchema::new().field("name", StringSchema::new());

        let extra = json!({"name": "Alice", "extra": "field"});
        assert!(schema.validate(&extra).is_ok());
    }
}
