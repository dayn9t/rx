use std::collections::HashMap;

use rx_core::text::*;

// https://netplan.io/reference

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct NameServers {
    pub addresses: Vec<String>,
}

impl Default for NameServers {
    fn default() -> Self {
        NameServers {
            addresses: vec!["114.114.114.114".to_string(), "8.8.8.8".to_string()],
        }
    }
}

impl NameServers {
    pub fn new(addresses: Vec<String>) -> Self {
        Self { addresses }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Ethernet {
    pub dhcp4: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<NameServers>,
}

impl Ethernet {
    /// 单一IP的以太网卡配置
    pub fn with_one_ip(ip_addr: &str, gateway: &str) -> Self {
        Ethernet {
            dhcp4: false,
            addresses: Some(vec![ip_addr.to_string()]),
            gateway4: Some(gateway.to_string()),
            nameservers: Some(NameServers::default()),
        }
    }

    /// 使用DHCP的以太网卡配置
    pub fn with_dhcp() -> Self {
        Ethernet {
            dhcp4: true,
            addresses: None,
            gateway4: None,
            nameservers: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Network {
    pub version: i32,
    pub renderer: String,
    pub ethernets: HashMap<String, Ethernet>,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            version: 2,
            renderer: "NetworkManager".to_string(),
            ethernets: Default::default(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NetplanCfg {
    pub network: Network,
}

impl NetplanCfg {
    /// 单一IP的网络配置
    pub fn with_one_ip(eth_name: &str, ip_addr: &str, gateway: &str) -> Self {
        let mut cfg = Self::default();
        let eth = Ethernet::with_one_ip(ip_addr, gateway);
        cfg.network.ethernets.insert(eth_name.to_string(), eth);
        cfg
    }

    /// 使用DHCP的网络配置
    pub fn with_dhcp(eth_name: &str) -> Self {
        let mut cfg = Self::default();
        let eth = Ethernet::with_dhcp();
        cfg.network.ethernets.insert(eth_name.to_string(), eth);
        cfg
    }

    /// 使用Ethernet信息的网络配置
    pub fn with_ethernet(eth_name: &str, eth: Ethernet) -> Self {
        let mut cfg = Self::default();
        cfg.network.ethernets.insert(eth_name.to_string(), eth);
        cfg
    }

    /// 保存到配置文件
    pub fn load(path: impl AsRef<Path>) -> BoxResult<Self> {
        yaml::load(path)
    }

    /// 保存到配置文件
    pub fn save(&self, path: impl AsRef<Path>) -> BoxResult<()> {
        yaml::save(self, path)
    }

    /// 获取指定网卡
    pub fn get_ethernet(&self, ethernet_name: &str) -> Option<&Ethernet> {
        self.network.ethernets.get(ethernet_name)
    }

    /// 更新以太网
    fn _update_ethernets(&mut self, names: Vec<String>) -> BoxResult<()> {
        if names.len() == self.network.ethernets.len() {}
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let p = "/home/jiang/rs/rx/rx-net/data/2eth.yaml";

        let cfg: NetplanCfg = yaml::load(p).unwrap();

        let dst = "/home/jiang/1/2eth.yaml";
        yaml::save(&cfg, dst).unwrap();

        //assert_eq!(m.topic(), topic);
    }
}
