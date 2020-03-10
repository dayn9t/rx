mod encoding;
mod html;

use http_req::request;


#[macro_use]
extern crate html5ever;

#[macro_use]
extern crate serde_derive;


fn main() {
    //let url = "http://116.228.67.70:30080/";

    //let url = "https://www.i7wx.com/book/54/54350/";
    let url = "https://www.i7wx.com/book/54/54350/15739329.html";
    let mut data = Vec::new(); //container for body of a response
    let res = request::get(url, &mut data).unwrap();

    //println!("res: {}", res.headers());
    //println!("Status: {} {}", res.status_code(), res.reason());

    let doc = html::Document::parse(&mut &data[..]).unwrap();

    let root = html::walk(&doc.dom.document);

    println!("\nDoc: {}", root.to_json());

    if !doc.dom.errors.is_empty() {
        //println!("\nParse errors:");
        for err in doc.dom.errors.iter() {
            //println!("    {}", err);
        }
    }
}
