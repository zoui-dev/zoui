use serde_json::json;
use zoui::prelude::*;

fn main() {
    let config_schema = object()
        .field("host", string())
        .field("port", number().int())
        .optional_field("timeout", number().int());

    let config = json!({"host": "localhost", "port": 8080});
    println!("{:?}", config_schema.validate(&config));

    let coordinate_schema = tuple((number(), number()));
    println!("{:?}", coordinate_schema.validate(&json!([3.5, 7.2])));

    let rgb_tuple = tuple((
        number().int().min(0.0).max(255.0),
        number().int().min(0.0).max(255.0),
        number().int().min(0.0).max(255.0),
    ));
    println!("{:?}", rgb_tuple.validate(&json!([255, 128, 64])));

    let name_schema = string().trim().to_lowercase().min(2);
    println!("{:?}", name_schema.validate(&json!("  ALICE  ")));

    let even_number = number().int().refine(|n| {
        if n % 2.0 == 0.0 {
            Ok(())
        } else {
            Err("must be even".to_string())
        }
    });
    println!("{:?}", even_number.validate(&json!(4)));
    println!("{:?}", even_number.validate(&json!(5)));

    let post_schema = object()
        .field("id", number().int())
        .field("title", string().min(1))
        .field(
            "author",
            object()
                .field("id", number().int())
                .field("name", string()),
        )
        .field("tags", array(string()).min(1))
        .optional_field("published", boolean());

    let post = json!({
        "id": 1,
        "title": "Hello World",
        "author": {"id": 100, "name": "Alice"},
        "tags": ["rust", "validation"],
        "published": true
    });
    println!("{:?}", post_schema.validate(&post));

    let api_response = object()
        .field("success", boolean())
        .field(
            "data",
            union()
                .variant(object().field("id", number().int()).field("message", string()))
                .variant(null()),
        )
        .optional_field("error", string());

    let success_resp = json!({
        "success": true,
        "data": {"id": 42, "message": "OK"}
    });
    println!("{:?}", api_response.validate(&success_resp));

    let error_resp = json!({
        "success": false,
        "data": null,
        "error": "Server error"
    });
    println!("{:?}", api_response.validate(&error_resp));
}
