# Zoui

Runtime validation library for Rust, inspired by Zod.

[English](README.md) | [中文](README_zh.md)

## Features

- Type-safe validation with fluent API
- Rich validators for primitives and composite types
- Custom validation with `refine()`
- Clear error messages
- Zero-copy design

## Installation

```toml
[dependencies]
zoui = "0.1"
serde_json = "1.0"
```

## Quick Start

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

## Schema Types

### Primitives

```rust
string()      // String validation
number()      // Number validation
boolean()     // Boolean validation
null()        // Null validation
literal("x")  // Literal value
```

### Composites

```rust
array(string())              // Array of strings
object()                     // Object with fields
tuple((string(), number()))  // Tuple validation
union()                      // Union types
```

### String

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

### Number

```rust
number()
    .min(0.0).max(100.0)
    .gt(0.0) / .gte(0.0) / .lt(100.0) / .lte(100.0)
    .int()
    .positive() / .negative()
    .finite()
    .refine(|n| ...)
```

### Array

```rust
array(element_schema)
    .min(1).max(10)
    .length(5)
    .nonempty()
```

### Object

```rust
object()
    .field("name", string())           // Required
    .optional_field("age", number())   // Optional
    .strict()                          // No extra fields
```

### Union

```rust
union()
    .variant(string())
    .variant(number())

// Enum-style
union()
    .variant(literal("active"))
    .variant(literal("inactive"))
```

### Tuple

```rust
tuple((string(), number(), boolean()))
```

## Examples

### User Validation

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

### Custom Validation

```rust
let even = number().int().refine(|n| {
    if n % 2.0 == 0.0 { Ok(()) } else { Err("must be even".into()) }
});

assert!(even.validate(&json!(4)).is_ok());
assert!(even.validate(&json!(5)).is_err());
```

### Parse from String

```rust
let schema = string().email();
let result = schema.parse(r#""user@example.com""#);
assert!(result.is_ok());
```

## More Examples

See [examples/](examples/) directory:

```bash
cargo run --example basic
cargo run --example object
cargo run --example array
cargo run --example union
cargo run --example advanced
```

## License

MIT