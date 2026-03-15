use crate::parse::Parser;

/// A mapping combinator.
/// Parses with a child parser, transforming the result as the caller desires.
#[derive(Debug, Clone)]
pub struct MapParser<T, M> {
    parser: T,
    map: M,
}

impl<T: Parser, Dest: Sized, M: Fn(T::Dest) -> Dest> MapParser<T, M> {
    pub fn new(parser: T, map: M) -> MapParser<T, M> {
        MapParser { parser, map }
    }
}

impl<T: Parser, Dest: Sized, M: Fn(T::Dest) -> Dest> Parser for MapParser<T, M> {
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser
            .parse(input)
            .map(|(remaining, pre_result)| (remaining, (self.map)(pre_result)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::as_is_parse::AsIsParser;
    use crate::parse::tag_parse::tag;

    #[test]
    fn map_transforms_result() {
        let parser = MapParser::new(AsIsParser, |s: String| s.len());
        assert_eq!(parser.parse("hello"), Ok(("", 5)));
    }

    #[test]
    fn map_preserves_remaining_input() {
        let parser = MapParser::new(tag("key="), |_| 42u32);
        assert_eq!(parser.parse("key=value"), Ok(("value", 42)));
    }

    #[test]
    fn map_propagates_child_error() {
        let parser = MapParser::new(tag("key="), |_| 42u32);
        assert_eq!(parser.parse("no_match"), Err(()));
    }
}
