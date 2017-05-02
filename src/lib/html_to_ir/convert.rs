extern crate log;

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
                    "ol" => {
                        info!("Converting ol");
                        ir::IR::from(convert_list(ir::ListType::Unordered, children))
                    }
                    "ul" => {
                        ir::IR::from(convert_list(ir::ListType::Unordered, children))
                    }
                    _ => {
                        info!("Could not handle element {}", tag);
                        ir::IR::pre(&format!("Could not handle element {}.", tag))
                    }
                }
            }
        }
    }
}

fn convert_list(style: ir::ListType, items: Nodes) -> ir::List {
    items.into_iter()
         .filter(|ref child| child.is_element())
         .map(|child| match child {
             Node::Text(_) => panic!("There should be no text now"),
             Node::Element { tag, children, .. } => {
                 match tag.as_str() {
                     "li" => convert_list_item(children),
                     _ => panic!("There is a none li tag in the list"),
                 }
             }
         })
         .fold(ir::List::new(style), |list, item| list.add(item))
}

fn convert_list_item(children: Nodes) -> ir::ListItem {
    children.into_iter()
        .map(|child| match child {
            Node::Text(x) => ir::ListContent::from(x),
            Node::Element { tag, children, .. } => {
                match tag.as_str() {
                    "ol" => {
                        ir::ListContent::from(convert_list(ir::ListType::Ordered, children))
                    }
                    "ul" => {
                        ir::ListContent::from(convert_list(ir::ListType::Unordered, children))
                    }
                    _ => {
                        panic!("Found {} tag in a list", tag)
                    }
                }
            }
        })
        .fold(ir::ListItem::new(), |item, content| item.add(content))
}

fn convert_textblock(nodes: Nodes) -> ir::TextBlock {
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
