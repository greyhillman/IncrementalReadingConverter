use ir;
use super::html::Node;
use super::html::Nodes;

impl Into<ir::IR> for Node {
    fn into(self) -> ir::IR {
        match self {
            Node::Text(x) => {
                println!("Shouldn't really have a text node at the top level.");

                ir::IR::from(convert_textblock(Nodes::from(Node::Text(x))))
            }
            Node::Element { tag, attributes, mut children } => {
                match tag.as_str() {
                    "img" => {
                        let (_, src) = attributes.into_iter()
                            .find(|&(ref k, _)| k == "src")
                            .expect("img tag has no src attribute");

                        ir::IR::img(&src)
                    }
                    "p" => {
                        ir::IR::from(convert_textblock(children))
                    }
                    "pre" => {
                        let node = children.pop().unwrap_or(Node::Text("".to_string()));

                        let content = match node {
                            Node::Text(x) => x,
                            _ => {
                                println!("Pre tag has non-text as children");
                                String::new()
                            }
                        };

                        ir::IR::pre(&content)
                    }
                    _ => {
                        ir::IR::pre(&format!("Could not handle element {}.", tag))
                    }
                }
            }
        }
    }
}

pub fn convert_textblock(nodes: Nodes) -> ir::TextBlock {
    nodes.into_iter()
        .map(|node| {
            match node {
                Node::Text(x) => ir::Text::text(&x),
                Node::Element { tag, mut children, .. } => {
                    match tag.as_str() {
                        "sup" => ir::Text::Sup(convert_textblock(children)),
                        "sub" => ir::Text::Sub(convert_textblock(children)),
                        "code" => {
                            let node = children.pop().unwrap_or(Node::Text("".to_string()));

                            let content = match node {
                                Node::Text(x) => x,
                                _ => {
                                    println!("code tag has non-text as children");
                                    String::new()
                                }
                            };
                            ir::Text::Code(content)
                        }
                        _ => ir::Text::text("lmoa"),
                    }
                }
            }
        })
        .fold(ir::TextBlock::new(), |block, node| block.add(node))
}
