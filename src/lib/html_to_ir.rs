extern crate html5ever;

use std::iter::repeat;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

fn process(children: &Vec<Handle>) -> String {
    children.iter()
        .map(|child| walk(child.clone()))
        .fold(String::new(), |acc, x| format!("{}{}", acc, x))
}

fn walk(handle: Handle) -> String {
    let node = handle.borrow();

    match node.node {
        Text(ref text) => {
            if node.children.is_empty() {
                escape_default(text)
            } else {
                panic!("shit")
            }
        }
        Element(ref name, _, ref attrs) => {
            let tag = &name.local;
            let attributes = attrs.iter()
                .map(|attr| format!(" {}=\"{}\"", attr.name.local, attr.value))
                .fold(String::new(), |acc, x| acc + &x);
            let inner = process(&node.children);

            format!("<{} {}>{}</{}>", tag, attributes, inner, tag)
        }
        _ => {
            if !node.children.is_empty() {
                process(&node.children)
            } else {
                "".to_string()
            }
        }
    }
}

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn html_to_ir(contents: &str) -> String {
    let dom = parse_document(RcDom::default(), Default::default())
        .one(contents);
    // Ignore any errors right now.

    println!("");
    format!("<body>{}</body>", walk(dom.document))
}

mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(html_to_ir(""), "<body></body>".to_string());
    }

    #[test]
    fn body() {
        assert_eq!(html_to_ir("<body></body>"), "<body></body>".to_string());
    }

    #[test]
    fn p() {
        assert_eq!(html_to_ir("<body><p>Test</p></body>"), "".to_string());
    }

}
