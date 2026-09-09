use serde_json::json;
use zoui::prelude::*;

fn main() {
    let tags_schema = array(string()).min(1).max(5);
    println!("{:?}", tags_schema.validate(&json!(["rust", "validation"])));
    println!("{:?}", tags_schema.validate(&json!([])));

    let scores_schema = array(number().min(0.0).max(100.0)).nonempty();
    println!("{:?}", scores_schema.validate(&json!([85, 92, 78])));
    println!("{:?}", scores_schema.validate(&json!([85, 105, 78])));

    let user_schema = object()
        .field("id", number().int())
        .field("name", string());

    let users_schema = array(user_schema).min(1);
    let users = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]);
    println!("{:?}", users_schema.validate(&users));

    let matrix_schema = array(array(number())).min(2);
    let matrix = json!([[1, 2, 3], [4, 5, 6]]);
    println!("{:?}", matrix_schema.validate(&matrix));

    let rgb_schema = array(number().int().min(0.0).max(255.0)).length(3);
    println!("{:?}", rgb_schema.validate(&json!([255, 128, 64])));
    println!("{:?}", rgb_schema.validate(&json!([255, 128])));
}
