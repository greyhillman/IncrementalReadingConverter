#[derive(Debug, PartialEq)]
pub enum Text {
    Text(String),
    Sub(String),
    Sup(String),
    Code(String),
}

impl Text {
    pub fn text(text: &str) -> Self {
        Text::Text(text.to_string())
    }

    pub fn sub(text: &str) -> Self {
        Text::Sub(text.to_string())
    }

    pub fn sup(text: &str) -> Self {
        Text::Sup(text.to_string())
    }

    pub fn code(code: &str) -> Self {
        Text::Code(code.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct TextBlock(Vec<Text>);

impl TextBlock {
    pub fn new() -> Self {
        TextBlock(vec![])
    }

    pub fn add(self, text: Text) -> Self {
        let TextBlock(mut content) = self;

        use self::Text::*;

        match (content.pop(), text) {
            (Some(Text(x)), Text(y)) => content.push(Text(x + &y)),
            (Some(x), y) => {
                content.push(x);
                content.push(y);
            }
            (None, x) => content.push(x),
        }

        TextBlock(content)
    }
}

impl IntoIterator for TextBlock {
    type Item = Text;
    type IntoIter = ::std::vec::IntoIter<Text>;

    fn into_iter(self) -> Self::IntoIter {
        let TextBlock(content) = self;
        content.into_iter()
    }
}

#[derive(Debug, PartialEq)]
pub enum ListItem {
    Item(TextBlock),
    Nested(TextBlock, List),
}

impl ListItem {
    pub fn item(text: TextBlock) -> Self {
        ListItem::Item(text)
    }

    pub fn item_nested_list(text: TextBlock, list: List) -> Self {
        ListItem::Nested(text, list)
    }
}

#[derive(Debug, PartialEq)]
pub struct List {
    is_ordered: bool,
    items: Vec<ListItem>,
}

impl List {
    pub fn new() -> Self {
        List {
            is_ordered: false,
            items: vec![],
        }
    }

    pub fn ordered(self) -> Self {
        List {
            is_ordered: true,
            .. self
        }
    }

    pub fn unordered(self) -> Self {
        List {
            is_ordered: false,
            .. self
        }
    }

    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    pub fn add(self, item: ListItem) -> Self {
        let mut items = self.items;
        items.push(item);

        List {
            items: items,
            .. self
        }
    }
}

impl IntoIterator for List {
    type Item = ListItem;
    type IntoIter = ::std::vec::IntoIter<ListItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct TableCell(TextBlock);

impl TableCell {
    pub fn new(text: TextBlock) -> Self {
        TableCell(text)
    }

    pub fn text(self) -> TextBlock {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct TableRow {
    columns: Vec<TableCell>,
}

impl TableRow {
    pub fn new() -> Self {
        TableRow {
            columns: vec![],
        }
    }

    pub fn add(self, cell: TableCell) -> Self {
        let mut columns = self.columns;
        columns.push(cell);

        TableRow {
            columns: columns,
            .. self
        }
    }
}

impl IntoIterator for TableRow {
    type Item = TableCell;
    type IntoIter = ::std::vec::IntoIter<TableCell>;

    fn into_iter(self) -> Self::IntoIter {
        self.columns.into_iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct Table {
    header: Option<TableRow>,
    body: Vec<TableRow>,
    footer: Option<TableRow>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            header: None,
            body: vec![],
            footer: None,
        }
    }

    pub fn set_header(self, header: TableRow) -> Self {
        Table {
            header: Some(header),
            .. self
        }
    }

    pub fn set_footer(self, footer: TableRow) -> Self {
        Table {
            footer: Some(footer),
            .. self
        }
    }

    pub fn add(self, row: TableRow) -> Self {
        let mut body = self.body;
        body.push(row);

        Table {
            body: body,
            .. self
        }
    }

    pub fn header(&mut self) -> Option<TableRow> {
        self.header.take()
    }

    pub fn footer(&mut self) -> Option<TableRow> {
        self.footer.take()
    }

    pub fn body(self) -> Vec<TableRow> {
        self.body
    }
}

#[derive(Debug, PartialEq)]
pub enum IR {
    Img(String),
    Pre(String),
    Par(TextBlock),
    List(List),
    Table(Table),
}

impl IR {
    pub fn img(src: String) -> Self {
        IR::Img(src)
    }

    pub fn pre(text: String) -> Self {
        IR::Pre(text)
    }

    pub fn par(text: TextBlock) -> Self {
        IR::Par(text)
    }

    pub fn list(list: List) -> Self {
        IR::List(list)
    }

    pub fn table(table: Table) -> Self {
        IR::Table(table)
    }
}

pub struct Document {
    children: Vec<IR>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            children: vec![],
        }
    }

    pub fn add(self, item: IR) -> Self {
        let mut children = self.children;
        children.push(item);

        Document {
            children: children
        }
    }
}

impl IntoIterator for Document {
    type Item = IR;
    type IntoIter = ::std::vec::IntoIter<IR>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textblock() {
        let block = TextBlock::new()
            .add(Text::text("a "))
            .add(Text::text("b"));
        let result = TextBlock::new()
            .add(Text::text("a b"));
        assert_eq!(block, result);
    }

    #[test]
    fn textblock_separated_text() {
        let block = TextBlock::new()
            .add(Text::text("a "))
            .add(Text::text("b"))
            .add(Text::code("i = 1"))
            .add(Text::text("c "))
            .add(Text::text("d"));
        let result = TextBlock::new()
            .add(Text::text("a b"))
            .add(Text::code("i = 1"))
            .add(Text::text("c d"));
        assert_eq!(block, result);
    }
}

