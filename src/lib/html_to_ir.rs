extern crate html5ever;
extern crate regex;
extern crate xmltree;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

use self::regex::Regex;

#[derive(Debug)]
struct Document {
    children: Vec<HTMLNode>,
}

#[derive(Debug)]
enum HTMLNode {
    Text(String),
    Node {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<HTMLNode>,
    },
}

fn convert(handle: &Handle) -> HTMLNode {
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

            HTMLNode::Node {
                tag: tag,
                attributes: attributes,
                children: children,
            }
        }
        Text(ref text) => HTMLNode::Text(text.as_ref().to_string()),
        Document => {
            let children = node.children
                .iter()
                .map(|child| convert(child))
                .collect();

            HTMLNode::Node {
                tag: "".to_string(),
                attributes: vec![],
                children: children,
            }
        }
        _ => HTMLNode::Text("".to_string()),
    }
}

fn convert_dom(handle: &Handle) -> Document {
    let node = handle.borrow();

    use self::html5ever::rcdom::NodeEnum;
    match node.node {
        NodeEnum::Document => {
            let children = node.children
                .iter()
                .map(|child| convert(child))
                .collect();

            Document { children: children }
        }
        _ => panic!(),
    }
}

pub fn convert_file(contents: &str) {
    fn fix_non_enclosing_tags(contents: &str) -> String {
        Regex::new(r"<img((.|\n)*?)/{0}>")
            .unwrap()
            .replace_all(contents, "<img$1 />")
            .into_owned()
    }
    let contents = fix_non_enclosing_tags(contents);

    let dom = parse_document(RcDom::default(), Default::default()).one(contents);

    let _ = convert_dom(&dom.document);

    panic!()
}

#[cfg(test)]
mod tests {

}
