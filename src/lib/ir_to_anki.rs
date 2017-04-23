use group_lines::group_lines;

#[derive(Debug, PartialEq)]
pub enum Text {
    Text(String),
    Sub(String),
    Sup(String),
    Code(String),
}

#[derive(Debug, PartialEq)]
pub enum ListItem {
    Item(Vec<Text>),
    Nested(Vec<Text>, List),
}

#[derive(Debug, PartialEq)]
pub enum List {
    Ordered(Vec<ListItem>),
    Unordered(Vec<ListItem>),
}

#[derive(Debug, PartialEq)]
pub enum IR {
    Img(String),
    Pre(String),
    Par(Vec<Text>),
    List(List),
    Body(Vec<IR>),
}

impl Text {
    fn handle(self) -> String {
        use self::Text::*;
        match self {
            Sub(text) => format!("_{{{}}}", text),
            Sup(text) => format!("^{{{}}}", text),
            Code(code) => format!("`{}`", group_lines(&code)),
            Text(text) => format!("{}", text),
        }
    }
}

impl<'a> Text {
    pub fn as_string(&'a self) -> &'a String {
        use self::Text::*;

        match *self {
            Text(ref x) => x,
            Sub(ref x) => x,
            Sup(ref x) => x,
            Code(ref x) => x,
        }
    }
}

impl ListItem {
    fn handle(self, depth: u8) -> String {
        use self::ListItem::*;

        fn handle_texts(texts: Vec<Text>) -> String {
            texts.into_iter()
                .map(|x| x.handle())
                .fold(String::new(), |acc, x| acc + &x)
        }

        match self {
            Item(texts) => format!("{}", handle_texts(texts)),
            Nested(texts, list) => {
                format!("{}\n{}", handle_texts(texts), list.handle(depth + 1).trim())
            }
        }
    }
}

impl List {
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

    fn handle(self, depth: u8) -> String {
        use self::List::*;

        match self {
            Ordered(items) => {
                items.into_iter()
                    .map(|x| x.handle(depth))
                    .zip((1..)) // Used for the numbers
                    .map(|(item, number)| {
                        format!("{}{}) {}", List::get_indent(depth - 1), number, item)
                    })
                    .fold(String::new(), |acc, x| acc + &x + "\n")
            }
            Unordered(items) => {
                items.into_iter()
                    .map(|x| format!("{} {}", List::get_indent(depth), x.handle(depth)))
                    .fold(String::new(), |acc, x| acc + &x + "\n")
            }
        }
    }
}

