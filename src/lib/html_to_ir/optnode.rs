extern crate itertools;

use self::itertools::join;

use html_to_ir::optnodes::OptNodes;
use html_to_ir::html;

use ir;

#[derive(Debug)]
pub enum Tag {
    Code,
    Pre,
    P,
    Table,
    THead,
    TBody,
    TFoot,
    TR,
    TD,
    Img,
    Div,
    UL,
    OL,
    LI,
    Sub,
    Sup,
}

#[derive(Debug)]
pub enum OptNode {
    Text(String),
    Code(String),
    Pre(String),
    Node(Tag, OptNodes),
    Nothing
}

impl OptNode {
    fn optimize_node(tag: Tag, children: OptNodes) -> OptNodes {
        children
    }

    pub fn optimize(self) -> OptNodes {
        match self {
            OptNode::Nothing => OptNodes::new(),
            OptNode::Node(tag, children) => OptNode::optimize_node(tag, children),
            x => OptNodes::new(),
        }
    }
}

impl From<html::Node> for OptNode {
    fn from(node: html::Node) -> Self {
        fn to_code_string_children(children: html::Nodes) -> String {
            String::new()
        }
        fn from_children(children: html::Nodes) -> OptNodes {
            OptNodes::from(children)
        }

        match node {
            html::Node::Text(x) => OptNode::Text(x),
            html::Node::Element { tag, attributes, children } => {
                match tag.as_str() {
                    "img" => OptNode::Node(Tag::Img, OptNodes::new()),
                    "pre" => {
                        OptNode::Pre(to_code_string_children(children))
                    }
                    "code" => {
                        OptNode::Code(to_code_string_children(children))
                    }
                    "sub" => OptNode::Node(Tag::Sub, OptNodes::from(children)),
                    "sup" => OptNode::Node(Tag::Sup, OptNodes::from(children)),
                    "li" => OptNode::Node(Tag::LI, OptNodes::from(children)),
                    "ol" => OptNode::Node(Tag::OL, OptNodes::from(children)),
                    "ul" => OptNode::Node(Tag::UL, OptNodes::from(children)),
                    x => {
                        println!("No conversion from {} to OptNode", x);
                        OptNode::Nothing
                    }
                }
            }
        }
    }
}

