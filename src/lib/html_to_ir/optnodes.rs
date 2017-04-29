use html_to_ir::optnode;
use html_to_ir::html;
use std::iter::FromIterator;


#[derive(Debug)]
pub struct OptNodes {
    nodes: Vec<optnode::OptNode>,
}

impl OptNodes {
    pub fn new() -> Self {
        OptNodes { nodes: vec![] }
    }

    pub fn add(self, node: optnode::OptNode) -> Self {
        self
    }
}

impl From<html::Nodes> for OptNodes {
    fn from(nodes: html::Nodes) -> Self {
        nodes.into_iter()
            .map(optnode::OptNode::from)
            .fold(OptNodes::new(), |nodes, node| nodes.add(node))
    }
}

impl IntoIterator for OptNodes {
    type Item = optnode::OptNode;
    type IntoIter = ::std::vec::IntoIter<optnode::OptNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl FromIterator<optnode::OptNode> for OptNodes {
    fn from_iter<I: IntoIterator<Item = optnode::OptNode>>(iter: I) -> Self {
        iter.into_iter()
            .fold(OptNodes::new(), |nodes, node| nodes.add(node))
    }
}

//impl FromIterator<OptNodes> for OptNodes {
//    fn from_iter<I: IntoIterator<Item=OptNodes>>(iter: I) -> Self {
//        iter.into_iter()
//            .map(|nodes| nodes.into_iter())
//            .fold(OptNodes::new().into_iter(), |iter, i| iter.chain(i))
//            .fold(OptNodes::new(), |nodes, node| nodes.add(node))
//    }
//}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing() {}
}
