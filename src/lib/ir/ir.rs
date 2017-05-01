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
    pub fn img(src: &str) -> Self {
        IR::Img(src.to_string())
    }

    pub fn pre(text: &str) -> Self {
        IR::Pre(text.to_string())
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
