use serde_json::json;
use zoui::prelude::*;

fn main() {
    let name_schema = string().min(2).max(50);
    println!("{:?}", name_schema.validate(&json!("Alice")));
    println!("{:?}", name_schema.validate(&json!("A")));

    let age_schema = number().int().min(0.0).max(120.0);
    println!("{:?}", age_schema.validate(&json!(25)));
    println!("{:?}", age_schema.validate(&json!(150)));

    let bool_schema = boolean();
    println!("{:?}", bool_schema.validate(&json!(true)));

    let email_schema = string().email();
    println!("{:?}", email_schema.validate(&json!("user@example.com")));
    println!("{:?}", email_schema.validate(&json!("not-an-email")));
}
