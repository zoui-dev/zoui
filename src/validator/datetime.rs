pub fn is_datetime(s: &str) -> bool {
    // 简单验证：包含 'T' 和可选的时区
    s.contains('T') && (s.contains('Z') || s.contains('+') || s.contains('-'))
}