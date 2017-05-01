use super::List;
use ir::TextBlock;

#[derive(Debug, PartialEq)]
pub enum ListContent {
    Text(TextBlock),
    List(List),
}

