use crate::error::{Result, UnionError, ValidationError};
use crate::validate::Validate;
use serde_json::Value;
use smol_str::SmolStr;
use std::fmt::Debug;
use std::sync::Arc;

/// Union schema - 验证值是否匹配任意一个 variant
///
/// 使用 `.variant()` 方法添加可选的 schema，按顺序尝试每个 variant，
/// 返回第一个匹配的结果。如果所有 variant 都失败，则返回包含所有错误的 Union 错误。
///
/// # 示例
///
/// ```
/// use zoui::prelude::*;
/// use serde_json::json;
///
/// // 字符串 literal 的 union
/// let schema = union()
///     .variant(literal("hello"))
///     .variant(literal("world"));
/// assert!(schema.validate(&json!("hello")).is_ok());
/// assert!(schema.validate(&json!("world")).is_ok());
/// assert!(schema.validate(&json!("other")).is_err());
///
/// // 枚举风格的 literal union
/// let status = union()
///     .variant(literal("active"))
///     .variant(literal("inactive"))
///     .variant(literal("pending"));
/// assert!(status.validate(&json!("active")).is_ok());
/// assert!(status.validate(&json!("unknown")).is_err());
///
/// // 混合类型的 union
/// let schema = union()
///     .variant(string())
///     .variant(number());
/// assert!(schema.validate(&json!("hello")).is_ok());
/// assert!(schema.validate(&json!(42)).is_ok());
/// assert!(schema.validate(&json!(true)).is_err());
/// ```
pub struct UnionSchema {
    variants: Vec<Arc<dyn UnionVariant>>,
}

trait UnionVariant: Send + Sync {
    fn try_validate(&self, value: &Value) -> Result<Value>;
    fn clone_box(&self) -> Box<dyn UnionVariant>;
}

struct VariantValidator<S, T> {
    schema: S,
    _phantom: std::marker::PhantomData<T>,
}

impl<S, T> VariantValidator<S, T> {
    fn new(schema: S) -> Self {
        Self {
            schema,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S, T> UnionVariant for VariantValidator<S, T>
where
    S: Validate<T> + Send + Sync + Clone + 'static,
    T: serde::Serialize + Send + Sync + Debug + 'static,
{
    fn try_validate(&self, value: &Value) -> Result<Value> {
        let validated = self.schema.validate(value)?;
        serde_json::to_value(validated).map_err(|e| {
            ValidationError::Custom {
                message: SmolStr::new(format!("Failed to serialize validated value: {}", e)),
            }
            .into()
        })
    }

    fn clone_box(&self) -> Box<dyn UnionVariant> {
        Box::new(Self {
            schema: self.schema.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

impl Clone for UnionSchema {
    fn clone(&self) -> Self {
        Self {
            variants: self
                .variants
                .iter()
                .map(|v| Arc::from(v.clone_box()))
                .collect(),
        }
    }
}

impl UnionSchema {
    /// 创建一个空的 union schema
    pub fn new() -> Self {
        Self {
            variants: Vec::new(),
        }
    }

    pub fn variant<S, T>(mut self, schema: S) -> Self
    where
        S: Validate<T> + Send + Sync + Clone + 'static,
        T: serde::Serialize + Send + Sync + Debug + 'static,
    {
        self.variants
            .push(Arc::new(VariantValidator::new(schema)));
        self
    }
}

impl Default for UnionSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl Validate<Value> for UnionSchema {
    fn validate(&self, input: &Value) -> Result<Value> {
        let mut errors = Vec::with_capacity(self.variants.len());

        for variant in &self.variants {
            match variant.try_validate(input) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    if let crate::Error::Validation(ve) = e {
                        errors.push(ve);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(ValidationError::Union(UnionError::NoMatch { errors }).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{literal, number, string};
    use serde_json::json;

    #[test]
    fn test_union_with_literals() {
        let schema = UnionSchema::new()
            .variant(literal("active"))
            .variant(literal("inactive"));

        assert!(schema.validate(&json!("active")).is_ok());
        assert!(schema.validate(&json!("inactive")).is_ok());
        assert!(schema.validate(&json!("unknown")).is_err());
    }

    #[test]
    fn test_union_enum_style() {
        let status_schema = UnionSchema::new()
            .variant(literal("active"))
            .variant(literal("inactive"))
            .variant(literal("pending"));

        assert!(status_schema.validate(&json!("active")).is_ok());
        assert!(status_schema.validate(&json!("inactive")).is_ok());
        assert!(status_schema.validate(&json!("pending")).is_ok());
        assert!(status_schema.validate(&json!("unknown")).is_err());
    }

    #[test]
    fn test_union_preserves_value() {
        let schema = UnionSchema::new()
            .variant(literal("test"))
            .variant(literal("other"));

        let input = json!("test");
        let result = schema.validate(&input).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_empty_union_always_fails() {
        let schema = UnionSchema::new();
        assert!(schema.validate(&json!("anything")).is_err());
        assert!(schema.validate(&json!(123)).is_err());
        assert!(schema.validate(&json!(null)).is_err());
    }

    #[test]
    fn test_single_variant_union() {
        let schema = UnionSchema::new().variant(literal("only"));
        assert!(schema.validate(&json!("only")).is_ok());
        assert!(schema.validate(&json!("other")).is_err());
    }

    #[test]
    fn test_many_variants() {
        let schema = UnionSchema::new()
            .variant(literal("a"))
            .variant(literal("b"))
            .variant(literal("c"))
            .variant(literal("d"))
            .variant(literal("e"));

        assert!(schema.validate(&json!("a")).is_ok());
        assert!(schema.validate(&json!("e")).is_ok());
        assert!(schema.validate(&json!("f")).is_err());
    }

    #[test]
    fn test_union_rejects_wrong_types() {
        let schema = UnionSchema::new()
            .variant(literal("a"))
            .variant(literal("b"));

        assert!(schema.validate(&json!(123)).is_err());
        assert!(schema.validate(&json!(true)).is_err());
        assert!(schema.validate(&json!(null)).is_err());
        assert!(schema.validate(&json!([])).is_err());
        assert!(schema.validate(&json!({})).is_err());
    }

    #[test]
    fn test_union_mixed_types() {
        let schema = UnionSchema::new().variant(string()).variant(number());

        assert!(schema.validate(&json!("hello")).is_ok());
        assert!(schema.validate(&json!(42)).is_ok());
        assert!(schema.validate(&json!(3.5)).is_ok());
        assert!(schema.validate(&json!(true)).is_err());
    }
}
