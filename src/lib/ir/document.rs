use ir::IR;

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
