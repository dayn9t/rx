use std::path::Path;
use std::sync::mpsc::Sender;

use rumqttc::{Client, Event, Incoming, QoS};
use serde::de::DeserializeOwned;

use rx_core::log::error;
use rx_core::text::json;

use crate::mqtt::cfg::MqttCfg;

/// 数据源组
pub struct MqttReceiver<T> {
    topic: String,
    cfg: MqttCfg,
    sender: Sender<T>,
}

impl<T: DeserializeOwned> MqttReceiver<T> {
    pub fn new(cfg: MqttCfg, topic: impl AsRef<Path>, sender: Sender<T>) -> Self {
        Self {
            topic: cfg.full_topic(topic),
            cfg,
            sender,
        }
    }

    /// 从mqtt获取消息并发送到sender
    pub fn run(&self) {
        let opt = self.cfg.to_option();
        //info!("Receiver opt: {:?}", opt);
        //info!("Receiver topic: {:?}", self.topic);
        let (client, mut connection) = Client::new(opt, 10);
        client.subscribe(&self.topic, QoS::ExactlyOnce).unwrap();

        for notification in connection.iter() {
            match notification {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    let msg_str = std::str::from_utf8(&p.payload).unwrap();
                    match json::from_str(msg_str) {
                        Ok(msg) => self.sender.send(msg).unwrap(),
                        Err(e) => {
                            error!("Load: {:?}  error: {:?}", msg_str, e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {}
