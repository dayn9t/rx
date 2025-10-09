use rx_core::log::init_log;
use rx_net::mqtt::*;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    init_log(2);
    /*
        let mqtt_cfg = MqttCfg {
        server_url: "tcp://localhost:1883".to_string(),
        root_topic: None,
        keep_alive: 30,
        user: None,
        password: None,
    };
    */

    let mqtt_cfg = MqttCfg {
        server_url: "tcp://192.168.18.147:1883".to_string(),
        root_topic: None,
        keep_alive: 30,
        user: Some("admin".to_string()),
        password: Some("Howell.net.cn".to_string()),
    };
    println!("mqtt_cfg: {:?}", mqtt_cfg);

    let topic = "ias/shws/home";

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let sender = MqttSender::<String>::new(mqtt_cfg.clone(), topic, rx1);
    let receiver = MqttReceiver::<String>::new(mqtt_cfg, topic, tx2);

    let thread2 = thread::spawn(move || {
        receiver.run();
    });

    let thread1 = thread::spawn(move || {
        sender.run();
    });

    let n = 10;

    for i in 0..n {
        let i = i.to_string();
        println!("Send: {}", i);
        tx1.send(i).unwrap();
        //sleep(Duration::from_millis(200));
    }

    for i in 0..n {
        let _i = i.to_string();
        let i1 = rx2.recv().unwrap();
        println!("Receive: {}", i1);
        //assert_eq!(i, i1);
    }

    println!("1");
    thread1.join().unwrap();
    println!("2");
    thread2.join().unwrap();
    println!("3");
}
