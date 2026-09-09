use validator::ValidateUrl;

/// 验证 URL 格式
#[inline]
pub fn is_url(s: &str) -> bool {
    ValidateUrl::validate_url(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        assert!(is_url("http://example.com"));
        assert!(is_url("https://example.com"));
        assert!(is_url("https://example.com/path?query=value"));
    }

    #[test]
    fn test_invalid_urls() {
        assert!(!is_url("example.com"));
        assert!(!is_url("not a url"));
        assert!(!is_url(""));
    }
}
