extern crate html5ever;
extern crate xmltree;
extern crate itertools;

use self::html5ever::rcdom::Handle;

use std::iter::FromIterator;

mod node;
pub use self::node::Node;

mod nodes;
pub use self::nodes::Nodes;

mod document;
pub use self::document::Document;

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
