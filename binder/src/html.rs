use std::collections::HashMap;
pub use std::io::Result;

use html5ever::parse_document;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::tendril::TendrilSink;

use crate::encoding::gbk_to_utf8;


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
    pub fn get_charset(&self) -> Option<String> {
        find_charset(&self.dom.document)
    }
}

#[derive(Default, Clone, Serialize)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub text: Vec<String>,
    pub children: Vec<Box<Node>>,
}

impl Node {
    /// 获取文本长度
    pub fn text_len(&self) -> usize {
        self.text.iter().map(|s| s.len()).sum()
    }

    /// 查找第一个满足条件的节点
    pub fn find_first<Cond>(&self, fun: &Cond) -> Option<Node>
    where
        Cond: Fn(&Node) -> bool,
    {
        let mut first = None;
        self.walk(&mut |node| {
            if fun(node) {
                first = Some(node.clone());
                false
            } else {
                true
            }
        });
        first
    }

    /// 查找全部满足条件的节点
    pub fn find_all<Cond>(&self, fun: &Cond) -> Vec<Node>
    where
        Cond: Fn(&Node) -> bool,
    {
        let mut nodes = Vec::new();
        self.walk(&mut |node| {
            if fun(node) {
                nodes.push(node.clone());
            }
            true
        });
        nodes
    }

    /// 查找最满足条件的节点
    pub fn find_max<F, R>(&self, fun: &F) -> Option<Node>
    where
        F: Fn(&Node) -> R,
        R: Ord,
    {
        let mut max = None;
        self.walk(&mut |node| {
            if max.is_none() || fun(node) > fun(max.as_ref().unwrap()) {
                max = Some(node.clone())
            }
            true
        });
        max
    }

    /// 查找最大文本的节点
    pub fn find_max_text(&self) -> Option<Node> {
        self.find_max(&|node: &Node| node.text_len())
    }

    /// 查找最多有子节点的的节点
    pub fn find_max_children(&self) -> Option<Node> {
        self.find_max(&|node: &Node| node.children.len())
    }

    /// 获取h1
    pub fn find_all_h1(&self) -> Vec<Node> {
        self.find_all(&mut |node| node.name == "h1")
    }

    /// 遍历节点
    pub fn walk<F>(&self, fun: &mut F) -> bool
    where
        F: FnMut(&Node) -> bool,
    {
        if !fun(self) {
            return false;
        }
        for child in &self.children {
            if !child.walk(fun) {
                return false;
            }
        }
        true
    }
}
/*
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
*/

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
                let text = contents.borrow().to_string().trim().to_owned();
                if !text.is_empty() {
                    node.text.push(text);
                }
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

// 遍历属性
fn visit_attrs<F>(node: &Handle, path: &[&str], fun: &mut F) -> bool
where
    F: FnMut(&str, &str) -> bool,
{
    if path.is_empty() {
        return false;
    }
    let node_name = path.first().unwrap();

    match node.data {
        NodeData::Document if *node_name == "" => {}
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            //println!("name.local: {}", name.local);
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
        if name != "content" {
            return false;
        }
        if let Some(cap) = re.captures(&value) {
            charset = Some(cap.get(1).unwrap().as_str().to_uppercase());
            true
        } else {
            false
        }
    });
    charset
}
