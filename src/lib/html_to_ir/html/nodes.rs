use super::Node;

use std::iter::FromIterator;

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
            (Some(Node::Text(x)), Node::Text(y)) => {
                nodes.push(Node::Text(x + &y));
            }
            (Some(x), Node::Text(y)) => {
                if y.is_empty() {
                    nodes.push(x);
                } else {
                    nodes.push(x);
                    nodes.push(Node::Text(y));
                }
            }
            (Some(x), y) => {
                nodes.push(x);
                nodes.push(y);
            }
        }

        Nodes { nodes: nodes }
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
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

impl From<Node> for Nodes {
    fn from(node: Node) -> Self {
        Nodes::new().add(node)
    }
}
