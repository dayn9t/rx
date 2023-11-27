use std::path::{Component, Path};

pub use network_interface::V4IfAddr;
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

use rx_core::fs;

// 根据网卡设备路径确定网卡类型
fn device_catalog(device_path: &Path) -> Option<String> {
    let link = device_path.read_link().unwrap();
    let cs = link.components();
    //println!("{:?}", &cs);
    //Components([ParentDir, ParentDir, Normal("devices"), Normal("virtual"), Normal("net"), Normal("lo")])
    let catalog = cs.skip(3).next().unwrap();
    if let Component::Normal(str) = catalog {
        str.to_str().map(|s| s.to_string())
    } else {
        None
    }
}

/// 获取网卡
pub fn get_adapters(catalog: Option<&str>) -> Vec<String> {
    let paths = fs::dirs_in(&"/sys/class/net").unwrap();
    let mut adapters = Vec::new();
    for path in paths {
        //println!("{:?}", dir);
        let catalog_part = device_catalog(&path);
        if let Some(c) = catalog_part {
            if catalog.is_none() || c.contains(catalog.unwrap()) {
                adapters.push(fs::file_name_owned(&path));
            }
        }
    }
    adapters
}

/// 获取PCI网卡
pub fn get_pci_adapters() -> Vec<String> {
    get_adapters(Some(&"pci"))
}

/// 获取网卡的IPv4信息
pub fn get_ip_addr_v4(adapter_name: &str) -> Option<V4IfAddr> {
    let interfaces = NetworkInterface::show().unwrap();

    for interface in interfaces.iter() {
        if interface.name == adapter_name {
            for addr in &interface.addr {
                match addr {
                    Addr::V4(v4) => {
                        return Some(*v4); // FIXME: 遗漏其他IP
                    }
                    Addr::V6(_) => {}
                }
            }
        }
    }
    None
}

/// 获取PCI网卡IPv4
pub fn get_pci_adapter_ip_addr_v4() -> Option<V4IfAddr> {
    let names = get_pci_adapters();
    if let Some(name) = names.first() {
        return get_ip_addr_v4(name);
    }
    None
}

#[cfg(test)]
mod tests {
    use rx_core::text::yaml;

    use super::*;

    #[test]
    fn adapters() {
        let a = get_adapters(Some(&"pci"));
        println!("pci: {:?}", a);
        let a = get_adapters(None);
        println!("all: {:?}", a);
    }
}
