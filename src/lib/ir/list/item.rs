use ir::TextBlock;
use ir::List;

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
