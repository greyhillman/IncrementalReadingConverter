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
    Header(usize, String),
}

impl IR {
    pub fn img(src: &str) -> Self {
        IR::Img(src.to_string())
    }

    pub fn pre(text: &str) -> Self {
        IR::Pre(text.to_string())
    }

    pub fn header(level: usize, text: &str) -> Self {
        IR::Header(level, text.to_string())
    }
}

impl From<TextBlock> for IR {
    fn from(text: TextBlock) -> Self {
        IR::Par(text)
    }
}

impl From<List> for IR {
    fn from(list: List) -> Self {
        IR::List(list)
    }
}

impl From<Table> for IR {
    fn from(table: Table) -> Self {
        IR::Table(table)
    }
}
