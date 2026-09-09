use uuid::Uuid;

/// 验证 UUID 格式（所有版本）
#[inline]
pub fn is_uuid(s: &str) -> bool {
    Uuid::parse_str(s).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uuid() {
        assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(is_uuid("6ba7b810-9dad-11d1-80b4-00c04fd430c8"));
        assert!(is_uuid("00000000-0000-0000-0000-000000000000"));
    }

    #[test]
    fn test_invalid_uuid() {
        assert!(!is_uuid("invalid-uuid"));
        assert!(!is_uuid("550e8400-e29b-41d4-a716"));
        // 注意：uuid crate 可以解析没有分隔符的 UUID，所以这个是有效的
        // assert!(!is_uuid("550e8400e29b41d4a716446655440000"));
        assert!(!is_uuid("550e8400-e29b-41d4-a716-446655440000-extra"));
        assert!(!is_uuid("gggggggg-gggg-gggg-gggg-gggggggggggg")); // 非十六进制
        assert!(!is_uuid(""));
    }
}
