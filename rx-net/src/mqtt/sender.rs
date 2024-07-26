use crate::mqtt::cfg::MqttCfg;

use rumqttc::{Client, MqttOptions, QoS};
use rx_core::log::{error, info};
use serde::Serialize;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

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

        //info!("Sender opt: {:?}", opt);
        //info!("Sender topic: {:?}", self.topic);
        let (client, mut connection) = Client::new(opt, 10);
        //client.subscribe(&self.topic, QoS::ExactlyOnce).unwrap();

        // 开启一个线程用于运行 connection 事件循环
        let thread = thread::spawn(move || {
            for notification in connection.iter() {
                match notification {
                    Ok(_) => {}
                    Err(e) => error!("Connection error: {:?}", e),
                }
            }
        });

        for msg in self.receiver.iter() {
            match serde_json::to_string(&msg) {
                Ok(payload) => {
                    //info!("send: {}", &payload);
                    client
                        .publish(&self.topic, QoS::AtLeastOnce, false, payload)
                        .unwrap();
                }
                Err(e) => {
                    error!("Failed to serialize message: {:?}", e);
                }
            }
        }
        thread.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_send() {
        let mqtt_cfg = MqttCfg {
            server_url: "tcp://localhost:1883".to_string(),
            root_topic: None,
            keep_alive: 30,
        };

        let topic = "ias/shws/home";
        let (tx, rx) = channel();
        let sender = MqttSender::new(mqtt_cfg, Path::new(topic), rx);

        tx.send("Hello, MQTT!").unwrap();
        sender.run();
    }
}
