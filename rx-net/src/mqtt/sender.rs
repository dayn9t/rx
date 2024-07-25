use crate::mqtt::cfg::MqttCfg;
use path_macro::path;
use rumqttc::{Client, MqttOptions, QoS};
use rx_core::log::{error, info};
use rx_core::sys::fs::to_string;
use serde::Serialize;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::thread::Thread;

/// 数据发送器
pub struct MqttSender<T> {
    topic: String,
    cfg: MqttCfg,
    receiver: Receiver<T>,
}

impl<T: Serialize> MqttSender<T> {
    pub fn new(cfg: MqttCfg, topic: impl AsRef<Path>, receiver: Receiver<T>) -> Self {
        Self {
            topic: cfg.full_topic(topic),
            cfg,
            receiver,
        }
    }

    /// 从receiver获取消息, 并发送到 mqtt
    pub fn run(&self) {
        let opt = self.cfg.to_option();
        let (client, mut connection) = Client::new(opt, 10);
        client.subscribe(&self.topic, QoS::ExactlyOnce).unwrap();
        info!("Subscribed to mqtt: {:?}", self.topic);

        for msg in self.receiver.iter() {
            match serde_json::to_string(&msg) {
                Ok(payload) => {
                    client
                        .publish(&self.topic, QoS::ExactlyOnce, false, payload.as_bytes())
                        .unwrap();
                }
                Err(e) => {
                    error!("Failed to serialize message: {:?}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        let mqtt_cfg = MqttCfg {
            server_url: "tcp://localhost:1883".to_string(),
            root_topic: None,
            keep_alive: 30,
        };

        let topic = "ias/shws/home";
        //let sender = MqttSender::new(mqtt_cfg, Path::new(topic));

        let message = "Hello, MQTT!";
        //sender.send(&message).unwrap();
    }
}
