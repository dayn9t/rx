mod encoding;

use encoding::gbk_to_utf8;

use http_req::request;


#[macro_use]
extern crate html5ever;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::LocalName;


// This is not proper HTML serialization, of course.

fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    // FIXME: don't allocate
    //    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document => println!("#Document"),

        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        NodeData::Text { ref contents } => {
            println!("#text: -{}-", escape_default(&contents.borrow()))
        }

        NodeData::Comment { ref contents } => println!("<!-- {} -->", escape_default(contents)),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("##<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 4, child);
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn find_attr(
    handle: &Handle,
    elem_name: &LocalName,
    attr_name: &LocalName,
) -> Option<String> {
    let node = handle;

    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            println!("name.local: {}", name.local);
            if name.local == *elem_name {
                for attr in attrs.borrow().iter() {
                    if attr.name.local == *attr_name {
                        return Some(attr.value.to_string());
                    }
                }
            }
        }
        _ => {}
    }


    for child in node.children.borrow().iter() {
        let value = find_attr(child, elem_name, attr_name);
        if value.is_some() {
            return value;
        }
    }

    None
}


fn find_charset(handle: &Handle) -> Option<String> {
    //let node = handle;
    let content = find_attr(handle, &LocalName::from("meta"), &LocalName::from("content"))?;

    let re = regex::Regex::new(r"charset=(\w+)").unwrap();
    let cap = re.captures(&content)?;
    Some(cap.get(1).unwrap().as_str().to_uppercase())
}


pub fn parse(data: &mut &[u8]) -> std::io::Result<RcDom> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(data)?;

    if let Some(charset) = find_charset(&dom.document) {
        println!("charset: {}", charset);
        if charset == "GBK" {
            let data = gbk_to_utf8(data).unwrap();
            let dom = parse_document(RcDom::default(), Default::default())
                .from_utf8()
                .read_from(&mut &data[..])?;
            return Ok(dom);
        }
    }
    Ok(dom)
}


fn main() {
    //let url = "http://116.228.67.70:30080/";
    let url = "https://www.i7wx.com/book/54/54350/";
    let mut data = Vec::new(); //container for body of a response
    let res = request::get(url, &mut data).unwrap();

    println!("res: {}", res.headers());

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Len: {}", data.len());


    let dom = parse(&mut &data[..]).unwrap();

    walk(0, &dom.document);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }
}
