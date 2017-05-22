mod item;
pub use self::item::ListItem;

mod style;
pub use self::style::ListType;

mod content;
pub use self::content::ListContent;

#[derive(Debug, PartialEq, Clone)]
pub struct List {
    style: ListType,
    items: Vec<ListItem>,
}

impl List {
    pub fn new(style: ListType) -> Self {
        List {
            style: style,
            items: vec![],
        }
    }

    pub fn style(&self) -> &ListType {
        &self.style
    }

    pub fn add(&mut self, item: ListItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl IntoIterator for List {
    type Item = ListItem;
    type IntoIter = ::std::vec::IntoIter<ListItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
