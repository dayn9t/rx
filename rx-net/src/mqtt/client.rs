//use crate::basic::*;

use std::path::Path;
use std::time::Duration;

/// MQTT消息&结果
pub use paho_mqtt::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;

use rx_core::sys::fs::to_string;
use rx_core::text::json::to_pretty;
use rx_core::text::{json, AnyResult};

pub type MqttResult<T> = paho_mqtt::Result<T>;

/// MQTT消息接受者
pub type Receiver = paho_mqtt::Receiver<Option<Message>>;

/// MQTT客户
pub struct MqttClient {
    client: paho_mqtt::Client,
}

impl MqttClient {
    /// 新建对象
    pub fn connect(id: impl Into<String>, server_uri: impl Into<String>) -> MqttResult<MqttClient> {
        let id = id.into();
        let clean_session = id.is_empty();
        let opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(server_uri)
            .client_id(id)
            .finalize();
        let client = paho_mqtt::Client::new(opts)?;

        let d = Duration::from_secs(20);
        let opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(d)
            .clean_session(clean_session)
            .automatic_reconnect(d, d) // 固定间隔重连
            .finalize();
        client.connect(opts)?;

        Ok(MqttClient { client })
    }

    /// 订阅主题
    pub fn subscribe(&mut self, topic: impl AsRef<Path>) -> MqttResult<Receiver> {
        let topic = to_string(topic.as_ref());
        let rx = self.client.start_consuming();
        self.client.subscribe(&topic, 1)?;
        Ok(rx)
    }

    /// 发布消息 - 转换成 Vec<u8
    pub fn publish_as_bytes<V>(&mut self, topic: &str, payload: V) -> AnyResult<()>
    where
        V: Into<Vec<u8>>,
    {
        let msg = paho_mqtt::MessageBuilder::new()
            .topic(topic)
            .payload(payload)
            .qos(1)
            .finalize();
        self.client.publish(msg)?;
        Ok(())
    }

    /// 发布消息 - 序列换成JSON
    pub fn publish_as_json(&mut self, topic: &str, ob: &impl Serialize) -> AnyResult<()> {
        let s = to_pretty(ob)?;
        self.publish_as_bytes(topic, s)
    }
}

/// 解析消息成对象
pub fn parse_json<T: DeserializeOwned>(msg: &Message) -> Result<T, serde_json::Error> {
    let s = String::from_utf8(msg.payload().to_vec()).unwrap();
    json::from_str(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let topic = "test";
        let payload = "hi111你好吗";

        let mut client = MqttClient::connect("test_id", "tcp://localhost:1883").unwrap();
        let rx = client.subscribe(topic).unwrap();
        client.publish_as_bytes(topic, payload).unwrap();

        let m = rx.iter().next().unwrap().unwrap();

        assert_eq!(m.topic(), topic);
        assert_eq!(m.payload(), payload.as_bytes());
    }
}