impl IR {
    pub fn handle(self) -> String {
        use self::IR::*;
        match self {
            Img(src) => {
                format!("<img src=\"{}\" />\n",
                        src.split("/")
                            .last()
                            .expect("Failed to get the filename for an img tag."))
            }
            Pre(text) => format!("```\n{}\n```", text),
            List(list) => format!("{}", list.handle(1)),
            Body(children) => {
                children.into_iter()
                    .map(|x| x.handle())
                    .fold(String::new(), |acc, x| acc + &x)
                    .trim()
                    .to_string()
            }
            Par(children) => {
                let text = children.into_iter()
                    .map(|x| x.handle())
                    .collect::<String>()
                    .trim()
                    .to_string();
                format!("{}\n\n", group_lines(&text))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_par() {
        let content = IR::Par(vec![Text::Text("Test\ntest".to_string())]);
        assert_eq!(content.handle(), String::from("Test test\n\n"));
    }

    #[test]
    fn convert_par_par() {
        let content = IR::Body(vec![IR::Par(vec![Text::Text("Test".to_string())]),
                                    IR::Par(vec![Text::Text("Test".to_string())])]);
        let result = "Test\n\nTest".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_par_sub() {
        let content = IR::Par(vec![Text::Text("Test".to_string()),
                                   Text::Sub("2".to_string())]);
        let result = String::from("Test_{2}\n\n");
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_par_sup() {
        let content = IR::Par(vec![Text::Text("Test".to_string()),
                                   Text::Sup("2".to_string())]);
        let result = String::from("Test^{2}\n\n");
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_ul() {
        let content = IR::List(List::Unordered(vec![
            ListItem::Item(vec![Text::Text("a".to_string())]),
            ListItem::Item(vec![Text::Text("b".to_string())]),
            ListItem::Item(vec![Text::Text("c".to_string())])
        ]));
        let result = "- a\n- b\n- c\n".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_ol() {
        let content = IR::List(List::Ordered(vec![
            ListItem::Item(vec![Text::Text("a".to_string())]),
            ListItem::Item(vec![Text::Text("b".to_string())]),
            ListItem::Item(vec![Text::Text("c".to_string())])
        ]));
        let result = "1) a\n2) b\n3) c\n".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_nested_ul_ul() {
        let content = IR::List(List::Unordered(vec![
            ListItem::Item(vec![Text::Text("a".to_string())]),
            ListItem::Nested(vec![Text::Text("b".to_string())],
                List::Unordered(vec![
                    ListItem::Item(vec![Text::Text("b1".to_string())]),
                    ListItem::Item(vec![Text::Text("b2".to_string())])
                ])),
            ListItem::Item(vec![Text::Text("c".to_string())])
        ]));
        let result = "- a\n- b\n--- b1\n--- b2\n- c\n".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_nested_ol_ul() {
        let content = IR::List(List::Ordered(vec![
            ListItem::Item(vec![Text::Text("a".to_string())]),
            ListItem::Nested(vec![Text::Text("b".to_string())],
                List::Unordered(vec![
                    ListItem::Item(vec![Text::Text("b1".to_string())]),
                    ListItem::Item(vec![Text::Text("b2".to_string())])
                ])),
            ListItem::Item(vec![Text::Text("c".to_string())])
        ]));
        let result = "1) a\n2) b\n--- b1\n--- b2\n3) c\n".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_nested_ul_ol() {
        let content = IR::List(List::Unordered(vec![
            ListItem::Item(vec![Text::Text("a".to_string())]),
            ListItem::Nested(vec![Text::Text("b".to_string())],
                List::Ordered(vec![
                    ListItem::Item(vec![Text::Text("b1".to_string())]),
                    ListItem::Item(vec![Text::Text("b2".to_string())])
                ])),
            ListItem::Item(vec![Text::Text("c".to_string())])
        ]));
        let result = "- a\n- b\n-1) b1\n-2) b2\n- c\n".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn convert_nested_ol_ol() {
        let content = IR::List(List::Ordered(vec![
            ListItem::Item(vec![Text::Text("a".to_string())]),
            ListItem::Nested(vec![Text::Text("b".to_string())],
                List::Ordered(vec![
                    ListItem::Item(vec![Text::Text("b1".to_string())]),
                    ListItem::Item(vec![Text::Text("b2".to_string())])
                ])),
            ListItem::Item(vec![Text::Text("c".to_string())])
        ]));
        let result = "1) a\n2) b\n-1) b1\n-2) b2\n3) c\n".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn shorten_img_src() {
        let content = IR::Img("a/b/c/d.png".to_string());
        let result = "<img src=\"d.png\" />".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn code() {
        let content = Text::Code("i".to_string());
        let result = "`i`".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn code_multiline() {
        let content = Text::Code("i = 0;\ni++".to_string());
        let result = "`i = 0; i++`".to_string();
        assert_eq!(content.handle(), result);

        let content = Text::Code("i = 0;\n\ni++;\n\n\n\nj = i;\n\n\n".to_string());
        let result = "`i = 0; i++; j = i;`".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn pre() {
        let content = IR::Pre("int i = 0;\ni++;\n\nint j = i".to_string());
        let result = "```\nint i = 0;\ni++;\n\nint j = i\n```".to_string();
        assert_eq!(content.handle(), result);
    }

    #[test]
    fn par_text_text() {
        let content = IR::Par(vec![Text::Text("a ".to_string()),
                                   Text::Text("b".to_string())]);
        let result = "a b\n\n".to_string();
        assert_eq!(content.handle(), result);
    }
}
