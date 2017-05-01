use super::Nodes;
use super::Node;

use super::super::optimize;
//use super::super::convert;

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
        let doc = self.children.into_iter()
            .flat_map(|child| optimize::remove_tags(child))
            .collect::<Nodes>();
        println!("Removed tags: {:#?}", doc);

        let doc = doc.into_iter()
            .flat_map(|child| optimize::handle_containers(child))
            .collect::<Nodes>();
        println!("handle_containers: {:#?}", doc);

        let doc = doc.into_iter()
            .map(|child| child.into())
            .collect::<ir::Document>();
        println!("convert: {:#?}", doc);

        doc
    }
}
