#[derive(Debug, PartialEq)]
pub enum Text {
    Text(String),
    Sub(String),
    Sup(String),
    Code(String),
}

impl Text {
    pub fn text(text: &str) -> Self {
        Text::Text(text.to_string())
    }

    pub fn sub(text: &str) -> Self {
        Text::Sub(text.to_string())
    }

    pub fn sup(text: &str) -> Self {
        Text::Sup(text.to_string())
    }

    pub fn code(code: &str) -> Self {
        Text::Code(code.to_string())
    }
}
