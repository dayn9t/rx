use rx_core::log::init_log;
use rx_net::mqtt::*;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    init_log(2);

    let mqtt_cfg = MqttCfg {
        server_url: "tcp://localhost:1883".to_string(),
        root_topic: None,
        keep_alive: 30,
        user: None,
        password: None,
    };

    println!("mqtt_cfg: {:?}", mqtt_cfg);

    let topic = "ias/shws/home";

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let sender = MqttSender::<String>::new(mqtt_cfg.clone(), topic, rx1);
    let receiver = MqttReceiver::<String>::new(mqtt_cfg, topic, tx2);

    let thread1 = thread::spawn(move || {
        sender.run();
    });

    let thread2 = thread::spawn(move || {
        receiver.run();
    });

    for i in 0..10 {
        let i = i.to_string();
        println!("Send: {}", i);
        tx1.send(i).unwrap();
    }

    for i in 0..10 {
        let i = i.to_string();
        let i1 = rx2.recv().unwrap();
        println!("Receive: {}", i);
        assert_eq!(i, i1);
    }

    thread1.join().unwrap();
    thread2.join().unwrap();
}
