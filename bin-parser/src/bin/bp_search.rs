use rx_core::algo::{search_slice, slice_by_radius};
use std::fs::File;
use std::io::{stdin, Read};
//use std::num;

struct Searcher {
    contents: Vec<u8>,
    radius: usize,
    values: Vec<u8>,
    addrs: Vec<usize>,
}

impl Searcher {
    /// 创建搜索器
    pub fn new(path: &str, radius: usize) -> Searcher {
        let mut file = File::open(path).unwrap();

        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        println!("File size: {}", contents.len());

        return Searcher {
            contents,
            radius,
            values: Vec::new(),
            addrs: Vec::new(),
        };
    }

    /// 搜索一个数值
    pub fn search(&mut self, value: u8) {
        let addrs = if self.addrs.is_empty() {
            // 用第一个值确定候选地址集合
            search_slice(&self.contents[..], value)
        } else {
            // 通过检查是否存在后继值，筛选候选地址集合
            let mut addrs = Vec::new();
            for addr in &self.addrs {
                let (_, slice) = slice_by_radius(&self.contents[..], *addr, self.radius);
                let a = search_slice(slice, value);
                if !a.is_empty() {
                    addrs.push(*addr);
                }
            }
            addrs
        };
        if addrs.is_empty() {
            println!("no address found for value");
        } else {
            println!("{} addrs found.", addrs.len());
            self.addrs = addrs;
            self.values.push(value)
        }
    }

    pub fn show_addrs(&self) {
        for (i, addr1) in self.addrs.iter().enumerate() {
            println!("#{} {:#X}", i, addr1);
            let (begin, slice) = slice_by_radius(&self.contents[..], *addr1, self.radius);
            for value in &self.values {
                print!("\t{}:", value);
                let addrv = search_slice(slice, *value);
                let mut offsets: Vec<_> = addrv
                    .iter()
                    .map(|x| (begin + *x) as i64 - *addr1 as i64)
                    .collect();
                offsets.sort_unstable_by_key(|a| a.abs());
                for offset in offsets {
                    print!("\t{:+}", offset)
                }
                print!("\n")
            }
        }
    }
}

fn main() {
    //let path = "/home/jiang/game/tk5x-data/TW/Snr0_TW.TR5";
    //let path = "/home/jiang/game/w/t5/save_data/SaveDat1.TR5";
    // 0x6D8A: 5维
    // 0x6DAE:
    // 36字节
    print!("{:X}", 1400);
    let path = "/home/jiang/game/tk5x-data/SaveDat1.TR5";
    let mut searcher = Searcher::new(path, 128);

    loop {
        println!("Input a number:");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<usize>() {
            searcher.search(n as u8);
        } else {
            searcher.show_addrs()
        }
    }
}
