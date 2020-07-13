use rx_web::req;

use curl::easy::Easy;
use std::collections::HashMap;
use std::io::{stdout, Write};

use serde_json::to_string_pretty;

pub fn http_get(url: &str, writer: &mut Vec<u8>) {
    let mut cfg = req::RequestCfg::default();
    cfg.headers.insert(
        "User-Agent".to_string(),
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.2; SV1)".to_string(),
    );

    let s = to_string_pretty(&cfg).unwrap();
    println!("Headers: {}", s);

    let resp = req::get(url, writer, &cfg).unwrap();

    println!("Status: {} {}", resp.status_code(), resp.reason());
}

fn curl_test(url: &str, dst: &mut Vec<u8>) {
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}

fn main() {
    let url = "http://116.228.67.70:30080/";

    //let url_tx = "http://shushan.zhangyue.net/book/83780/#directory"; //铁血残明
    //let url_tx1 = "http://shushan.zhangyue.net/book/83780/13023184"; //铁血残明

    let url_gy = "https://www.i7wx.com/book/54/54350/";
    let url_gy1 = "https://www.i7wx.com/book/54/54350/15739329.html";

    let url_tx = "https://www.biquge.biz/28_28641/";
    let url_tx1 = "https://www.biquge.biz/28_28641/12089871.html";

    let url_dw = "https://www.qianqianxs.com/10/10361/";
    let url_dw1 = "https://www.qianqianxs.com/10/10361/12346671.html";

    for url in &[url_gy, url_tx, url_dw] {
        let mut data = Vec::new();
        //http_req_test(url);
        //http_req_test1(url, &mut data);
        //curl_test(url, &mut data);
        http_get(url, &mut data);
        println!("Len: {}", data.len());
    }
}
