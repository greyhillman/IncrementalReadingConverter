use ir::TextBlock;
use ir::List;
use ir::Table;

#[derive(Debug, PartialEq)]
pub enum IR {
    Img(String),
    Pre(String),
    Par(TextBlock),
    List(List),
    Table(Table),
}

impl IR {
    pub fn img(src: String) -> Self {
        IR::Img(src)
    }

    pub fn pre(text: String) -> Self {
        IR::Pre(text)
    }

    pub fn par(text: TextBlock) -> Self {
        IR::Par(text)
    }

    pub fn list(list: List) -> Self {
        IR::List(list)
    }

    pub fn table(table: Table) -> Self {
        IR::Table(table)
    }
}
