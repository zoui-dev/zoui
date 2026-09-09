# Zoui

受 Zod 启发的 Rust 运行时验证库。

[English](README.md) | [中文](README_zh.md)

## 特性

- 类型安全的流畅 API
- 丰富的基础类型和复合类型验证器
- 使用 `refine()` 自定义验证
- 清晰的错误信息
- 零拷贝设计

## 安装

```toml
[dependencies]
zoui = "0.1"
serde_json = "1.0"
```

## 快速开始

```rust
use zoui::prelude::*;
use serde_json::json;

let schema = string().min(3).email();
assert!(schema.validate(&json!("user@example.com")).is_ok());

let schema = number().int().min(0.0).max(100.0);
assert!(schema.validate(&json!(42)).is_ok());

let schema = object()
    .field("name", string().min(1))
    .field("age", number().int())
    .field("email", string().email());

let data = json!({
    "name": "Alice",
    "age": 25,
    "email": "alice@example.com"
});
assert!(schema.validate(&data).is_ok());
```

## Schema 类型

### 基础类型

```rust
string()      // 字符串验证
number()      // 数字验证
boolean()     // 布尔验证
null()        // Null 验证
literal("x")  // 字面量
```

### 复合类型

```rust
array(string())              // 字符串数组
object()                     // 对象字段
tuple((string(), number()))  // 元组验证
union()                      // 联合类型
```

### 字符串

```rust
string()
    .min(3).max(50)
    .email()
    .url()
    .uuid()
    .ip() / .ipv4() / .ipv6()
    .regex(regex::Regex::new(r"^\d+$").unwrap())
    .trim()
    .to_lowercase() / .to_uppercase()
    .refine(|s| ...)
```

### 数字

```rust
number()
    .min(0.0).max(100.0)
    .gt(0.0) / .gte(0.0) / .lt(100.0) / .lte(100.0)
    .int()
    .positive() / .negative()
    .finite()
    .refine(|n| ...)
```

### 数组

```rust
array(element_schema)
    .min(1).max(10)
    .length(5)
    .nonempty()
```

### 对象

```rust
object()
    .field("name", string())           // 必需字段
    .optional_field("age", number())   // 可选字段
    .strict()                          // 不允许额外字段
```

### 联合类型

```rust
union()
    .variant(string())
    .variant(number())

// 枚举风格
union()
    .variant(literal("active"))
    .variant(literal("inactive"))
```

### 元组

```rust
tuple((string(), number(), boolean()))
```

## 示例

### 用户验证

```rust
let user_schema = object()
    .field("id", number().int().positive())
    .field("username", string().min(3).max(20))
    .field("email", string().email())
    .optional_field("age", number().int().min(0.0))
    .field("tags", array(string()).max(5))
    .strict();

let data = json!({
    "id": 1,
    "username": "alice",
    "email": "alice@example.com",
    "tags": ["rust"]
});

match user_schema.validate(&data) {
    Ok(user) => println!("{:?}", user),
    Err(e) => println!("{}", e),
}
```

### 自定义验证

```rust
let even = number().int().refine(|n| {
    if n % 2.0 == 0.0 { Ok(()) } else { Err("must be even".into()) }
});

assert!(even.validate(&json!(4)).is_ok());
assert!(even.validate(&json!(5)).is_err());
```

### 从字符串解析

```rust
let schema = string().email();
let result = schema.parse(r#""user@example.com""#);
assert!(result.is_ok());
```

## 更多示例

查看 [examples/](examples/) 目录：

```bash
cargo run --example basic
cargo run --example object
cargo run --example array
cargo run --example union
cargo run --example advanced
```

## 许可证

MIT
