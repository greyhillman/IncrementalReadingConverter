extern crate xmltree;
use xmltree::Element;

use std::iter::repeat;

pub mod group_lines;

fn handle(element: &Element) -> String {
    match element.name.as_str() {
        "body" => handle_children(element, "\n"),
        "p" => handle_children(element, "") + "\n",
        "text" => handle_text(element, ("", "")),
        "sub" => handle_text(element, ("_{", "}")),
        "sup" => handle_text(element, ("^{", "}")),
        "img" => handle_img(element),
        "figure" => handle_children(element, "\n"),
        "ol" | "ul" | "li" => handle_list(0, element),
        _ => String::new(),
    }
}

fn handle_list(depth: u8, element: &Element) -> String {
    match element.name.as_str() {
        "ol" => handle_ol(depth + 1, element).trim().to_string() + "\n",
        "ul" => handle_ul(depth + 1, element).trim().to_string() + "\n",
        "li" => handle_li(depth, element).trim().to_string(),
        _ => handle(element),
    }
}

fn handle_li(depth: u8, element: &Element) -> String {
    element.children.iter()
        .map(|x| handle_list(depth, x))
        .fold(String::new(), |acc, x| acc + &x + "\n")
}

fn get_indent(depth: u8) -> String {
    match depth {
        0 => "".to_string(),
        _ => repeat("-").take(usize::from(1 + 2 * (depth - 1))).collect(),
    }
}

fn handle_ul(depth: u8, element: &Element) -> String {
    element.children.iter()
        .map(|x| format!("{} {}", get_indent(depth), handle_list(depth, x)))
        .fold(String::new(), |acc, x| acc + &x + "\n").trim().to_string()
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
    let attributes = element.attributes.iter()
        .fold(String::new(), |acc, (k, v)| {
            format!("{}{}=\"{}\" ", acc, k, v)
        });
    format!("<img {}/>", attributes)
}

fn handle_children(element: &Element, sep: &str) -> String {
    element.children.iter()
        .map(|x| handle(x))
        .fold(String::new(), |acc, x| acc + &x + sep)
}

fn handle_text(element: &Element, format: (&str, &str)) -> String {
    let (start, end) = format;
    match element.text {
        Some(ref s) => {
            format!("{}{}{}", start, group_lines::group_lines_file(s), end)
        },
        None => String::new(),
    }
}

pub fn convert_file(contents: &str) -> String {
    let contents = contents.lines()
        .map(|x| x.trim())
        .fold(String::new(), |acc, x| acc + &x + "\n");

    let element = Element::parse(contents.as_bytes()).unwrap();

    handle(&element).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_par() {
        let contents = "\
        <body>\
        <p>
            <text>Test
            test</text>
        </p>\
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
    fn convert_figure() {
        let contents = "<body>
        <figure>
        <text>Figure 1</text>
        <img src=\"test.png\" />
        <text>Caption</text>
        </figure>
        </body>";
        let result = "Figure 1\n<img src=\"test.png\" />\nCaption".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_par_figure() {
        let contents = "<body>
        <p><text>Here is the following figure:</text></p>
        <figure>
        <text>Figure 1</text>
        <img src=\"test.png\" />
        <text>Caption</text>
        </figure>
        </body>";
        let result = "Here is the following figure:\n\n\
                      Figure 1\n<img src=\"test.png\" />\nCaption".to_string();
        assert_eq!(convert_file(contents), result);
    }

    #[test]
    fn convert_figure_par() {
        let contents = "<body>
        <figure>
        <text>Figure 1</text>
        <img src=\"test.png\" />
        <text>Caption</text>
        </figure>
        <p><text>The above figure</text></p>
        </body>";
        let result = "Figure 1\n<img src=\"test.png\" />\nCaption\n\n\
                      The above figure".to_string();
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
        let result = "List:\n\n- a\n- b\n- c".to_string();
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
        let result = "List:\n\n1) a\n2) b\n3) c".to_string();
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
}
