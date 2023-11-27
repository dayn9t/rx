use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// CSV文件加载成二维数组
pub fn load_csv_to_2d(path: &Path) -> Vec<Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut arr2d = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split('\t').map(|x| x.to_string()).collect();
        arr2d.push(fields);
    }
    arr2d
}

/// CSV文件加载成二维数组
pub fn load_csv_to_maps(path: &Path) -> Vec<HashMap<String, String>> {
    let mut src = load_csv_to_2d(path);
    let mut dst = Vec::new();

    let titles = src.remove(0);
    for fields in src.iter() {
        let mut m = HashMap::new();
        for (i, f) in fields.iter().enumerate() {
            m.insert(titles[i].clone(), f.to_string());
        }
        dst.push(m);
    }
    dst
}

#[test]
fn io() {
    let path = "/home/jiang/game/tk5x/client/data/scenarios/1/char_names.tsv";
    let path = Path::new(path);
    let arr2d = load_csv_to_2d(path);
    for _a in arr2d {
        //println!("{:?}", a);
    }

    let map = load_csv_to_maps(path);
    for a in map {
        println!("{:?}", a);
    }
    //assert_eq!(s1, "k");
}
