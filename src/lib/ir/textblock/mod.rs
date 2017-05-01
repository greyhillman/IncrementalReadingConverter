mod text;
pub use self::text::Text;

#[derive(Debug, PartialEq)]
pub struct TextBlock(Vec<Text>);

impl TextBlock {
    pub fn new() -> Self {
        TextBlock(vec![])
    }

    pub fn add(self, text: Text) -> Self {
        let TextBlock(mut content) = self;

        match (content.pop(), text) {
            (Some(Text::Text(x)), Text::Text(y)) => {
                content.push(Text::Text(x + &y));
            }
            (Some(x), y) => {
                content.push(x);
                content.push(y);
            }
            (None, x) => {
                content.push(x);
            }
        }

        TextBlock(content)
    }
}

impl IntoIterator for TextBlock {
    type Item = Text;
    type IntoIter = ::std::vec::IntoIter<Text>;

    fn into_iter(self) -> Self::IntoIter {
        let TextBlock(content) = self;

        content.into_iter()
    }
}

impl From<Text> for TextBlock {
    fn from(text: Text) -> Self {
        TextBlock::new()
            .add(text)
    }
}

impl From<String> for TextBlock {
    fn from(text: String) -> Self {
        TextBlock::from(Text::text(&text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_text() {
        let block = TextBlock::new()
            .add(Text::text("a "))
            .add(Text::text("b"));
        let result = TextBlock::new().add(Text::text("a b"));
        assert_eq!(block, result);
    }

    #[test]
    fn separated_text() {
        let block = TextBlock::new()
            .add(Text::text("a "))
            .add(Text::text("b"))
            .add(Text::code("i = 1"))
            .add(Text::text("c "))
            .add(Text::text("d"));
        let result = TextBlock::new()
            .add(Text::text("a b"))
            .add(Text::code("i = 1"))
            .add(Text::text("c d"));
        assert_eq!(block, result);
    }
}
