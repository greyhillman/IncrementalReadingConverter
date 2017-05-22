use super::List;
use ir::TextBlock;

#[derive(Debug, PartialEq, Clone)]
pub enum ListContent {
    Text(TextBlock),
    List(List),
}

impl From<TextBlock> for ListContent {
    fn from(text: TextBlock) -> Self {
        ListContent::Text(text)
    }
}

impl From<String> for ListContent {
    fn from(text: String) -> Self {
        ListContent::Text(TextBlock::from(text))
    }
}

impl<'a> From<&'a str> for ListContent {
    fn from(text: &str) -> Self {
        ListContent::Text(TextBlock::from(text))
    }
}

impl From<List> for ListContent {
    fn from(list: List) -> Self {
        ListContent::List(list)
    }
}
