use super::ListContent;
use super::List;
use ir::TextBlock;

#[derive(Debug, PartialEq)]
pub struct ListItem {
    content: Vec<ListContent>,
}

impl ListItem {
    pub fn new() -> Self {
        ListItem {
            content: vec![]
        }
    }

    pub fn add(self, c: ListContent) -> Self {
        let mut content = self.content;
        content.push(c);

        ListItem {
            content,
            .. self
        }
    }

    pub fn item(text: TextBlock) -> Self {
        ListItem::new()
            .add(ListContent::Text(text))
    }

    pub fn item_nested_list(text: TextBlock, list: List) -> Self {
        ListItem::item(text)
            .add(ListContent::List(list))
    }
}

impl IntoIterator for ListItem {
    type Item = ListContent;
    type IntoIter = ::std::vec::IntoIter<ListContent>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
