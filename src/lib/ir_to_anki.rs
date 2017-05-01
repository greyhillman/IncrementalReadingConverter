//use group_lines::group_lines;

extern crate itertools;
use self::itertools::join;

use ir::*;

pub trait ToAnki {
    fn to_anki(self) -> String;
}

pub trait ToAnkiWithDepth {
    fn to_anki(self, usize) -> String;
}

impl ToAnki for Text {
    fn to_anki(self) -> String {
        match self {
            Text::Text(x) => x,
            Text::Code(x) => format!("`{}`", x),
            Text::Sub(block) => format!("_{{{}}}", block.to_anki()),
            Text::Sup(block) => format!("^{{{}}}", block.to_anki()),
        }
    }
}

impl ToAnki for TextBlock {
    fn to_anki(self) -> String {
        self.into_iter()
            .map(|child| child.to_anki())
            .collect()
    }
}

impl ToAnkiWithDepth for ListContent {
    fn to_anki(self, depth: usize) -> String {
        match self {
            ListContent::Text(x) => x.to_anki(),
            ListContent::List(x) => x.to_anki(depth),
        }
    }
}

impl ToAnkiWithDepth for ListItem {
    fn to_anki(self, depth: usize) -> String {
        let content = self.into_iter()
            .map(|content| content.to_anki(depth + 1));

        join(content, "\n")
    }
}

impl ToAnkiWithDepth for List {
    fn to_anki(self, depth: usize) -> String {
        match *self.style() {
            ListType::Ordered => {
                let indent = String::from("--").repeat(depth - 1);

                let items = self.into_iter()
                    .map(|item| item.to_anki(depth))
                    .zip(1..)
                    .map(|(item, num)| format!("{}{}) {}", indent, num, item));

                join(items, "\n")
            }
            ListType::Unordered => {
                let indent = String::from("--").repeat(depth);

                let items = self.into_iter()
                    .map(|item| item.to_anki(depth))
                    .map(|item| format!("{} {}", indent, item));

                join(items, "\n")
            }
        }
    }
}

impl ToAnki for TableCell {
    fn to_anki(self) -> String {
        self.text().to_anki()
    }
}

impl ToAnki for TableRow {
    fn to_anki(self) -> String {
        let cols = self.into_iter()
            .map(|cell| cell.to_anki());

        join(cols, " | ")
    }
}

impl ToAnki for Table {
    fn to_anki(mut self) -> String {
        let header = self.header();
        let footer = self.footer();

        let body = self.body()
            .into_iter()
            .map(|row| row.to_anki());
        let body = join(body, "\n");

        match (header, footer) {
            (Some(h), Some(f)) => {
                format!("{}\n-----\n{}\n-----\n{}",
                        h.to_anki(),
                        body,
                        f.to_anki())
            }
            (None, Some(f)) => format!("{}\n-----\n{}", body, f.to_anki()),
            (Some(h), None) => format!("{}\n-----\n{}", h.to_anki(), body),
            (None, None) => format!("{}", body),
        }
    }
}

impl ToAnki for IR {
    fn to_anki(self) -> String {
        match self {
            IR::Img(src) => format!("<img src=\"{}\" />\n", src),
            IR::Pre(content) => format!("```{}```\n\n", content),
            IR::Par(text) => format!("{}\n\n", text.to_anki()),
            IR::List(list) => format!("{}\n\n", list.to_anki(1)),
            IR::Table(table) => format!("{}\n\n", table.to_anki()),
        }
    }
}

pub fn convert(document: Document) -> String {
    document.into_iter()
        .map(|x| x.to_anki())
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
