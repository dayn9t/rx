use rx_core::text::yaml;
use rx_net::cfg::NetplanCfg;
use rx_net::ip::is_ipv4_str;

fn main() {
    let dhcp = NetplanCfg::with_dhcp(&"eth1");

    println!("dhcp: {}", yaml::to_string(&dhcp).unwrap());

    let one_ip = NetplanCfg::with_one_ip(&"eth2", &"10.1.0.7/24", "10.1.1.1");
    println!("one_ip: {}", yaml::to_string(&one_ip).unwrap());

    one_ip.save("/home/jiang/1/one_ip.yaml").unwrap();

    let ss = ["a", "localhost", "1.1.1.1", "10.1.1.1", "0.0.0.0"];
    for s in ss {
        println!("{} is_ipv4_str: {}", s, is_ipv4_str(s));
    }
}
