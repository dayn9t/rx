use crate::prelude::Endpoint;
use regex::Regex;
use rx_core::text::BoxResult;
use rx_core::time::NaiveDateTime;
use std::str::FromStr;

pub fn parse_time(datetime_str: &str) -> BoxResult<NaiveDateTime> {
    let format = "%Y/%m/%d %H:%M:%S";
    NaiveDateTime::parse_from_str(datetime_str, format).map_err(Into::into)
}

pub const TIME: &str = r"(\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2})";
pub const CONTENT: &str = r"\[([^\]]+)\]";

/// 用户连接消息
#[derive(Debug)]
pub struct UserConnectionMsg {
    pub time: NaiveDateTime,
    pub id: String,
    pub name: String,
    pub endpoint: Endpoint,
}

impl FromStr for UserConnectionMsg {
    type Err = anyhow::Error;

    fn from_str(msg_str: &str) -> Result<Self, Self::Err> {
        // "2024/08/15 15:15:53 [I] [proxy.go:199] [671e15174eb9423f] [name-51009] get a user connection [91.92.249.181:40224]"
        let pattern = format!(
            "{} {} {} {} {} get a user connection {}",
            TIME, CONTENT, CONTENT, CONTENT, CONTENT, CONTENT
        );
        let re = Regex::new(&pattern)?;

        if let Some(caps) = re.captures(msg_str) {
            Ok(Self {
                time: parse_time(&caps[1])?,
                id: caps[4].to_string(),
                name: caps[5].to_string(),
                endpoint: Endpoint::from_str(&caps[6])?,
            })
        } else {
            Err(anyhow::anyhow!(
                "Invalid user connection message: {}",
                msg_str
            ))
        }
    }
}

/// 代理存在消息
#[derive(Debug)]
pub struct ProxyExistMsg {
    pub time: NaiveDateTime,
    pub id: String,
    pub name: String,
    pub protocol: String,
}

impl FromStr for ProxyExistMsg {
    type Err = anyhow::Error;

    fn from_str(msg_str: &str) -> Result<Self, Self::Err> {
        // "2024/08/15 15:15:53 [W] [control.go:493] [c34d45ac838c9177] new proxy [name-51054] type [tcp] error: proxy [name-51054] already exists"
        let pattern = format!(
            "{} {} {} {} new proxy {} type {} error: proxy {} already exists",
            TIME, CONTENT, CONTENT, CONTENT, CONTENT, CONTENT, CONTENT
        );
        let re = Regex::new(&pattern)?;

        if let Some(caps) = re.captures(msg_str) {
            Ok(Self {
                time: parse_time(&caps[1])?,
                id: caps[4].to_string(),
                name: caps[5].to_string(),
                protocol: caps[6].to_string(),
            })
        } else {
            Err(anyhow::anyhow!(
                "Invalid user connection message: {}",
                msg_str
            ))
        }
    }
}

/// 代理成功消息
#[derive(Debug)]
pub struct ProxySuccessMsg {
    pub time: NaiveDateTime,
    pub id: String,
    pub name: String,
    pub protocol: String,
}

impl FromStr for ProxySuccessMsg {
    type Err = anyhow::Error;

    fn from_str(msg_str: &str) -> Result<Self, Self::Err> {
        // "2024/08/15 04:13:59 [I] [control.go:497] [b4f12fbff7c9de0f] new proxy [name-51011] type [tcp] success"
        let pattern = format!(
            "{} {} {} {} new proxy {} type {} success",
            TIME, CONTENT, CONTENT, CONTENT, CONTENT, CONTENT
        );
        let re = Regex::new(&pattern)?;

        if let Some(caps) = re.captures(msg_str) {
            Ok(Self {
                time: parse_time(&caps[1])?,
                id: caps[4].to_string(),
                name: caps[5].to_string(),
                protocol: caps[6].to_string(),
            })
        } else {
            Err(anyhow::anyhow!(
                "Invalid user connection message: {}",
                msg_str
            ))
        }
    }
}

/// 代理监听消息
#[derive(Debug)]
pub struct ProxyListenMsg {
    pub time: NaiveDateTime,
    pub id: String,
    pub name: String,
    pub port: u16,
}

impl FromStr for ProxyListenMsg {
    type Err = anyhow::Error;

    fn from_str(msg_str: &str) -> Result<Self, Self::Err> {
        // "2024/08/15 04:13:59 [I] [tcp.go:81] [b4f12fbff7c9de0f] [name-51011] tcp proxy listen port [51011]"
        let pattern = format!(
            "{} {} {} {} {} tcp proxy listen port {}",
            TIME, CONTENT, CONTENT, CONTENT, CONTENT, CONTENT
        );
        let re = Regex::new(&pattern)?;

        if let Some(caps) = re.captures(msg_str) {
            Ok(Self {
                time: parse_time(&caps[1])?,
                id: caps[4].to_string(),
                name: caps[5].to_string(),
                port: u16::from_str(&caps[6])?,
            })
        } else {
            Err(anyhow::anyhow!(
                "Invalid user connection message: {}",
                msg_str
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_user_connection_msg_from_str() {
        let log_entry = "2024/08/15 15:15:53 [I] [proxy.go:199] [9f9d61c865a4249c] [name-51048] get a user connection [94.156.64.251:46078]";
        let expected = UserConnectionMsg {
            time: parse_time("2024/08/15 15:15:53").unwrap(),
            id: "9f9d61c865a4249c".to_string(),
            name: "name-51048".to_string(),
            endpoint: Endpoint::from_str("94.156.64.251:46078").unwrap(),
        };

        let parsed = UserConnectionMsg::from_str(log_entry).unwrap();
        assert_eq!(parsed.time, expected.time);
        assert_eq!(parsed.id, expected.id);
        assert_eq!(parsed.name, expected.name);
        assert_eq!(parsed.endpoint, expected.endpoint);
    }

    #[test]
    fn test_user_connection_msg_from_str_invalid() {
        let log_entry = "invalid log entry";
        let result = UserConnectionMsg::from_str(log_entry);
        assert!(result.is_err());
    }

    #[test]
    fn test_proxy_exist_msg_from_str() {
        let log_entry = "2024/08/15 15:15:53 [W] [control.go:493] [c34d45ac838c9177] new proxy [name-51054] type [tcp] error: proxy [name-51054] already exists";
        let expected = ProxyExistMsg {
            time: parse_time("2024/08/15 15:15:53").unwrap(),
            id: "c34d45ac838c9177".to_string(),
            name: "name-51054".to_string(),
            protocol: "tcp".to_string(),
        };

        let parsed = ProxyExistMsg::from_str(log_entry).unwrap();
        assert_eq!(parsed.time, expected.time);
        assert_eq!(parsed.id, expected.id);
        assert_eq!(parsed.name, expected.name);
        assert_eq!(parsed.protocol, expected.protocol);
    }

    #[test]
    fn test_proxy_exist_msg_from_str_invalid() {
        let log_entry = "invalid log entry";
        let result = ProxyExistMsg::from_str(log_entry);
        assert!(result.is_err());
    }
}
