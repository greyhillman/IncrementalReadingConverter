extern crate xmltree;
use self::xmltree::Element;

use group_lines::group_lines_file;

fn handle(element: &Element) -> String {
    match element.name.as_str() {
        "body" => handle_children(element),
        "p" => format!("\n{}\n", handle_children(element).trim()),
        "text" => handle_text("", element, ""),
        "sub" => handle_text("_{", element, "}"),
        "sup" => handle_text("^{", element, "}"),
        "img" => format!("\n{}\n", handle_img(element)),
        "ol" | "ul" | "li" => handle_list(0, element),
        _ => String::new(),
    }
}

fn handle_list(depth: u8, element: &Element) -> String {
    match element.name.as_str() {
        "ol" => format!("{}\n", handle_ol(depth + 1, element).trim()),
        "ul" => format!("{}\n", handle_ul(depth + 1, element).trim()),
        "li" => format!("{}", handle_li(depth, element).trim()),
        _ => handle(element),
    }
}

fn handle_li(depth: u8, element: &Element) -> String {
    element.children
        .iter()
        .map(|x| handle_list(depth, x))
        .filter(|x| !x.is_empty())
        .fold(String::new(), |acc, x| acc + &x + "\n")
}

fn get_indent(depth: u8) -> String {
    fn helper(acc: String, depth: u8) -> String {
        match depth {
            0 => acc,
            1 => helper(acc + "-", depth - 1),
            _ => helper(acc + "--", depth - 1),
        }
    }
    helper(String::new(), depth)
}

fn handle_ul(depth: u8, element: &Element) -> String {
    element.children
        .iter()
        .map(|x| format!("{} {}", get_indent(depth), handle_list(depth, x)))
        .fold(String::new(), |acc, x| acc + &x + "\n")
        .trim()
        .to_string()
}

fn handle_ol(depth: u8, element: &Element) -> String {
    let indent = if depth <= 1 {
        String::new()
    } else {
        get_indent(depth - 1) + "-"
    };

    element.children.iter()
        .map(|x| handle_list(depth, x))
        .zip((1..)) // Used for the numbers
        .map(|(child, li)| format!("{}{}) {}", indent, li, child))
        .fold(String::new(), |acc, x| acc + &x + "\n").trim().to_string()
}

fn handle_img(element: &Element) -> String {
    let (_, href) = element.attributes.iter()
        .find(|&(ref k, _)| *k == "href")
        .unwrap();
    let file = href.split("/").last().unwrap();

    format!("<img href=\"{}\" />", file)
}

fn handle_children(element: &Element) -> String {
    element.children
        .iter()
        .map(|x| handle(x))
        .filter(|x| !x.is_empty())
        .fold(String::new(), |acc, x| acc + &x)
}

fn handle_text(start: &str, element: &Element, end: &str) -> String {
    let children_content = handle_children(element).trim().to_string();
    let content = match element.text {
        Some(ref s) => format!("{}", s),
        None => String::new(),
    };

    if children_content.is_empty() {
        format!("{}{}{}", start, group_lines_file(&content), end)
    } else {
        format!("{}{}{}", start, group_lines_file(&children_content), end)
    }
}

pub fn convert_file(contents: &str) -> String {
    let contents = Element::parse(contents.as_bytes()).unwrap();

    handle(&contents).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_par() {
        let contents = "<body>
        <p>
            <text>Test
            test</text>
        </p>
        </body>";
        assert_eq!(convert_file(contents), String::from("Test test"));
    }

    #[test]
    fn convert_par_par() {
        let contents = "<body>
        <p><text>Test</text></p>
        <p><text>Test</text></p>
        </body>";
        let result = "Test\n\nTest".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_par_sub() {
        let contents = "\
        <body>\
        <p>
            <text>Test</text>
            <sub>2</sub>
        </p>
        </body>";
        let result = String::from("Test_{2}");
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_par_sup() {
        let contents = "\
        <body>\
        <p>
            <text>Test</text>
            <sup>2</sup>
        </p>
        </body>";
        let result = String::from("Test^{2}");
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_ul() {
        let contents = "<body>
        <ul>
        <li><text>a</text></li>
        <li><text>b</text></li>
        <li><text>c</text></li>
        </ul>
        </body>";
        let result = "- a\n- b\n- c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_ol() {
        let contents = "<body>
        <ol>
        <li><text>a</text></li>
        <li><text>b</text></li>
        <li><text>c</text></li>
        </ol>
        </body>";
        let result = "1) a\n2) b\n3) c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_nested_ul_ul() {
        let contents = "<body>
        <ul>
        <li><text>a</text></li>
        <li><text>b</text>
            <ul>
                <li><text>b1</text></li>
                <li><text>b2</text></li>
            </ul>
        </li>
        <li><text>c</text></li>
        </ul>
        </body>";
        let result = "- a\n- b\n--- b1\n--- b2\n- c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_nested_ol_ul() {
        let contents = "<body>
        <ol>
        <li><text>a</text></li>
        <li><text>b</text>
            <ul>
                <li><text>b1</text></li>
                <li><text>b2</text></li>
            </ul>
        </li>
        <li><text>c</text></li>
        </ol>
        </body>";
        let result = "1) a\n2) b\n--- b1\n--- b2\n3) c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_nested_ul_ol() {
        let contents = "<body>
        <ul>
        <li><text>a</text></li>
        <li><text>b</text>
            <ol>
                <li><text>b1</text></li>
                <li><text>b2</text></li>
            </ol>
        </li>
        <li><text>c</text></li>
        </ul>
        </body>";
        let result = "- a\n- b\n--1) b1\n--2) b2\n- c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_nested_ol_ol() {
        let contents = "<body>
        <ol>
        <li><text>a</text></li>
        <li><text>b</text>
            <ol>
                <li><text>b1</text></li>
                <li><text>b2</text></li>
            </ol>
        </li>
        <li><text>c</text></li>
        </ol>
        </body>";
        let result = "1) a\n2) b\n--1) b1\n--2) b2\n3) c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_par_ul() {
        let contents = "<body>
        <p><text>List:</text></p>
        <ul>
        <li><text>a</text></li>
        <li><text>b</text></li>
        <li><text>c</text></li>
        </ul>
        </body>";
        let result = "List:\n- a\n- b\n- c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_par_ol() {
        let contents = "<body>
        <p><text>List:</text></p>
        <ol>
        <li><text>a</text></li>
        <li><text>b</text></li>
        <li><text>c</text></li>
        </ol>
        </body>";
        let result = "List:\n1) a\n2) b\n3) c".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_ul_par() {
        let contents = "<body>
        <ul>
        <li><text>a</text></li>
        <li><text>b</text></li>
        <li><text>c</text></li>
        </ul>
        <p><text>List</text></p>
        </body>";
        let result = "- a\n- b\n- c\n\nList".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_ol_par() {
        let contents = "<body>
        <ol>
        <li><text>a</text></li>
        <li><text>b</text></li>
        <li><text>c</text></li>
        </ol>
        <p><text>List</text></p>
        </body>";
        let result = "1) a\n2) b\n3) c\n\nList".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn text_in_sup() {
        let contents = "<body><sup><text>a</text></sup></body>";
        let result = "^{a}".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn shorten_img_href() {
        let contents = "<body><img href=\"a/b/c/d.png\" /></body>";
        let result = "<img href=\"d.png\" />".to_string();
        assert_eq!(convert_file(contents), result);
    }
}
