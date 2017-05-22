mod cell;
pub use self::cell::TableCell;

mod row;
pub use self::row::TableRow;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn set_header(&mut self, header: TableRow) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn set_footer(&mut self, footer: TableRow) -> &mut Self {
        self.footer = Some(footer);
        self
    }

    pub fn add(&mut self, row: TableRow) -> &mut Self {
        self.body.push(row);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
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
