mod encoding;

use http_req::request;

#[macro_use]
extern crate html5ever;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

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
            println!("#text: {}", escape_default(&contents.borrow()))
        }

        NodeData::Comment { ref contents } => println!("<!-- {} -->", escape_default(contents)),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
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

fn main() {
    //let url = "http://116.228.67.70:30080/";
    let url = "https://www.i7wx.com/book/54/54350/";
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get(url, &mut writer).unwrap();

    println!("res: {}", res.headers());

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Len: {}", writer.len());

    if let Some(s) = encoding::to_utf8(&writer) {
        println!("result: ##{}##", s);
    }

    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();
    walk(0, &dom.document);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }
}
