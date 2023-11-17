use crate::consts::LOCALHOST;
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;
use std::net::IpAddr;

lazy_static! {
    static ref ADDR_REGEX: Regex =
        Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}:[0-9]{1,5}$").unwrap();
}

pub fn check_addr(addr: &str) -> bool {
    ADDR_REGEX.is_match(addr)
}

pub fn local_ip() -> IpAddr {
    match local_ip_address::local_ip() {
        Ok(ip) => ip,
        Err(err) => {
            warn!("Get local ip address failed: {}", err);
            LOCALHOST.parse().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ip::check_addr;
    use local_ip_address::local_ip;

    #[test]
    fn test_check_addr() {
        assert!(check_addr("127.0.0.1:8080"));

        assert!(!check_addr("127.0.1:8080"));

        assert!(!check_addr("127.0.0.1"));
    }

    #[test]
    fn test_ip() {
        println!("This is my local IP address: {:?}", local_ip().unwrap());
    }
}
