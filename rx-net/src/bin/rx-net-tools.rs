use std::io;
use std::io::Write;

use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use rx_net::cfg::{get_adapters, get_ip_addr_v4, get_pci_adapter_ip_addr_v4, get_pci_adapters};

fn f1() {
    let sys = System::new_all();

    //sys.refresh_all();
    //sys.refresh_networks();

    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    for (interface_name, data) in sys.networks().iter() {
        writeln!(
            &mut io::stdout(),
            "{}:\n  input data  (new / total): {} / {} B\n  output data (new / total): {} / {} B",
            interface_name,
            data.received(),
            data.total_received(),
            data.transmitted(),
            data.total_transmitted(),
        );
    }
}

fn f2() {
    let network_interfaces = NetworkInterface::show().unwrap();

    for itf in network_interfaces.iter() {
        println!("{:?}", itf);
    }
}

fn main() {
    let names = get_pci_adapters();
    println!("pci: {:?}", names);

    let ip = get_ip_addr_v4(&names[0]);
    println!("ip: {:?}", ip);

    let ip = get_pci_adapter_ip_addr_v4().unwrap();
    println!("ip: {:?}", ip.ip.to_string());

    let names = get_adapters(None);
    println!("all: {:?}", names);

    f2()
}
