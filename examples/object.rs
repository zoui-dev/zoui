use serde_json::json;
use zoui::prelude::*;

fn main() {
    let user_schema = object()
        .field("name", string().min(2))
        .field("age", number().int().min(0.0))
        .field("email", string().email())
        .optional_field("bio", string().max(500));

    let valid_user = json!({
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com",
        "bio": "Software developer"
    });
    println!("{:?}", user_schema.validate(&valid_user));

    let user_no_bio = json!({
        "name": "Bob",
        "age": 25,
        "email": "bob@example.com"
    });
    println!("{:?}", user_schema.validate(&user_no_bio));

    let invalid_user = json!({
        "name": "Charlie",
        "email": "charlie@example.com"
    });
    println!("{:?}", user_schema.validate(&invalid_user));

    let strict_schema = object()
        .field("id", number().int())
        .field("name", string())
        .strict();

    let with_extra = json!({"id": 1, "name": "Test", "extra": "not allowed"});
    println!("{:?}", strict_schema.validate(&with_extra));

    let non_strict = object()
        .field("id", number().int())
        .field("name", string());
    println!("{:?}", non_strict.validate(&with_extra));

    let address_schema = object()
        .field("street", string())
        .field("city", string());

    let person_schema = object()
        .field("name", string())
        .field("address", address_schema);

    let person = json!({
        "name": "David",
        "address": {
            "street": "123 Main St",
            "city": "New York"
        }
    });
    println!("{:?}", person_schema.validate(&person));
}
