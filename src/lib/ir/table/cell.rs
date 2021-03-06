use ir::TextBlock;

#[derive(Debug, PartialEq, Clone)]
pub struct TableCell(TextBlock);

impl TableCell {
    pub fn new(text: TextBlock) -> Self {
        TableCell(text)
    }

    pub fn text(self) -> TextBlock {
        self.0
    }
}
