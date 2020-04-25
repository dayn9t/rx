use rx_web::node::*;

use http::uri::Builder;
use http::uri::Uri;
use http_req::request;
use serde_json::to_string_pretty as to_json;

fn main() {
    let url = "http://116.228.67.70:30080/";

    let url_tx = "http://shushan.zhangyue.net/book/83780/#directory"; //铁血残明
    let url_tx1 = "http://shushan.zhangyue.net/book/83780/13023184"; //铁血残明

    let url_gy = "https://www.i7wx.com/book/54/54350/";
    let url_gy1 = "https://www.i7wx.com/book/54/54350/15739329.html";

    let url_tx = "https://www.biquge.biz/28_28641/";
    let url_tx1 = "https://www.biquge.biz/28_28641/12089871.html";

    let url = url_gy;

    let root = Node::pull(url).unwrap();

    println!("Title: {}", root.find_title().unwrap());

    let text_node = root.find_max_text().unwrap();
    println!(
        "\ntext_node: {} {}",
        text_node.text_len(),
        to_json(&text_node).unwrap()
    );

    let parent_node = root.find_max_children().unwrap();
    println!("\nparent_node: {} ", to_json(&parent_node).unwrap());
}
