use crate::parse::Parser;

/// A combinator with a discardable prefix,
/// a simplified version of Delimited (analogous to preceded from nom)
#[derive(Debug, Clone)]
pub struct PrecededParser<Prefix, T> {
    prefix_to_ignore: Prefix,
    dest_parser: T,
}

impl<Prefix, T> PrecededParser<Prefix, T>
where
    Prefix: Parser,
    T: Parser,
{
    pub fn new(prefix_to_ignore: Prefix, dest_parser: T) -> PrecededParser<Prefix, T> {
        PrecededParser {
            prefix_to_ignore,
            dest_parser,
        }
    }
}

impl<Prefix, T> Parser for PrecededParser<Prefix, T>
where
    Prefix: Parser,
    T: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, _) = self.prefix_to_ignore.parse(input)?;
        self.dest_parser.parse(remaining)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::std_parse::U32Parser;
    use crate::parse::tag_parse::TagParser;

    #[test]
    fn test_preceded_success() {
        assert_eq!(
            PrecededParser::new(TagParser::new("value="), U32Parser).parse("value=42 rest"),
            Ok((" rest", 42))
        );
    }

    #[test]
    fn test_preceded_consumes_fully() {
        assert_eq!(
            PrecededParser::new(TagParser::new("a"), U32Parser).parse("a1"),
            Ok(("", 1))
        );
    }

    #[test]
    fn test_preceded_prefix_mismatch() {
        assert_eq!(
            PrecededParser::new(TagParser::new("key="), U32Parser).parse("val=42"),
            Err(())
        );
    }

    #[test]
    fn test_preceded_dest_parser_fails() {
        assert_eq!(
            PrecededParser::new(TagParser::new("key="), U32Parser).parse("key=not_a_number"),
            Err(())
        );
    }

    #[test]
    fn test_preceded_empty_input() {
        assert_eq!(
            PrecededParser::new(TagParser::new("key="), U32Parser).parse(""),
            Err(())
        );
    }

    #[test]
    fn test_preceded_only_prefix() {
        assert_eq!(
            PrecededParser::new(TagParser::new("key="), U32Parser).parse("key="),
            Err(())
        );
    }
}
