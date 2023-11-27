use http::uri::Builder;
use http::uri::Uri;

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

    let url = _url_dw;
    let uri = url.parse::<Uri>().unwrap();

    let u = Builder::new()
        .authority(uri.authority().unwrap().as_str())
        .scheme(uri.scheme_str().unwrap())
        .path_and_query(uri.path_and_query().unwrap().as_str())
        .build()
        .unwrap();

    println!("uri: {:?}", uri);
    println!("uri: {:?}", u);
    println!(
        "authority_part: {:?}, {:?}",
        u,
        u.authority().unwrap().as_str()
    );
    println!("scheme_str: {:?}, {:?}", u, u.scheme_str().unwrap());
    println!(
        "path_and_query: {:?}, {:?}",
        u,
        u.path_and_query().unwrap().to_string()
    );
    println!("url: {}", u.to_string());
}
