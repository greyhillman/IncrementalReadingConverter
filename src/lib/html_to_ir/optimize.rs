use super::html::Nodes;
use super::html::Node;

pub fn remove_tags(node: Node) -> Nodes {
    fn handle_children(children: Nodes) -> Nodes {
        children.into_iter()
            .flat_map(remove_tags)
            .collect()
    }
    match node {
        Node::Text(x) => {
            Nodes::from(Node::Text(x))
        }
        Node::Element { tag, attributes, children } => {
            let children = handle_children(children);

            match tag.as_str() {
                "head" | "script" | "style" | "span" => children,
                "nav" | "header" | "footer" | "body" | "html" => {
                    let tag = "div".to_string();

                    Nodes::from(Node::Element {
                        tag,
                        attributes: vec![],
                        children,
                    })
                }
                _ => {
                    Nodes::from(Node::Element {
                        tag,
                        attributes,
                        children,
                    })
                }
            }
        }
    }
}

pub fn handle_containers(node: Node) -> Nodes {
    fn handle_children(children: Nodes) -> Nodes {
        children.into_iter()
            .flat_map(handle_containers)
            .collect()
    }
    match node {
        Node::Text(x) => Nodes::from(Node::Text(x)),
        Node::Element { tag, attributes, children } => {
            let children = handle_children(children);

            match tag.as_str() {
                "div" => {
                    children
                }
                _ => {
                    Nodes::from(Node::Element {
                        tag,
                        attributes,
                        children
                    })
                }
            }
        }
    }
}
