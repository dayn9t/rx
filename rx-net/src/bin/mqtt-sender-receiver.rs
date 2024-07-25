use rx_net::mqtt::*;
use std::path::Path;

use rx_core::log::init_log;
use std::thread;

fn main() {
    init_log(2);

    let mqtt_cfg = MqttCfg {
        server_url: "tcp://localhost:1883".to_string(),
        root_topic: None,
        keep_alive: 30,
    };

    println!("mqtt_cfg: {:?}", mqtt_cfg);

    let topic = "ias/shws/home";

    let (tx1, rx1) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();

    let sender = MqttSender::<i32>::new(mqtt_cfg.clone(), Path::new(topic), rx1);
    let receiver = MqttReceiver::<i32>::new(mqtt_cfg, Path::new(topic), tx2);

    let thread1 = thread::spawn(move || {
        sender.run();
    });

    let thread2 = thread::spawn(move || {
        receiver.run();
    });

    for i in 0..10 {
        tx1.send(i).unwrap();
        println!("Send: {}", i);
    }

    for i in 0..10 {
        let i1 = rx2.recv().unwrap();
        assert_eq!(i, i1);
        println!("Receive: {}", i);
    }

    thread1.join().unwrap();
    thread2.join().unwrap();
}
