use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ADDR_REGEX: Regex =
        Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}$").unwrap();
}

pub fn check_addr(addr: &str) -> bool {
    ADDR_REGEX.is_match(addr)
}

#[cfg(test)]
mod tests {
    use crate::utils::ip::check_addr;

    #[test]
    fn test_check_addr() {
        assert!(check_addr("127.0.0.1:8080"));

        assert!(!check_addr("127.0.1:8080"));

        assert!(!check_addr("127.0.0.1"));
    }
}
