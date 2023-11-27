use std::net::{AddrParseError, IpAddr};

/// 判断字符串是否IPv4
pub fn is_ipv4_str(s: &str) -> bool {
    let ip: Result<IpAddr, AddrParseError> = s.parse();
    ip.is_ok()
}

/// 判断字符串是否IPv4 & Mask，形如："10.1.1.1/24"
pub fn is_ipv4_mask_str(s: &str) -> bool {
    match s.split_once("/") {
        None => {
            return false;
        }
        Some((ip, mask)) => is_ipv4_str(ip) && mask.parse::<u8>().is_ok(),
    }
}
