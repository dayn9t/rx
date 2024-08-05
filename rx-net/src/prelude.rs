use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use url::Url;

use rx_core::serde_export::{Deserialize, Serialize};

/// Endpoint信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// 主机
    pub host: String,
    /// 端口
    pub port: u16,
}

/// 授权信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    /// 用户
    pub user: String,
    /// 密码
    pub password: String,
}

/// URL信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UrlInfo {
    /// 端点
    pub endpoint: Endpoint,
    /// 授权信息
    pub auth_info: AuthInfo,
    /// 路径
    pub path: PathBuf,
    /// 查询参数
    pub queries: HashMap<String, String>,
}

impl FromStr for UrlInfo {
    type Err = url::ParseError;

    fn from_str(url_str: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(url_str)?;

        let endpoint = Endpoint {
            host: url.host_str().unwrap_or("").to_string(),
            port: url.port().unwrap_or(0),
        };

        let auth_info = AuthInfo {
            user: url.username().to_string(),
            password: url.password().unwrap_or("").to_string(),
        };

        let path = PathBuf::from(url.path());

        let mut queries = HashMap::new();
        for (key, value) in url.query_pairs() {
            queries.insert(key.to_string(), value.to_string());
        }

        Ok(UrlInfo {
            endpoint,
            auth_info,
            path,
            queries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let url = "hikn://admin:howell1409@10.1.0.21:8000/record/101?start_time=2024-07-29T12:00:00&end_time=2024-07-29T12:10:00";

        let url_info: UrlInfo = url.parse().unwrap();

        assert_eq!(url_info.endpoint.host, "10.1.0.21");
        assert_eq!(url_info.endpoint.port, 8000);
        assert_eq!(url_info.auth_info.user, "admin");
        assert_eq!(url_info.auth_info.password, "howell1409");
        assert_eq!(url_info.path, PathBuf::from("/record/101"));
        assert_eq!(
            url_info.queries.get("start_time").unwrap(),
            "2024-07-29T12:00:00"
        );
        assert_eq!(
            url_info.queries.get("end_time").unwrap(),
            "2024-07-29T12:10:00"
        );
    }
}
