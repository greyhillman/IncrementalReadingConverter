extern crate html5ever;

use self::html5ever::rcdom::Handle;

use super::Nodes;

#[derive(Debug)]
pub enum Node {
    Text(String),
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Nodes,
    },
}

impl <'a> From<&'a Handle> for Node {
    fn from(handle: &Handle) -> Self {
        let node = handle.borrow();

        use self::html5ever::rcdom::NodeEnum;
        match node.node {
            NodeEnum::Element(ref name, _, ref attrs) => {
                let tag = name.local.to_string();

                let mut attributes = Vec::new();
                for attr in attrs {
                    let key = attr.name.local.to_string();
                    let value = attr.value.as_ref().to_string();

                    attributes.push((key, value));
                }

                let children = node.children
                    .iter()
                    .map(|child| Node::from(child))
                    .collect();

                Node::Element {
                    tag,
                    attributes,
                    children
                }
            }
            NodeEnum::Text(ref text) => {
                Node::Text(text.as_ref().to_string())
            }
            _ => Node::Text(String::new()),
        }
    }
}