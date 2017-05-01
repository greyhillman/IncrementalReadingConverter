use super::ListContent;

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
}

impl IntoIterator for ListItem {
    type Item = ListContent;
    type IntoIter = ::std::vec::IntoIter<ListContent>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
