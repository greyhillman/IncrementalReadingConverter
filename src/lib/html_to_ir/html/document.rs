use super::Nodes;
use super::Node;

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

    pub fn convert(self) -> ir::Document {
        ir::Document::new()
    }
}
