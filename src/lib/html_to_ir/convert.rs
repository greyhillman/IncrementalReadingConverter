extern crate log;

use ir;
use super::html::Node;
use super::html::Nodes;

impl From<Node> for ir::IR {
    fn from(node: Node) -> ir::IR {
        match node {
            Node::Text(x) => {
                info!("Shouldn't really have a text node at the top level.");

                ir::IR::from(convert_textblock(Nodes::from(Node::Text(x))))
            }
            Node::Element { tag, attributes, children } => {
                match tag.as_str() {
                    "img" => convert_img(attributes),
                    "p" => convert_p(children),
                    "pre" => convert_pre(children),
                    "ol" => convert_ol(children),
                    "ul" => convert_ul(children),
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        convert_h(tag, children)
                    }
                    _ => {
                        let msg = format!("Could not handle `{}` element", tag);
                        //info!(msg);

                        ir::IR::pre(&msg)
                    }
                }
            }
        }
    }
}

fn convert_h(tag: String, mut children: Nodes) -> ir::IR {
    let digit = &tag[1..2];
    let level = usize::from_str_radix(digit, 10)
        .unwrap();

    let text = children.pop()
        .unwrap_or(Node::Text("".to_string()))
        .unwrap_text()
        .unwrap_or("".to_string());

    ir::IR::header(level, text.trim())
}

fn convert_ol(children: Nodes) -> ir::IR {
    ir::IR::from(convert_list(ir::ListType::Ordered, children))
}

fn convert_ul(children: Nodes) -> ir::IR {
    ir::IR::from(convert_list(ir::ListType::Unordered, children))
}

fn convert_pre(mut children: Nodes) -> ir::IR {
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

fn convert_p(children: Nodes) -> ir::IR {
    ir::IR::from(convert_textblock(children))
}

fn convert_img(attributes: Vec<(String, String)>) -> ir::IR {
    let (_, src) = attributes.into_iter()
        .find(|&(ref k, _)| k == "src")
        .expect("img tag has no src attribute");

    ir::IR::img(&src)
}

fn convert_list(style: ir::ListType, items: Nodes) -> ir::List {
    items.into_iter()
        .filter(|ref child| child.is_element())
        .map(|child| match child {
            Node::Text(_) => panic!("There should be no text now"),
            Node::Element { tag, children, .. } => {
                match tag.as_str() {
                    "li" => convert_list_item(children),
                    _ => panic!("There is a non li tag in the list: {}", tag),
                }
            }
        })
        .fold(&mut ir::List::new(style), |list, item| list.add(item))
        .build()
}

fn convert_list_item(children: Nodes) -> ir::ListItem {
    fn convert_element(tag: String, children: Nodes) -> ir::ListContent {
        match tag.as_str() {
            "ol" => {
                ir::ListContent::from(convert_list(ir::ListType::Ordered,
                                                   children))
            }
            "ul" => {
                ir::ListContent::from(convert_list(ir::ListType::Unordered,
                                                   children))
            }
            "p" => ir::ListContent::from(convert_textblock(children)),
            "sup" => {
                let content = convert_textblock(children);
                let sup = ir::Text::Sup(content);

                ir::ListContent::from(ir::TextBlock::from(sup))
            }
            "code" => {
                let node = Node::Element {
                    tag,
                    children,
                    attributes: vec![]
                };
                let nodes = Nodes::from(node);

                ir::ListContent::from(convert_textblock(nodes))
            }
            "pre" => {
                let node = Node::Element {
                    tag: "code".to_string(),
                    children,
                    attributes: vec![],
                };
                let nodes = Nodes::from(node);

                ir::ListContent::from(convert_textblock(nodes))
            }
            _ => panic!("Found {} tag in a list", tag),
        }
    }
    children.into_iter()
        .map(|child| match child {
            Node::Text(x) => ir::ListContent::from(x),
            Node::Element { tag, children, .. } => {
                convert_element(tag, children)
            }
        })
        .fold(&mut ir::ListItem::new(), |item, content| item.add(content))
        .build()
}

fn convert_textblock(nodes: Nodes) -> ir::TextBlock {
    nodes.into_iter()
        .map(|node| match node {
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
        })
        .fold(&mut ir::TextBlock::new(), |block, node| block.add(node))
        .build()
}
