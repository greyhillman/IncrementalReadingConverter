use super::ListContent;
use super::List;
use ir::TextBlock;

#[derive(Debug, PartialEq, Clone)]
pub struct ListItem {
    content: Vec<ListContent>,
}

impl ListItem {
    pub fn new() -> Self {
        ListItem { content: vec![] }
    }

    pub fn add(&mut self, content: ListContent) -> &mut Self {
        self.content.push(content);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }

    pub fn item(text: TextBlock) -> Self {
        ListItem::new()
            .add(ListContent::Text(text))
            .build()
    }

    pub fn item_nested_list(text: TextBlock, list: List) -> Self {
        ListItem::new()
            .add(ListContent::Text(text))
            .add(ListContent::List(list))
            .build()
    }
}

impl IntoIterator for ListItem {
    type Item = ListContent;
    type IntoIter = ::std::vec::IntoIter<ListContent>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
