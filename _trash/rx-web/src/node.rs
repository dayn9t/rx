use std::collections::HashMap;

use html5ever::rcdom::{Handle, NodeData};
use serde::{Deserialize, Serialize};

use crate::req::{self, RequestCfg};
use crate::{html, url};

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinkInfo {
    pub text: String,
    pub url: Option<String>,
}

impl LinkInfo {
    /// 补全URL
    pub fn complete_by(&mut self, page_url: &str) {
        //println!("url: {:?}, {}", &self.url, page_url);
        if let Some(u) = self.url.as_ref() {
            self.url = url::complete(&u, page_url)
        }
    }
}

#[derive(Default, Clone, Serialize, Debug)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub text: Vec<String>,
    pub children: Vec<Node>,
}

impl Node {
    /// 拉取网页创建节点树
    pub fn pull(url: &str, cfg: &RequestCfg) -> Option<Node> {
        let mut data = Vec::new();
        //let resp =
        req::get(url, &mut data, cfg).ok()?;
        //println!("res: {}", resp.headers());
        //println!("Status: {} {}", resp.status_code(), resp.reason());

        let doc = html::Document::parse(&data[..]).ok()?;
        let root = build_from(&doc.dom.document);
        Some(root)
    }

    /// 获取文本长度
    pub fn text_len(&self) -> usize {
        self.text.iter().map(|s| s.len()).sum()
    }

    /// 获取类型
    pub fn type_(&self) -> Option<&String> {
        self.attrs.get("type")
    }

    /// 判断节点是否为脚本
    pub fn type_match(&self, type_: &str) -> bool {
        if let Some(t) = self.type_() {
            if t == type_ {
                return true;
            }
        }
        false
    }

    /// 判断节点是否为脚本
    pub fn is_script(&self) -> bool {
        let t = "text/javascript";
        self.type_match(t)
    }

    /// 判断节点是否为脚本
    pub fn is_css(&self) -> bool {
        let t = "text/css";
        self.type_match(t)
    }

    /// 判断节点是否为链接
    pub fn is_link(&self) -> bool {
        self.name == "a" && self.text_len() > 0 && self.attrs.contains_key("href")
    }

    /// 判断节点是否为链接
    pub fn get_link(&self) -> Option<LinkInfo> {
        if self.is_link() {
            Some(LinkInfo {
                text: self.text.first().unwrap().clone(),
                url: self.attrs.get("href").cloned(),
            })
        } else {
            None
        }
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

    /// 查找第一个链接节点
    pub fn find_first_link(&self) -> Option<Node> {
        self.find_first(&|node| node.is_link())
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
    pub fn find_max_text(&self) -> Vec<String> {
        let node = self
            .find_max(&|node: &Node| {
                if node.is_script() || node.is_css() {
                    0
                } else {
                    node.text_len()
                }
            })
            .unwrap();
        node.text
    }

    /// 查找最多有子节点的的节点
    pub fn find_max_children(&self) -> Option<Node> {
        self.find_max(&|node: &Node| node.children.len())
    }

    /// 查找最大链接列表
    pub fn find_max_links(&self) -> Vec<LinkInfo> {
        let mut vec = Vec::new();
        let node = self.find_max_children().unwrap();
        for child in node.children {
            if let Some(node) = child.find_first_link() {
                vec.push(node.get_link().unwrap());
            }
        }
        vec
    }

    /// 获取h1
    pub fn find_all_h1(&self) -> Vec<Node> {
        self.find_all(&mut |node| node.name == "h1")
    }

    /// 获取标题
    pub fn find_title(&self) -> Option<String> {
        let vec = self.find_all_h1();
        let node = vec.first()?;
        Some(node.text.first()?.to_string())
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

// 遍历节点建造树
fn build_from(handle: &Handle) -> Node {
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
            let child_node = build_from(child);
            node.children.push(child_node);
        }
    }
    node
}
