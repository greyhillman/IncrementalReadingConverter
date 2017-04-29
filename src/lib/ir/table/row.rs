use ir::TableCell;

#[derive(Debug, PartialEq)]
pub struct TableRow {
    columns: Vec<TableCell>,
}

impl TableRow {
    pub fn new() -> Self {
        TableRow { columns: vec![] }
    }

    pub fn add(self, cell: TableCell) -> Self {
        let mut columns = self.columns;
        columns.push(cell);

        TableRow { columns: columns, ..self }
    }
}

impl IntoIterator for TableRow {
    type Item = TableCell;
    type IntoIter = ::std::vec::IntoIter<TableCell>;

    fn into_iter(self) -> Self::IntoIter {
        self.columns.into_iter()
    }
}
