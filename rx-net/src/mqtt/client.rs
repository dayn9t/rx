//use crate::basic::*;

use std::time::Duration;

/// MQTT消息&结果
pub use paho_mqtt::{Message, MqttResult};

/// MQTT消息接受者
pub type Receiver = std::sync::mpsc::Receiver<Option<Message>>;

/// MQTT客户
pub struct MqttClient {
    client: paho_mqtt::Client,
}

impl MqttClient {
    /// 新建对象
    pub fn connect(id: &str, server_uri: &str) -> MqttResult<MqttClient> {
        let opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(server_uri)
            .client_id(id)
            .finalize();
        let mut client = paho_mqtt::Client::new(opts)?;

        let d = Duration::from_secs(20);
        let opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(d)
            .clean_session(false)
            .automatic_reconnect(d, d) // 固定间隔重连
            .finalize();
        client.connect(opts)?;

        Ok(MqttClient { client })
    }

    /// 订阅主题
    pub fn subscribe(&mut self, topic: &str) -> MqttResult<Receiver> {
        let rx = self.client.start_consuming();
        self.client.subscribe(&topic, 1)?;
        Ok(rx)
    }

    /// 发布消息
    pub fn publish<V>(&mut self, topic: &str, payload: V) -> MqttResult<()>
    where
        V: Into<Vec<u8>>,
    {
        let msg = paho_mqtt::MessageBuilder::new()
            .topic(topic)
            .payload(payload)
            .qos(1)
            .finalize();
        self.client.publish(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let topic = "test";
        let payload = "hi111你好吗";

        let mut client = MqttClient::connect("test_id", "tcp://localhost:1883").unwrap();
        let mut rx = client.subscribe(topic).unwrap();
        client.publish(topic, payload).unwrap();

        let m = rx.iter().next().unwrap().unwrap();

        assert_eq!(m.topic(), topic);
        assert_eq!(m.payload(), payload.as_bytes());
    }
}
