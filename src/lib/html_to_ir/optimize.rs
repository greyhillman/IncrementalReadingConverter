use super::html::Nodes;
use super::html::Node;

pub fn remove_tags(node: Node) -> Nodes {
    fn handle_children(children: Nodes) -> Nodes {
        children.into_iter()
            .flat_map(remove_tags)
            .collect()
    }
    match node {
        Node::Text(x) => Nodes::from(Node::Text(x)),
        Node::Element { tag, attributes, children } => {
            let children = handle_children(children);

            match tag.as_str() {
                // Contents are not useful
                "head" | "button" | "noscript" | "form" | "script" | "style" => Nodes::new(),
                // Contents are useful
                "a" | "i" | "em" | "strong" | "mark" | "b" | "span" 
                    | "cite" | "q" => children,
                "dl" => {
                    Nodes::from(Node::Element {
                        tag: "ul".to_string(),
                        attributes: vec![],
                        children,
                    })
                }
                "dd" | "dt" => {
                    Nodes::from(Node::Element {
                        tag: "li".to_string(),
                        attributes: vec![],
                        children,
                    })
                }
                // Contents are useful but tags are containers
                "nav" | "header" | "footer" | "body" | "html" => {
                    let tag = "div".to_string();

                    Nodes::from(Node::Element {
                        tag: tag,
                        attributes: vec![],
                        children: children,
                    })
                }
                _ => {
                    Nodes::from(Node::Element {
                        tag: tag,
                        attributes: attributes,
                        children: children,
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
                "div" => children,
                _ => {
                    Nodes::from(Node::Element {
                        tag: tag,
                        attributes: attributes,
                        children: children,
                    })
                }
            }
        }
    }
}
