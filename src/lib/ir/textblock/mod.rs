mod text;
pub use self::text::Text;

#[derive(Debug, PartialEq, Clone)]
pub struct TextBlock {
    content: Vec<Text>,
}

impl TextBlock {
    pub fn new() -> Self {
        TextBlock { content: vec![] }
    }

    pub fn add(&mut self, text: Text) -> &mut Self {
        match (self.content.pop(), text) {
            (Some(Text::Text(x)), Text::Text(y)) => {
                self.content.push(Text::Text(x + &y));
            }
            (Some(x), y) => {
                self.content.push(x);
                self.content.push(y);
            }
            (None, x) => {
                self.content.push(x);
            }
        }

        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

impl IntoIterator for TextBlock {
    type Item = Text;
    type IntoIter = ::std::vec::IntoIter<Text>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

impl From<Text> for TextBlock {
    fn from(text: Text) -> Self {
        TextBlock::new()
            .add(text)
            .build()
    }
}

impl From<String> for TextBlock {
    fn from(text: String) -> Self {
        TextBlock::from(Text::text(&text))
    }
}

impl<'a> From<&'a str> for TextBlock {
    fn from(text: &str) -> Self {
        TextBlock::from(Text::text(text))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_text() {
        let block = TextBlock::new()
            .add(Text::text("a "))
            .add(Text::text("b"))
            .build();
        let result = TextBlock::from("a b");
        assert_eq!(block, result);
    }

    #[test]
    fn separated_text() {
        let block = TextBlock::new()
            .add(Text::text("a "))
            .add(Text::text("b"))
            .add(Text::code("i = 1"))
            .add(Text::text("c "))
            .add(Text::text("d"))
            .build();
        let result = TextBlock::new()
            .add(Text::text("a b"))
            .add(Text::code("i = 1"))
            .add(Text::text("c d"))
            .build();
        assert_eq!(block, result);
    }
}
