use validator::ValidateEmail;


#[inline]
pub fn is_email(s: &str) -> bool {
    ValidateEmail::validate_email(&s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(is_email("test@example.com"));
        assert!(is_email("user.name@example.com"));
        assert!(is_email("user+tag@sub.example.com"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_email(""));
        assert!(!is_email("test"));
        assert!(!is_email("@example.com"));
        assert!(!is_email("test@"));
        assert!(!is_email("test@@example.com"));
        assert!(!is_email("test@.com"));
        assert!(!is_email("test@example."));
        assert!(!is_email("test@example..com"));
        assert!(!is_email("a".repeat(65).as_str())); 
    }
}