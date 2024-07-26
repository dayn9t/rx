use rumqttc::{Client, MqttOptions, QoS};
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

fn main() {
    //client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    thread::spawn(move || {
        f1();
    });

    // Iterate to poll the eventloop for connection progress
    /*
    for (i, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification);
    }*/
    thread::sleep(Duration::from_millis(10000));
}
