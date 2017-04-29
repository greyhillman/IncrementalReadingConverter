//use group_lines::group_lines;

extern crate itertools;
use self::itertools::join;

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

fn convert_list(depth: usize, list: List) -> String {
    fn get_indent(depth: usize) -> String {
        match depth {
            0 => "".to_string(),
            1 => "-".to_string(),
            x => format!("{}--", get_indent(x - 1)),
        }
    }
    fn convert_listitem(depth: usize, item: ListItem) -> String {
        match item {
            ListItem::Item(text) => convert_textblock(text),
            ListItem::Nested(text, list) => {
                format!("{}\n{}",
                        convert_textblock(text),
                        convert_list(depth + 1, list))
            }
        }
    }

    match *list.style() {
        ListType::Ordered => {
            let items = list.into_iter()
                .map(|item| convert_listitem(depth, item))
                .zip(1..)
                .map(|(item, num)| format!("{}{}) {}", get_indent(depth - 1), num, item));

            join(items, "\n")
        }
        ListType::Unordered => {
            let items = list.into_iter()
                .map(|item| convert_listitem(depth, item))
                .map(|item| format!("{} {}", get_indent(depth), item));

            join(items, "\n")
        }
    }
}

fn convert_table(mut table: Table) -> String {
    fn convert_row(row: TableRow) -> String {
        let row = row.into_iter()
            .map(|cell| convert_textblock(cell.text()));

        join(row, " | ")
    }

    let header = table.header();
    let footer = table.footer();

    let body = table.body()
        .into_iter()
        .map(convert_row);
    let body = join(body, "\n");

    match (header, footer) {
        (Some(h), Some(f)) => {
            format!("{}\n-----\n{}\n-----\n{}",
                    convert_row(h),
                    body,
                    convert_row(f))
        }
        (None, Some(f)) => format!("{}\n-----\n{}", body, convert_row(f)),
        (Some(h), None) => format!("{}\n-----\n{}", convert_row(h), body),
        (None, None) => format!("{}", body),
    }
}

fn convert_ir(x: IR) -> String {
    match x {
        IR::Img(src) => format!("<img src=\"{}\" />\n", src),
        IR::Pre(content) => format!("```{}```\n\n", content),
        IR::Par(text) => format!("{}\n\n", convert_textblock(text)),
        IR::List(list) => format!("{}\n\n", convert_list(1, list)),
        IR::Table(table) => format!("{}\n\n", convert_table(table)),
    }
}

pub fn convert(document: Document) -> String {
    document.into_iter()
        .map(|x| convert_ir(x))
        .collect::<String>()
        .trim()
        .to_string()
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
        let par = IR::par(TextBlock::new().add(Text::text("a")));
        let result = "a\n\n".to_string();
        assert_eq!(convert_ir(par), result);
    }

    #[test]
    fn ordered_list_single_item() {
        let text = TextBlock::new().add(Text::text("a"));
        let item = ListItem::item(text);
        let list = IR::list(List::new(ListType::Ordered).add(item));
        let result = "1) a\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn ordered_list_multiple_item() {
        let text1 = TextBlock::new().add(Text::text("a"));
        let text2 = TextBlock::new().add(Text::text("a"));
        let item1 = ListItem::item(text1);
        let item2 = ListItem::item(text2);
        let list = IR::list(List::new(ListType::Ordered)
            .add(item1)
            .add(item2));
        let result = "1) a\n2) a\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn unordered_list_single_item() {
        let text = TextBlock::new().add(Text::text("a"));
        let item = ListItem::item(text);
        let list = IR::list(List::new(ListType::Unordered).add(item));
        let result = "- a\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn unordered_list_multiple_item() {
        let text1 = TextBlock::new().add(Text::text("a"));
        let text2 = TextBlock::new().add(Text::text("a"));
        let item1 = ListItem::item(text1);
        let item2 = ListItem::item(text2);
        let list = IR::list(List::new(ListType::Unordered)
            .add(item1)
            .add(item2));
        let result = "- a\n- a\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn ordered_list_nested_ordered_list() {
        let text = TextBlock::new().add(Text::text("child"));
        let item = ListItem::item(text);
        let nested_list = List::new(ListType::Ordered).add(item);

        let text = TextBlock::new().add(Text::text("parent"));
        let item = ListItem::item_nested_list(text, nested_list);
        let list = IR::list(List::new(ListType::Ordered).add(item));
        let result = "1) parent\n-1) child\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn ordered_list_nested_unordered_list() {
        let text = TextBlock::new().add(Text::text("child"));
        let item = ListItem::item(text);
        let nested_list = List::new(ListType::Unordered).add(item);

        let text = TextBlock::new().add(Text::text("parent"));
        let item = ListItem::item_nested_list(text, nested_list);
        let list = IR::list(List::new(ListType::Ordered).add(item));
        let result = "1) parent\n--- child\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn unordered_list_nested_ordered_list() {
        let text = TextBlock::new().add(Text::text("child"));
        let item = ListItem::item(text);
        let nested_list = List::new(ListType::Ordered).add(item);

        let text = TextBlock::new().add(Text::text("parent"));
        let item = ListItem::item_nested_list(text, nested_list);
        let list = IR::list(List::new(ListType::Unordered).add(item));
        let result = "- parent\n-1) child\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn unordered_list_nested_unordered_list() {
        let text = TextBlock::new().add(Text::text("child"));
        let item = ListItem::item(text);
        let nested_list = List::new(ListType::Unordered).add(item);

        let text = TextBlock::new().add(Text::text("parent"));
        let item = ListItem::item_nested_list(text, nested_list);
        let list = IR::list(List::new(ListType::Unordered).add(item));
        let result = "- parent\n--- child\n\n".to_string();
        assert_eq!(convert_ir(list), result);
    }

    #[test]
    fn table() {
        let cell1 = TableCell::new(TextBlock::new().add(Text::text("a")));
        let cell2 = TableCell::new(TextBlock::new().add(Text::text("b")));
        let header = TableRow::new()
            .add(cell1)
            .add(cell2);
        let cell1 = TableCell::new(TextBlock::new().add(Text::text("a")));
        let cell2 = TableCell::new(TextBlock::new().add(Text::text("b")));
        let row = TableRow::new()
            .add(cell1)
            .add(cell2);
        let cell1 = TableCell::new(TextBlock::new().add(Text::text("a")));
        let cell2 = TableCell::new(TextBlock::new().add(Text::text("b")));
        let footer = TableRow::new()
            .add(cell1)
            .add(cell2);

        let table = IR::table(Table::new()
            .set_header(header)
            .add(row)
            .set_footer(footer));
        let result = "\
                      a | b\n-----\na | b\n-----\na | b\n\n"
            .to_string();
        assert_eq!(convert_ir(table), result);
    }
}
