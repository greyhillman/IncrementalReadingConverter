extern crate html5ever;

use self::html5ever::{parse_document, Attribute};
use self::html5ever::rcdom::{Text, Element, RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

fn process(children: &Vec<Handle>) -> String {
    children.iter()
        .map(|child| walk(child.clone()))
        .fold(String::new(), |acc, x| format!("{}{}", acc, x))
}

fn needs_to_be_figure(children: &Vec<Handle>) -> bool {
    for child in children.iter() {
        match child.borrow().node {
            Element(ref name, _, _) => {
                match name.local.trim() {
                    "img" => return true,
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    false
}

fn handle_element(tag: &str, attributes: &Vec<Attribute>, children: &Vec<Handle>) -> String {
    match tag {
        "p" => format!("<p>{}</p>", process(children)),
        "body" => format!("<body>{}</body>", process(children)),
        "li" => format!("<li>{}</li>", process(children)),
        "ol" => format!("<ol>{}</ol>", process(children)),
        "ul" => format!("<ul>{}</ul>", process(children)),
        "img" => format!("<img src=\"{}\" />", 
                         &attributes.iter()
                            .find(|attr| &attr.name.local == "src")
                            .unwrap()
                            .value),
        "div" => if needs_to_be_figure(children) {
            format!("<figure>{}</figure>", process(children))
        } else {
            process(children)
        },
        "head" | "script" => String::new(),
        _ => process(children),
    }
}

fn walk(handle: Handle) -> String {
    let node = handle.borrow();

    match node.node {
        Text(ref text) if !text.trim().is_empty() => format!("<text>{}</text>", text.trim()),
        Element(ref name, _, ref attrs) => {
            let tag = &name.local;
            let attributes = attrs;

            let result = handle_element(tag, attributes, &node.children);
            result
        }
        _ => process(&node.children),
    }
}

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn convert_file(contents: &str) -> String {
    let dom = parse_document(RcDom::default(), Default::default()).one(contents);
    // Ignore any errors right now.

    println!("");
    walk(dom.document)
}

mod tests {
    use super::convert_file;

    #[test]
    fn empty() {
        assert_eq!(convert_file(""), "<body></body>".to_string());
    }

    #[test]
    fn body() {
        assert_eq!(convert_file("<body></body>"), "<body></body>".to_string());
    }

    #[test]
    fn p() {
        let content = "<body><p>Test</p></body>";
        let result = "<body><p><text>Test</text></p></body>".to_string();
        assert_eq!(convert_file(content), result);
    }

    #[test]
    fn img() {
        let content = "<body><div><img src=\"test.png\" /><p>Caption</p></div></body>";
        let result = "<body><figure>\
                      <img src=\"test.png\" /><text>Caption</text>\
                      </figure></body>".to_string();
        assert_eq!(convert_file(content), result);
    }

}
