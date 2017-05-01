extern crate html5ever;
extern crate xmltree;
extern crate itertools;

use self::html5ever::rcdom::Handle;

mod node;
pub use self::node::Node;

mod nodes;
pub use self::nodes::Nodes;

mod document;
pub use self::document::Document;

pub fn convert_dom(handle: &Handle) -> Result<Document, &'static str> {
    let node = handle.borrow();

    use self::html5ever::rcdom::NodeEnum;
    match node.node {
        NodeEnum::Document => {
            Ok(node.children
                .iter()
                .map(Node::from)
                .fold(Document::new(), |doc, child| doc.add(child)))
        }
        _ => Err("Converting from non-Document node"),
    }
}
