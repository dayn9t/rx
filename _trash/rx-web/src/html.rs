pub use std::io::Result;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

use rx_core::text;

/// HTML文档
pub struct Document {
    pub dom: RcDom,
}

impl Document {
    /// 解析数据产生文档对象
    pub fn parse(data: &[u8]) -> Result<Document> {
        let dom = if text::find_gbk(data) {
            let gbk_data = text::gbk_to_utf8(data).unwrap();
            parse_document(RcDom::default(), Default::default())
                .from_utf8()
                .read_from(&mut &gbk_data[..])?
        } else {
            parse_document(RcDom::default(), Default::default())
                .from_utf8()
                .read_from(&mut data.clone())?
        };
        Ok(Document { dom })
    }

    /// 获取字符集
    pub fn get_charset(&self) -> Option<String> {
        find_charset(&self.dom.document)
    }
}

// 遍历属性
fn visit_attrs<F>(node: &Handle, path: &[&str], fun: &mut F) -> bool
where
    F: FnMut(&str, &str) -> bool,
{
    if path.is_empty() {
        return false;
    }
    let node_name = path.first().unwrap();
    println!("visit_attrs nodes: {:?}", path);
    println!("visit_attrs node: {}", node_name);

    match node.data {
        NodeData::Document if *node_name == "" => {}
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            println!("name.local: {}", name.local);
            if node_name != &name.local.to_string() {
                return false;
            }
            if path.len() == 1 {
                for attr in attrs.borrow().iter() {
                    let finished = fun(&attr.name.local.to_string(), &attr.value.to_string());
                    if finished {
                        return true;
                    }
                }
                return false;
            }
        }
        _ => {
            return false;
        }
    }

    for child in node.children.borrow().iter() {
        let finished = visit_attrs(child, &path[1..], fun);
        if finished {
            return true;
        }
    }
    false
}

// 获取字符集
fn find_charset(handle: &Handle) -> Option<String> {
    //let node = handle;
    let path = ["", "html", "head", "meta"];
    let re = regex::Regex::new(r"charset=(\w+)").unwrap();
    let mut charset = None;
    visit_attrs(handle, &path[..], &mut |name: &str, value: &str| {
        println!("find_charset: {}  {}", name, value);
        match name {
            "charset" => {
                charset = Some(value.to_string());
                true
            }
            "content" => {
                if let Some(cap) = re.captures(&value) {
                    charset = Some(cap.get(1).unwrap().as_str().to_uppercase());
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    });
    charset
}
