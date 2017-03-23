extern crate html5ever;
extern crate regex;
extern crate xmltree;

use self::regex::Regex;

use self::html5ever::{parse_document, Attribute};
use self::html5ever::rcdom::{Text, Element, RcDom, Handle};
use self::html5ever::tendril::TendrilSink;

/// Merges two text tags that are next to each other into one.
fn merge_text(contents: &str) -> String {
    let inner_text_re = r"(?:.|\s)+?";
    let text_re = format!("<text>({})</text>", inner_text_re);
    let re = Regex::new(&format!("{0}{0}", text_re)).unwrap();

    let mut result = String::from(contents);
    while re.is_match(&result) {
        result = re.replace_all(&result, r"<text>$1$2</text>").into_owned();
    }
    result
}

fn process(is_in_pre: bool, children: &Vec<Handle>) -> String {
    children.iter()
        .map(|child| walk(is_in_pre, child.clone()))
        .fold(String::new(), |acc, x| acc + &x)
}

fn html_escape(content: &str) -> String {
    let content = String::from(content);
    let replacements = vec![(">", "&gt;"), ("<", "&lt;"), ("&", "&amp;")];
    replacements.iter()
        .fold(content, |acc, tuple| {
            let (x, r) = *tuple;
            Regex::new(x).unwrap()
                .replace_all(&acc, r)
                .into_owned()
        })
}

fn has_img_child(children: &Vec<Handle>) -> bool {
    children.iter()
        .any(|ref x| match x.borrow().node {
            Element(ref name, _, _) => &name.local == "img",
            _ => false,
        })
}

fn has_p_child(children: &Vec<Handle>) -> bool {
    children.iter()
        .any(|ref x| match x.borrow().node {
            Element(ref name, _, _) => &name.local == "p",
            _ => false,
        })
}

fn handle_div(children: &Vec<Handle>, is_in_pre: bool) -> String {
    if !has_img_child(children) && !has_p_child(children) {
        format!("<p>{}</p>", process(is_in_pre, children))
    } else {
        process(is_in_pre, children)
    }
}

fn handle_img(attributes: &Vec<Attribute>) -> String {
    let file = &attributes.iter()
                .find(|attr| &attr.name.local == "src")
                .unwrap()
                .value;
    format!("<img src=\"{}\" />", file)
}

fn handle_element(is_in_pre: bool,
                  tag: &str,
                  attributes: &Vec<Attribute>,
                  children: &Vec<Handle>)
                  -> String {
    match tag {
        "img" => handle_img(attributes),
        "div" => handle_div(children, is_in_pre),
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            format!("<p>{}</p>", process(is_in_pre, children))
        }
        "p" | "li" | "ol" | "ul" | "sup" | "sub" => {
            let content = process(is_in_pre, children);

            if !content.trim().is_empty() {
                format!("<{}>{}</{}>", tag, content, tag)
            } else {
                String::new()
            }
        }
        "body" => format!("<body>{}</body>", process(is_in_pre, children)),
        "pre" => format!("<pre>{}</pre>", process(true, children)),
        "head" | "script" | "noscript" => String::new(),
        _ => process(is_in_pre, children),
    }
}

fn handle_text(is_in_pre: bool, text: &str, children: &Vec<Handle>) -> String {
    println!("{}", text);
    if is_in_pre {
        format!("<text>{}</text>", html_escape(text))
    } else if !text.trim().is_empty() {
        format!("<text>{}</text>", html_escape(text))
    } else {
        process(is_in_pre, children)
    }
}

fn walk(is_in_pre: bool, handle: Handle) -> String {
    let node = handle.borrow();

    match node.node {
        Element(ref name, _, ref attrs) => {
            let tag = &name.local;
            let attributes = attrs;

            handle_element(is_in_pre, tag, attributes, &node.children)
        }
        Text(ref text) => handle_text(is_in_pre, text, &node.children),
        _ => process(is_in_pre, &node.children),
    }
}

fn fix_non_enclosing_tags(contents: &str) -> String {
    Regex::new(r"<img((.|\n)*?)/{0}>")
        .unwrap()
        .replace_all(contents, "<img$1 />")
        .into_owned()
}

pub fn convert_file(contents: &str) -> String {
    let contents = fix_non_enclosing_tags(contents);

    let dom = parse_document(RcDom::default(), Default::default()).one(contents);
    // Ignore any errors right now.

    println!("");
    let result = walk(false, dom.document);
    let result = merge_text(&result);

    result
}

mod tests {
    use super::convert_file;

    fn body(text: &str) -> String {
        format!("<body>{}</body>", text)
    }

    #[test]
    fn empty() {
        assert_eq!(convert_file(""), body(""));
    }

    #[test]
    fn body_empty() {
        assert_eq!(convert_file(&body("")), body(""));
    }

    #[test]
    fn p() {
        let content = body("<p>Test</p>");
        let result = body("<p><text>Test</text></p>");
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn p_text_a_text() {
        let content = body("<p>Test <a>a</a> to b</p>");
        let result = body("<p><text>Test a to b</text></p>");
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn img_p() {
        let content = body("<div><img src=\"test.png\" /><p>Caption</p></div>");
        let result = body("<img src=\"test.png\" /><p><text>Caption</text></p>");
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn img_text() {
        let content = body("<div><img src=\"test.png\" />Caption</div>");
        let result = body("<img src=\"test.png\" /><text>Caption</text>");
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn div_with_text() {
        let content = body("<div>text test</div>");
        let result = body("<p><text>text test</text></p>");
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn sup() {
        let content = body("<sup>a</sup>");
        let result = body("<sup><text>a</text></sup>");
        assert_eq!(convert_file(&content), result);
    }

    #[test]
    fn text_text() {
        let content = body("<text>a </text><text>b</text>");
        let result = body("<text>a b</text>");
        assert_eq!(convert_file(&content), result);
    }

}
