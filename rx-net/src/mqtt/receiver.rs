use std::path::Path;
use std::sync::mpsc::Sender;

use path_macro::path;
use rumqttc::{Client, Event, Incoming, QoS};
use serde::de::DeserializeOwned;

use rx_core::log::{error, info};
use rx_core::sys::fs::to_string;
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
        let topic = if let Some(ref root_topic) = cfg.root_topic {
            path!(root_topic / topic)
        } else {
            topic.as_ref().to_path_buf()
        };

        Self {
            topic: to_string(topic),
            cfg,
            sender,
        }
    }

    /// 从mqtt获取消息并发送到sender
    pub fn run(&self) {
        let opt = self.cfg.to_option();
        let (client, mut connection) = Client::new(opt, 10);
        client.subscribe(&self.topic, QoS::ExactlyOnce).unwrap();
        info!("Subscribed to mqtt: {:?}", self.topic);

        for (_i, notification) in connection.iter().enumerate() {
            match notification {
                Ok(Event::Incoming(incoming)) => match incoming {
                    Incoming::Publish(p) => {
                        let msg_str = std::str::from_utf8(&p.payload).unwrap();
                        info!("Received: {:?}", msg_str);
                        match json::from_str(msg_str) {
                            Ok(msg) => self.sender.send(msg).unwrap(),
                            Err(e) => {
                                error!("Load: {:?}  error: {:?}", msg_str, e);
                            }
                        }
                    }
                    _ => {}
                },
                Err(e) => {
                    error!("Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_works() {
        let mqtt_cfg = MqttCfg {
            server_url: "tcp://localhost:1883".to_string(),
            root_topic: None,
            keep_alive: 30,
        };

        let topic = "ias/shws/home";

        let (tx, _rx) = std::sync::mpsc::channel();

        let source = MqttReceiver::<i32>::new(mqtt_cfg, Path::new(topic), tx);

        source.run()
    }
}
