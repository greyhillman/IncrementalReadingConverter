extern crate html5ever;
extern crate xmltree;
extern crate itertools;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

use self::itertools::join;

use std::iter::FromIterator;

use ir;

#[derive(Debug)]
pub struct Document {
    children: Nodes,
}

impl Document {
    pub fn new() -> Self {
        Document { children: Nodes::new() }
    }

    pub fn add(self, child: Node) -> Self {
        Document { children: self.children.add(child) }
    }
}

#[derive(Debug)]
pub struct Nodes {
    nodes: Vec<Node>,
}

impl Nodes {
    pub fn new() -> Self {
        Nodes { nodes: vec![] }
    }

    pub fn add(self, node: Node) -> Self {
        let mut nodes = self.nodes;

        match (nodes.pop(), node) {
            (None, Node::Text(ref x)) if x.is_empty() => (),
            (None, x) => nodes.push(x),
            (Some(x), Node::Text(y)) => {
                if y.is_empty() {
                    nodes.push(x);
                } else {
                    nodes.push(x);
                    nodes.push(Node::Text(y))
                }
            }
            (Some(Node::Text(x)), Node::Text(y)) => nodes.push(Node::Text(x + &y)),
            (Some(x), y) => {
                nodes.push(x);
                nodes.push(y);
            }
        }

        Nodes { nodes: nodes }
    }
}

impl FromIterator<Node> for Nodes {
    fn from_iter<I: IntoIterator<Item = Node>>(iter: I) -> Self {
        iter.into_iter()
            .fold(Nodes::new(), |nodes, node| nodes.add(node))
    }
}

impl IntoIterator for Nodes {
    type Item = Node;
    type IntoIter = ::std::vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

#[derive(Debug)]
pub enum Node {
    Text(String),
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Nodes,
    },
}

fn convert(handle: &Handle) -> Node {
    let node = handle.borrow();

    use self::html5ever::rcdom::NodeEnum::*;
    match node.node {
        Element(ref name, _, ref attrs) => {
            let tag = name.local.to_string();

            let mut attributes = Vec::new();
            for attr in attrs {
                let key = attr.name.local.to_string();
                let value = attr.value.as_ref().to_string();

                attributes.push((key, value));
            }

            let children = node.children
                .iter()
                .map(|child| convert(child))
                .collect();

            Node::Element {
                tag: tag,
                attributes: attributes,
                children: children,
            }
        }
        Text(ref text) => Node::Text(text.as_ref().to_string()),
        _ => Node::Text("".to_string()),
    }
}

pub fn convert_dom(handle: &Handle) -> Result<Document, &'static str> {
    let node = handle.borrow();

    use self::html5ever::rcdom::NodeEnum;
    match node.node {
        NodeEnum::Document => {
            Ok(node.children
                .iter()
                .map(|child| convert(child))
                .fold(Document::new(), |doc, child| doc.add(child)))
        }
        _ => Err("Converting from non-Document node"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing() {}
}
