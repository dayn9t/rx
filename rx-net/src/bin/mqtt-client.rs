use rx_net::mqtt::MqttClient;

fn main() {
    let topic = "test";
    let payload = "hi111你好吗";

    let mut client = MqttClient::connect("test_id", "tcp://localhost:1883").unwrap();
    let rx = client.subscribe(topic).unwrap();
    client.publish(topic, payload).unwrap();

    let m = rx.iter().next().unwrap().unwrap();

    assert_eq!(m.topic(), topic);
    assert_eq!(m.payload(), payload.as_bytes());
}
