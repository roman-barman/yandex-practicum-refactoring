use crate::parse::Parser;

/// A parser that returns the result as is
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AsIsParser;
impl Parser for AsIsParser {
    type Dest = String;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((&input[input.len()..], input.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_regular_string() {
        assert_eq!(AsIsParser.parse("hello"), Ok(("", "hello".to_owned())));
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(AsIsParser.parse(""), Ok(("", "".to_owned())));
    }
}
