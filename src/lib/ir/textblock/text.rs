use super::TextBlock;

#[derive(Debug, PartialEq, Clone)]
pub enum Text {
    Text(String),
    Sub(TextBlock),
    Sup(TextBlock),
    Code(String),
}

impl Text {
    pub fn text(text: &str) -> Self {
        Text::Text(text.to_string())
    }

    pub fn sub(text: &str) -> Self {
        Text::Sub(TextBlock::from(Text::Text(text.to_string())))
    }

    pub fn sup(text: &str) -> Self {
        Text::Sup(TextBlock::from(Text::Text(text.to_string())))
    }

    pub fn code(code: &str) -> Self {
        Text::Code(code.to_string())
    }
}

