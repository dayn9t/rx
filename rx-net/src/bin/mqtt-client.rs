use std::{env, process, thread, time::Duration};

use paho_mqtt as mqtt;

use rx_net::mqtt::MqttClient;

fn _test_receive() {
    let topic = "test";
    let payload = "aaa";

    let mut client = MqttClient::connect("test_id", "tcp://localhost:1883").unwrap();
    let rx = client.subscribe(topic).unwrap();
    client.publish_as_bytes(topic, payload).unwrap();

    for m in rx.iter() {
        let m = m.unwrap();
        println!("topic: {:?}", m.topic());
        println!("payload: {:?}", m.payload());
    }

    //println!("message: {:?}", m);
    //assert_eq!(m.topic(), topic);
    //assert_eq!(m.payload(), payload.as_bytes());
}

fn test_send() {
    // Initialize the logger from the environment
    env_logger::init();

    // Create a client & define connect options
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    let mut cli = mqtt::Client::new(host).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // Use 5sec timeouts for sync calls.
    cli.set_timeout(Duration::from_secs(5));

    // Connect and wait for it to complete or fail.
    // The default connection uses MQTT v3.x
    if let Err(e) = cli.connect(None) {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    let payload = "Hello synchronous world!!!!!!!!!!!";
    // Create a message and publish it
    let msg = mqtt::MessageBuilder::new()
        .topic("test")
        .payload(payload)
        .qos(1)
        .finalize();

    if let Err(e) = cli.publish(msg) {
        println!("Error sending message: {:?}", e);
    } else {
        println!("Send: {}", payload);
    }

    // Disconnect from the broker
    cli.disconnect(None).unwrap();
}

fn main() {
    thread::spawn(test_send);
    //thread::sleep(Duration::from_secs(10));
}
