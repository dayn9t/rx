use std::time::Duration;

use rumqttc::MqttOptions;
use url::Url;
use uuid::Uuid;

use rx_core::serde_export::*;

/// Mqtt配置信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MqttCfg {
    /// MQTT服务器地址
    pub server_url: String,
    /// MQTT根主题
    pub root_topic: Option<String>,
    /// 心跳间隔(秒)
    pub keep_alive: usize,
}

impl MqttCfg {
    /// 生成MqttOptions
    pub fn to_option(&self) -> MqttOptions {
        let id = Uuid::new_v4().to_string();
        let uri = Url::parse(&self.server_url).unwrap();
        let mut opt = MqttOptions::new(id, uri.host_str().unwrap(), uri.port().unwrap());
        opt.set_keep_alive(Duration::from_secs(self.keep_alive as u64));
        opt
    }
}
