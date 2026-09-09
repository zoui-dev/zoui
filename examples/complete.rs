use serde_json::json;
use zoui::prelude::*;

fn main() {
    println!("=== 完整应用示例：用户注册验证 ===\n");

    // 构建用户注册 schema
    let registration_schema = object()
        .field("username", string().min(3).max(20))
        .field("email", string().email())
        .field("password", string().min(8))
        .field("age", number().int().min(13.0))
        .optional_field("phone", string())
        .optional_field("newsletter", boolean())
        .strict();

    // 测试用例 1: 完整的有效数据
    println!("测试 1: 完整有效的注册数据");
    let valid_registration = json!({
        "username": "alice123",
        "email": "alice@example.com",
        "password": "SecurePass123",
        "age": 25,
        "phone": "+1234567890",
        "newsletter": true
    });

    match registration_schema.validate(&valid_registration) {
        Ok(user) => {
            println!("✓ 注册数据验证成功");
            println!("用户名: {}", user["username"]);
            println!("邮箱: {}", user["email"]);
            println!("年龄: {}", user["age"]);
        }
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 2: 缺少可选字段
    println!("测试 2: 只包含必需字段");
    let minimal_registration = json!({
        "username": "bob456",
        "email": "bob@example.com",
        "password": "MyPassword1",
        "age": 18
    });

    match registration_schema.validate(&minimal_registration) {
        Ok(user) => {
            println!("✓ 注册数据验证成功");
            println!("用户名: {}", user["username"]);
        }
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 3: 用户名太短
    println!("测试 3: 用户名太短（小于3个字符）");
    let short_username = json!({
        "username": "ab",
        "email": "user@example.com",
        "password": "password123",
        "age": 20
    });

    match registration_schema.validate(&short_username) {
        Ok(_) => println!("✓ 验证成功"),
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 4: 无效的邮箱
    println!("测试 4: 无效的邮箱格式");
    let invalid_email = json!({
        "username": "charlie",
        "email": "not-an-email",
        "password": "password123",
        "age": 22
    });

    match registration_schema.validate(&invalid_email) {
        Ok(_) => println!("✓ 验证成功"),
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 5: 密码太短
    println!("测试 5: 密码太短（小于8个字符）");
    let short_password = json!({
        "username": "david",
        "email": "david@example.com",
        "password": "pass",
        "age": 25
    });

    match registration_schema.validate(&short_password) {
        Ok(_) => println!("✓ 验证成功"),
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 6: 年龄不够
    println!("测试 6: 年龄不够（小于13岁）");
    let too_young = json!({
        "username": "kid123",
        "email": "kid@example.com",
        "password": "password123",
        "age": 10
    });

    match registration_schema.validate(&too_young) {
        Ok(_) => println!("✓ 验证成功"),
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 7: 包含额外字段（严格模式）
    println!("测试 7: 包含未定义的字段（严格模式）");
    let extra_fields = json!({
        "username": "eve789",
        "email": "eve@example.com",
        "password": "password123",
        "age": 28,
        "extra_field": "not allowed"
    });

    match registration_schema.validate(&extra_fields) {
        Ok(_) => println!("✓ 验证成功"),
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}\n", "=".repeat(50));

    // 测试用例 8: 缺少必需字段
    println!("测试 8: 缺少必需字段（password）");
    let missing_field = json!({
        "username": "frank",
        "email": "frank@example.com",
        "age": 30
    });

    match registration_schema.validate(&missing_field) {
        Ok(_) => println!("✓ 验证成功"),
        Err(e) => println!("✗ 验证失败: {}", e),
    }

    println!("\n{}", "=".repeat(50));
    println!("\n✨ 示例完成！");
}
