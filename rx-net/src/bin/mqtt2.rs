use rumqttc::*;
use rx_core::log::{info, init_log};
use rx_net::mqtt::{MqttCfg, MqttSender};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn f1() {
    let mut opt = MqttOptions::new("rumqtt-sync1", "localhost", 1883);
    opt.set_keep_alive(Duration::from_secs(5));

    println!("opt: {:?}", opt);

    let (mut client, connection) = Client::new(opt, 10);

    for i in 0..10 {
        let msg = format!("msg1{}", i);
        println!("send: {}", msg);
        client
            .publish("hello/rumqtt", QoS::AtLeastOnce, false, msg)
            .unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn f2() {
    let mut opt = MqttOptions::new("rumqtt-sync1", "localhost", 1883);
    opt.set_keep_alive(Duration::from_secs(5));

    info!("Sender opt: {:?}", opt);
    //info!("Sender topic: {:?}", self.topic);
    let (client, _) = Client::new(opt, 10);
    //let (client, conn) = Client::new(opt, 10);
    //client.subscribe(&self.topic, QoS::ExactlyOnce).unwrap();

    for i in 0..10 {
        let payload = "hihi";
        //client.publish(&self.topic, QoS::AtLeastOnce, false, payload).unwrap();

        client
            .publish("hello/rumqtt", QoS::AtLeastOnce, false, payload)
            .unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    init_log(2);

    //f2();

    let mqtt_cfg = MqttCfg {
        server_url: "tcp://localhost:1883".to_string(),
        root_topic: None,
        keep_alive: 30,
    };

    let topic = "ias/shws/home";
    let (tx, rx) = channel();
    let sender = MqttSender::new(mqtt_cfg, topic, rx);

    tx.send("Hello, MQTT!").unwrap();
    sender.run();
}
