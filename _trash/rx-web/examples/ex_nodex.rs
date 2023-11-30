use rx_core::text::json;
use rx_web::node::*;
use rx_web::req::RequestCfg;

fn main() {
    let _url = "http://116.228.67.70:30080/";

    let _url_tx = "http://shushan.zhangyue.net/book/83780/#directory"; //铁血残明
    let _url_tx1 = "http://shushan.zhangyue.net/book/83780/13023184"; //铁血残明

    let _url_gy = "https://www.i7wx.com/book/54/54350/";
    let _url_gy1 = "https://www.i7wx.com/book/54/54350/15739329.html";

    let _url_tx = "https://www.biquge.biz/28_28641/";
    let _url_tx1 = "https://www.biquge.biz/28_28641/12089871.html";

    let _url_dw = "https://www.qianqianxs.com/10/10361/";
    let _url_dw1 = "https://www.qianqianxs.com/10/10361/12346671.html";

    let _url_xh = "https://www.ranwena.com/files/article/88/88687/";
    let _url_xh1 = "https://www.ranwena.com/files/article/88/88687/21948376.html";

    let url = _url_xh1;
    let mut cfg = RequestCfg::default();
    cfg.headers.insert(
        "User-Agent".to_string(),
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; SV1)".to_string(),
    );

    let root = Node::pull(url, &cfg).unwrap();
    println!("Title: {}", root.find_title().unwrap());

    let text_node = root.find_max_text();
    println!("\ntext: {}", json::to_pretty(&text_node).unwrap());

    let links = root.find_max_links();
    println!("\nlinks: {} ", json::to_pretty(&links).unwrap());
    //let parent_node = root.find_max_children().unwrap();
    //println!("\nparent_node: {} ", json::to_pretty(&parent_node).unwrap());
}
