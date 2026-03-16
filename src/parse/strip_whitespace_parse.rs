use crate::parse::Parser;

/// A combinator that skips a string without leading spaces.
#[derive(Debug, Clone)]
pub struct StripWhitespaceParser<T> {
    parser: T,
}

impl<T> StripWhitespaceParser<T> {
    pub fn new(parser: T) -> Self {
        StripWhitespaceParser { parser }
    }
}

impl<T: Parser> Parser for StripWhitespaceParser<T> {
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser
            .parse(input.trim_start())
            .map(|(remaining, parsed)| (remaining.trim_start(), parsed))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::std_parse::U32Parser;
    use crate::parse::tag_parse::TagParser;

    #[test]
    fn test_strip_whitespace() {
        assert_eq!(
            StripWhitespaceParser::new(TagParser::new("hello")).parse(" hello world".into()),
            Ok(("world".into(), ()))
        );
        assert_eq!(
            StripWhitespaceParser::new(TagParser::new("hello")).parse("hello".into()),
            Ok(("".into(), ()))
        );
        assert_eq!(
            StripWhitespaceParser::new(U32Parser).parse(" 42 answer".into()),
            Ok(("answer".into(), 42))
        );
    }
}
