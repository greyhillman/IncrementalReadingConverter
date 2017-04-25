//use group_lines::group_lines;

//extern crate itertools;
//use self::itertools::join;

use ir::*;

fn convert_text(text: Text) -> String {
    match text {
        Text::Text(x) => x,
        Text::Sup(x) => format!("^{{{}}}", x),
        Text::Sub(x) => format!("_{{{}}}", x),
        Text::Code(x) => format!("`{}`", x),
    }
}

fn convert_textblock(text: TextBlock) -> String {
    text.into_iter()
        .map(|x| convert_text(x))
        .collect()
}

fn convert_list(list: List) -> String {
    fn convert_listitem(item: ListItem) -> String {
        String::new()
    }
    String::new()
}

fn convert_table(table: Table) -> String {
    String::new()
}

fn convert_ir(x: IR) -> String {
    match x {
        IR::Img(src) => format!("<img src=\"{}\" />\n", src),
        IR::Pre(content) => format!("```{}```\n\n", content),
        IR::Par(text) => format!("{}\n\n", convert_textblock(text)),
        IR::List(list) => format!("{}\n\n", convert_list(list)),
        IR::Table(table) => format!("{}\n\n", convert_table(table)),
    }
}

pub fn convert(document: Document) -> String {
    document.into_iter()
        .map(|x| convert_ir(x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let doc = Document::new();
        let result = "".to_string();
        assert_eq!(convert(doc), result);
    }

    #[test]
    fn par() {
        let par = IR::par(TextBlock::new()
                          .add(Text::text("a")));
        let result = "a\n\n".to_string();
        assert_eq!(convert_ir(par), result);
    }
}
