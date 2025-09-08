use path_macro::path;
use rumqttc::MqttOptions;
use std::path::Path;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

use rx_core::prelude::*;
use rx_core::sys::fs::to_string;

/// Mqtt配置信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MqttCfg {
    /// MQTT服务器地址
    pub server_url: String,
    /// MQTT根主题
    pub root_topic: Option<String>,
    /// 心跳间隔(秒)
    pub keep_alive: usize,
    /// 用户名
    pub user: Option<String>,
    /// 密码
    pub password: Option<String>,
}

impl MqttCfg {
    /// 生成MqttOptions
    pub fn to_option(&self) -> MqttOptions {
        let id = Uuid::new_v4().to_string();
        let uri = Url::parse(&self.server_url).unwrap();
        let mut opt = MqttOptions::new(id, uri.host_str().unwrap(), uri.port().unwrap());
        opt.set_keep_alive(Duration::from_secs(self.keep_alive as u64));

        // 设置用户名和密码（如果提供的话）
        if let (Some(user), Some(password)) = (&self.user, &self.password) {
            opt.set_credentials(user, password);
        }

        opt
    }

    /// 获取完整主题
    pub fn full_topic(&self, topic: impl AsRef<Path>) -> String {
        let topic = if let Some(ref root_topic) = self.root_topic {
            path!(root_topic / topic)
        } else {
            topic.as_ref().to_path_buf()
        };
        to_string(topic)
    }
}
