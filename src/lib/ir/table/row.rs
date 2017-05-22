use ir::TableCell;

#[derive(Debug, PartialEq, Clone)]
pub struct TableRow {
    columns: Vec<TableCell>,
}

impl TableRow {
    pub fn new() -> Self {
        TableRow { columns: vec![] }
    }

    pub fn add(&mut self, cell: TableCell) -> &mut Self {
        self.columns.push(cell);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl IntoIterator for TableRow {
    type Item = TableCell;
    type IntoIter = ::std::vec::IntoIter<TableCell>;

    fn into_iter(self) -> Self::IntoIter {
        self.columns.into_iter()
    }
}
