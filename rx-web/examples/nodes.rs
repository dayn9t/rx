use rx_web::node::*;
use serde_json::to_string_pretty as to_json;

fn main() {
    let url = "http://116.228.67.70:30080/";

    let url_tx = "http://shushan.zhangyue.net/book/83780/#directory"; //铁血残明
    let url_tx1 = "http://shushan.zhangyue.net/book/83780/13023184"; //铁血残明

    let url_gy = "https://www.i7wx.com/book/54/54350/";
    let url_gy1 = "https://www.i7wx.com/book/54/54350/15739329.html";

    let url_tx = "https://www.biquge.biz/28_28641/";
    let url_tx1 = "https://www.biquge.biz/28_28641/12089871.html";

    let url_dw = "https://www.qianqianxs.com/10/10361/";
    let url_dw1 = "https://www.qianqianxs.com/10/10361/12346671.html";

    let url = url_dw;

    let root = Node::pull(url).unwrap();

    println!("Title: {}", root.find_title().unwrap());

    let text_node = root.find_max_text();
    println!("\ntext: {}", to_json(&text_node).unwrap());

    let links = root.find_max_links();
    println!("\nlinks: {} ", to_json(&links).unwrap());
    //let parent_node = root.find_max_children().unwrap();
    //println!("\nparent_node: {} ", to_json(&parent_node).unwrap());
}
