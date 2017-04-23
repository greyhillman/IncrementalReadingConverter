extern crate html5ever;
extern crate regex;
extern crate xmltree;

use self::html5ever::parse_document;
use self::html5ever::rcdom::{RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

use self::regex::Regex;

use std::collections::HashMap;

use ir_to_anki::IR;
use ir_to_anki;

#[derive(Debug)]
struct Document {
    children: Vec<HTMLNode>,
}

impl Document {
    fn convert(self) -> IR {
        let children = self.children
            .into_iter()
            .map(|child| child.convert())
            .collect();

        IR::Body(children)
    }

    fn optimize(self) -> Document {
        let children = self.children
            .into_iter()
            .flat_map(|child| child.optimize())
            .collect::<Vec<HTMLNode>>();

        Document { children: children }
    }

    fn print_tags(&self) {
        let mut tags = HashMap::new();

        for child in self.children.iter() {
            child.insert_tags(&mut tags);
        }

        println!("{:#?}", tags);
    }
}

#[derive(Debug)]
enum HTMLNode {
    Text(String),
    Node {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<HTMLNode>,
    },
}

impl HTMLNode {
    fn insert_tags(&self, tags: &mut HashMap<String, u32>) {
        match *self {
            HTMLNode::Text(_) => (),
            HTMLNode::Node { ref tag, ref children, .. } => {
                let num = match tags.get(tag) {
                    Some(x) => *x,
                    None => 0 as u32,
                };

                tags.insert(tag.clone(), num + 1);

                for child in children.iter() {
                    child.insert_tags(tags);
                }
            }
        }
    }

    fn to_string(self) -> String {
        match self {
            HTMLNode::Text(x) => x,
            HTMLNode::Node { tag, attributes, children } => {
                let attributes = attributes.into_iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect::<String>();
                let children = children.into_iter()
                    .map(|child| child.to_string())
                    .collect::<String>();

                if attributes.is_empty() {
                    format!("<{}>{}</{}>", tag, children, tag)
                } else {
                    format!("<{} {}>{}</{}>", tag, attributes, children, tag)
                }
            }
        }
    }

    fn to_pre_string(self) -> String {
        match self {
            HTMLNode::Text(x) => x,
            HTMLNode::Node { tag, attributes, children } => {
                let tag = tag;
                let attributes = attributes.into_iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect::<String>();
                let children = children.into_iter()
                    .map(|child| child.to_pre_string())
                    .collect::<String>();

                if tag == "span" {
                    format!("{}", children)
                } else if attributes.is_empty() {
                    format!("<{}>{}</{}>", tag, children, tag)
                } else {
                    format!("<{} {}>{}</{}>", tag, attributes, children, tag)
                }
            }
        }
    }

    fn optimize_node(tag: String,
                     attributes: Vec<(String, String)>,
                     children: Vec<HTMLNode>)
                     -> Vec<HTMLNode> {
        match tag.as_str() {
            "img" => {
                let attrs = attributes.into_iter()
                    .filter(|&(ref k, _)| k == "src")
                    .collect();

                vec![HTMLNode::Node {
                         tag: "img".to_string(),
                         attributes: attrs,
                         children: vec![],
                     }]
            }
            "pre" => {
                let content = children.into_iter()
                    .map(|child| child.to_pre_string())
                    .collect();

                vec![HTMLNode::Node {
                         tag: "pre".to_string(),
                         attributes: vec![],
                         children: vec![HTMLNode::Text(content)],
                     }]
            }
            "code" => {
                let content = children.into_iter()
                    .map(|child| child.to_string())
                    .fold(String::new(), |acc, x| acc + &x);

                vec![HTMLNode::Node {
                         tag: "pre".to_string(),
                         attributes: vec![],
                         children: vec![HTMLNode::Text(content)],
                     }]
            }
            x if children.is_empty() => {
                println!("No children for: {}", x);
                vec![]
            }
            "span" | "a" | "i" | "b" | "em" | "strong" | "h1" | "h2" | "h3" | "h4" |
            "h5" | "h6" | "cite" | "q" | "small" | "button" => {
                children.into_iter()
                    .flat_map(|child| child.optimize())
                    .collect::<Vec<HTMLNode>>()
            }
            "div" | "html" | "body" | "nav" | "header" | "main" | "aside" | "form" |
            "footer" => {
                let children = children.into_iter()
                    .flat_map(|child| child.optimize())
                    .collect::<Vec<HTMLNode>>();

                if children.len() == 1 {
                    return children;
                }

                fn helper(child: HTMLNode) -> HTMLNode {
                    HTMLNode::Node {
                        tag: "div".to_string(),
                        attributes: vec![],
                        children: vec![child],
                    }
                }

                let children = children.into_iter()
                    .map(|child| helper(child))
                    .flat_map(|child| child.optimize())
                    .collect();

                children
            }
            "script" | "meta" | "link" | "head" | "style" | "noscript" => vec![],
            x => {
                let children = children.into_iter()
                    .flat_map(|child| child.optimize())
                    .collect::<Vec<HTMLNode>>();

                println!("{}", x);
                vec![HTMLNode::Node {
                         tag: x.to_string(),
                         attributes: attributes,
                         children: children,
                     }]
            }
        }
    }

    fn optimize(self) -> Vec<HTMLNode> {
        match self {
            HTMLNode::Text(ref x) if x.split_whitespace().count() == 0 => vec![],
            HTMLNode::Text(x) => vec![HTMLNode::Text(x)],
            HTMLNode::Node { tag, attributes, children } => {
                HTMLNode::optimize_node(tag, attributes, children)
            }
        }
    }

    fn convert_to_text_ir(self) -> ir_to_anki::Text {
        match self {
            HTMLNode::Text(x) => ir_to_anki::Text::Text(x),
            HTMLNode::Node { tag, children, .. } => {
                let content = children.into_iter()
                    .map(|child| child.to_string())
                    .collect::<String>();

                match tag.as_str() {
                    "code" => ir_to_anki::Text::Code(content),
                    "sub" => ir_to_anki::Text::Sub(content),
                    "sup" => ir_to_anki::Text::Sup(content),
                    x => {
                        println!("ERROR: converting {} to text ir", x);

                        ir_to_anki::Text::Text("".to_string())
                    }
                }
            }
        }
    }

    fn convert_node(tag: String,
                    attrs: Vec<(String, String)>,
                    children: Vec<HTMLNode>)
                    -> IR {
        match tag.as_str() {
            "img" => {
                let (_, src) = attrs.into_iter()
                    .find(|&(ref k, _)| k == "src")
                    .expect("img has no src");
                IR::Img(src)
            }
            "p" => {
                let children = children.into_iter()
                    .map(|child| child.convert_to_text_ir())
                    .collect();

                IR::Par(children)
            }
            "pre" => {
                let content = match *children.first().unwrap() {
                    HTMLNode::Text(ref x) => x.clone(),
                    HTMLNode::Node { .. } => panic!(),
                };

                IR::Pre(content)
            }
            "ul" => IR::List(ir_to_anki::List::Unordered(vec![])),
            "ol" => IR::List(ir_to_anki::List::Ordered(vec![])),
            "table" => {
                let content = children.into_iter()
                    .map(|child| child.to_string())
                    .collect::<String>();

                IR::Pre(format!("<table>{}</table>", content))
            }
            x => panic!("{}", x),
        }
    }

    fn convert(self) -> IR {
        match self {
            HTMLNode::Text(x) => IR::Par(vec![ir_to_anki::Text::Text(x)]),
            HTMLNode::Node { tag, attributes, children } => {
                HTMLNode::convert_node(tag, attributes, children)
            }
        }
    }
}

fn convert(handle: &Handle) -> HTMLNode {
    let node = handle.borrow();

    use self::html5ever::rcdom::NodeEnum::*;
    match node.node {
        Element(ref name, _, ref attrs) => {
            let tag = name.local.to_string();

            let mut attributes = Vec::new();
            for attr in attrs {
                let key = attr.name.local.to_string();
                let value = attr.value.as_ref().to_string();

                attributes.push((key, value));
            }

            let children = node.children
                .iter()
                .map(|child| convert(child))
                .collect();

            HTMLNode::Node {
                tag: tag,
                attributes: attributes,
                children: children,
            }
        }
        Text(ref text) => HTMLNode::Text(text.as_ref().to_string()),
        Document => {
            let children = node.children
                .iter()
                .map(|child| convert(child))
                .collect();

            HTMLNode::Node {
                tag: "".to_string(),
                attributes: vec![],
                children: children,
            }
        }
        _ => HTMLNode::Text("".to_string()),
    }
}

fn convert_dom(handle: &Handle) -> Document {
    let node = handle.borrow();

    use self::html5ever::rcdom::NodeEnum;
    match node.node {
        NodeEnum::Document => {
            let children = node.children
                .iter()
                .map(|child| convert(child))
                .collect();

            Document { children: children }
        }
        _ => panic!(),
    }
}

pub fn convert_file(contents: &str) -> IR {
    fn fix_non_enclosing_tags(contents: &str) -> String {
        Regex::new(r"<img((.|\n)*?)/{0}>")
            .unwrap()
            .replace_all(contents, "<img$1 />")
            .into_owned()
    }
    let contents = fix_non_enclosing_tags(contents);

    let dom = parse_document(RcDom::default(), Default::default()).one(contents);

    let document = convert_dom(&dom.document);

    let doc = document.optimize();
    println!("Optimized: {:#?}", doc);
    doc.print_tags();

    let doc = doc.convert();

    println!("{:#?}", doc);

    doc
}

#[cfg(test)]
mod tests {
    use super::convert_file;
    use ir_to_anki::IR;
    use ir_to_anki::Text;

    fn body(text: &str) -> String {
        format!("<body>{}</body>", text)
    }

    #[test]
    fn p() {
        let content = body("<p>Test</p>");
        let result = IR::Body(vec![IR::Par(vec![Text::Text("Test".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn p_text_a_text() {
        let content = body("<p>Test <a>a</a> to b</p>");
        let result = IR::Body(vec![IR::Par(vec![Text::Text("Test ".to_string()),
                                                Text::Text("a".to_string()),
                                                Text::Text(" to b".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn img_p() {
        let content = body("<div><img src=\"test.png\" /><p>Caption</p></div>");
        let result = IR::Body(vec![IR::Img("test.png".to_string()),
                                   IR::Par(vec![Text::Text("Caption".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn img_text() {
        let content = body("<div><img src=\"test.png\" />Caption</div>");
        let result = IR::Body(vec![IR::Img("test.png".to_string()),
                                   IR::Par(vec![Text::Text("Caption".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn div_with_text() {
        let content = body("<div>text test</div>");
        let result = IR::Body(vec![IR::Par(vec![Text::Text("text test".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn sup() {
        let content = body("<p><sup>a</sup></p>");
        let result = IR::Body(vec![IR::Par(vec![Text::Sup("a".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn text_text() {
        let content = body("<p>a b</p>");
        let result = IR::Body(vec![IR::Par(vec![Text::Text("a b".to_string())])]);
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn pre_with_tags() {
        let content = body("<pre><h1>Example</h1></pre>");
        let result = IR::Body(vec![IR::Pre("<h1>Example</h1>".to_string())]);
        assert_eq!(convert_file(&content), result);
    }
}
