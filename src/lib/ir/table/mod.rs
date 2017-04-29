mod cell;
pub use self::cell::TableCell;

mod row;
pub use self::row::TableRow;

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
        Table { header: Some(header), ..self }
    }

    pub fn set_footer(self, footer: TableRow) -> Self {
        Table { footer: Some(footer), ..self }
    }

    pub fn add(self, row: TableRow) -> Self {
        let mut body = self.body;
        body.push(row);

        Table { body: body, ..self }
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
