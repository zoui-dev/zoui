use serde_json::json;
use zoui::prelude::*;

fn main() {
    let status_schema = union()
        .variant(literal("active"))
        .variant(literal("inactive"))
        .variant(literal("pending"));

    println!("{:?}", status_schema.validate(&json!("active")));
    println!("{:?}", status_schema.validate(&json!("unknown")));


    let mixed_schema = union()
        .variant(string())
        .variant(number());

    println!("{:?}", mixed_schema.validate(&json!("hello")));
    println!("{:?}", mixed_schema.validate(&json!(42)));
    println!("{:?}", mixed_schema.validate(&json!(true)));

    let nullable_string = union()
        .variant(string())
        .variant(null());

    println!("{:?}", nullable_string.validate(&json!("hello")));
    println!("{:?}", nullable_string.validate(&json!(null)));
    println!("{:?}", nullable_string.validate(&json!(123)));

    let http_method = union()
        .variant(literal("GET"))
        .variant(literal("POST"))
        .variant(literal("PUT"))
        .variant(literal("DELETE"));

    println!("{:?}", http_method.validate(&json!("GET")));
    println!("{:?}", http_method.validate(&json!("PATCH")));

    let array_or_single = union()
        .variant(array(string()))
        .variant(string());

    println!("{:?}", array_or_single.validate(&json!(["a", "b"])));
    println!("{:?}", array_or_single.validate(&json!("single")));

    let bool_or_string = union()
        .variant(boolean())
        .variant(literal("true"))
        .variant(literal("false"));

    println!("{:?}", bool_or_string.validate(&json!(true)));
    println!("{:?}", bool_or_string.validate(&json!("true")));
    println!("{:?}", bool_or_string.validate(&json!("yes")));
}
