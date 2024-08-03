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
