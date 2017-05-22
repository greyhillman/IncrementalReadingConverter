use ir::IR;

use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Document {
    children: Vec<IR>,
}

impl Document {
    pub fn new() -> Self {
        Document { children: vec![] }
    }

    pub fn add(self, item: IR) -> Self {
        let mut children = self.children;
        children.push(item);

        Document { children: children }
    }
}

impl IntoIterator for Document {
    type Item = IR;
    type IntoIter = ::std::vec::IntoIter<IR>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl FromIterator<IR> for Document {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = IR>
    {
        iter.into_iter()
            .fold(Document::new(), |doc, child| doc.add(child))
    }
}
