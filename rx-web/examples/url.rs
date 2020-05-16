use http::uri::Builder;
use http::uri::Uri;
use http_req::request;

fn main() {
    let url = "http://116.228.67.70:30080/";

    let url_tx = "http://shushan.zhangyue.net/book/83780/#directory"; //铁血残明
    let url_tx1 = "http://shushan.zhangyue.net/book/83780/13023184"; //铁血残明

    let url_gy = "https://www.i7wx.com/book/54/54350/";
    let url_gy1 = "https://www.i7wx.com/book/54/54350/15739329.html";

    let url_tx = "https://www.biquge.biz/28_28641/";
    let url_tx1 = "https://www.biquge.biz/28_28641/12089871.html";

    let url = url_gy;
    let uri = url.parse::<Uri>().unwrap();

    let u = Builder::new()
        .authority(uri.authority_part().unwrap().as_str())
        .scheme(uri.scheme_str().unwrap())
        .path_and_query(uri.path_and_query().unwrap().as_str())
        .build()
        .unwrap();

    println!("uri: {:?}", uri);
    println!("uri: {:?}", u);
    println!(
        "authority_part: {:?}, {:?}",
        u,
        u.authority_part().unwrap().as_str()
    );
    println!("scheme_str: {:?}, {:?}", u, u.scheme_str().unwrap());
    println!(
        "path_and_query: {:?}, {:?}",
        u,
        u.path_and_query().unwrap().to_string()
    );
    println!("url: {}", u.to_string());
}
