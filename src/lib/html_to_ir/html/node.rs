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
