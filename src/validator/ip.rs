use validator::ValidateIp;

/// 验证 IP 地址格式（v4 或 v6）
#[inline]
pub fn is_ip(s: &str) -> bool {
    ValidateIp::validate_ip(&s)
}

/// 验证 IPv4 格式
#[inline]
pub fn is_ipv4(s: &str) -> bool {
    ValidateIp::validate_ipv4(&s)
}

/// 验证 IPv6 格式
#[inline]
pub fn is_ipv6(s: &str) -> bool {
    ValidateIp::validate_ipv6(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4() {
        assert!(is_ipv4("192.168.1.1"));
        assert!(is_ipv4("0.0.0.0"));
        assert!(is_ipv4("255.255.255.255"));
        assert!(!is_ipv4("256.1.1.1"));
        assert!(!is_ipv4("192.168.1"));
        assert!(!is_ipv4("192.168.1.1.1"));
    }

    #[test]
    fn test_ipv6() {
        assert!(is_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(is_ipv6("::1"));
        assert!(is_ipv6("fe80::"));
        assert!(is_ipv6("::"));
        assert!(!is_ipv6("gggg::1"));
        assert!(!is_ipv6("192.168.1.1"));
    }

    #[test]
    fn test_ip() {
        assert!(is_ip("192.168.1.1"));
        assert!(is_ip("::1"));
        assert!(!is_ip("not an ip"));
    }
}
