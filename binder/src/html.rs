use crate::encoding::gbk_to_utf8;
use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::LocalName;
use http_req::request;

pub use serde_json::to_string_pretty as to_json;

use std::borrow::Borrow;
use std::collections::HashMap;

pub use std::io::Result;

/// HTML文档
pub struct Document {
    pub dom: RcDom,
}

impl Document {
    /// 解析数据产生文档对象
    pub fn parse(data: &[u8]) -> Result<Document> {
        let mut dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut data.clone())?;

        if let Some(charset) = find_charset(&dom.document) {
            if charset == "GBK" {
                let gbk_data = gbk_to_utf8(data).unwrap();
                dom = parse_document(RcDom::default(), Default::default())
                    .from_utf8()
                    .read_from(&mut &gbk_data[..])?;
            }
        }
        Ok(Document { dom })
    }

    /// 获取字符集
    fn get_charset(&self) -> Option<String> {
        find_charset(&self.dom.document)
    }
}

#[derive(Default, Serialize)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub text: Vec<String>,
    pub children: Vec<Box<Node>>,
}

impl Node {
    /// 获取h1
    fn find_h1(&self) -> Vec<Node> {
        let mut nodes = Vec::new();

        nodes
    }

    /// 遍历节点
    pub fn walk<F>(&self, fun: F) -> bool
        where
            F: FnMut(&Node) -> bool,
    {
        if !fun(self) { return false; }
        for child in &self.children {
            if !child.walk(fun) {
                return false;
            }
        }
        return true;
    }


    pub fn to_json(&self) -> String {
        to_json(self).unwrap()
    }
}

pub struct NodeIterator
{
    parents : Option<Box<NodeIterator>>,
}


impl Iterator for NodeIterator {
    type Item = Node;

    fn next (&mut self) -> Option<Self::Item> {

        Some((curr_fahr, curr_celc))
    }
}



/// 遍历节点
pub fn walk(handle: &Handle) -> Node {
    let mut node = Node::default();

    match handle.data {
        NodeData::Document => {
            node.name = "Document".to_string();
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            node.name = name.local.to_string();

            for attr in attrs.borrow().iter() {
                //print!(" {}=\"{}\"", attr.name.local, attr.value);
                node.attrs
                    .insert(attr.name.local.to_string(), attr.value.to_string());
            }
        }
        _ => unreachable!(),
    }

    for child in handle.children.borrow().iter() {
        let mut is_node = false;
        match child.data {
            NodeData::Document => is_node = true,
            NodeData::Element { .. } => is_node = true,
            NodeData::Text { ref contents } => {
                node.text.push(contents.borrow().to_string());
            }
            _ => {}
        }
        if is_node {
            let child_node = walk(child);
            node.children.push(Box::new(child_node));
        }
    }
    node
}

// 获取属性
fn find_attr(handle: &Handle, elem_name: &LocalName, attr_name: &LocalName) -> Option<String> {
    let node = handle;

    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            //println!("name.local: {}", name.local);
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

// 获取字符集
fn find_charset(handle: &Handle) -> Option<String> {
    //let node = handle;
    let content = find_attr(
        handle,
        &LocalName::from("meta"),
        &LocalName::from("content"),
    )?;

    let re = regex::Regex::new(r"charset=(\w+)").unwrap();
    let cap = re.captures(&content)?;
    Some(cap.get(1).unwrap().as_str().to_uppercase())
}
