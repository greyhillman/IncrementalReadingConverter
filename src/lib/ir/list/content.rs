use super::List;
use ir::TextBlock;

#[derive(Debug, PartialEq)]
pub enum ListContent {
    Text(TextBlock),
    List(List),
}

impl From<TextBlock> for ListContent {
    fn from(text: TextBlock) -> Self {
        ListContent::Text(text)
    }
}

impl From<List> for ListContent {
    fn from(list: List) -> Self {
        ListContent::List(list)
    }
}
