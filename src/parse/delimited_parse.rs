use crate::parse::Parser;

/// A combinator for parsing a string,
/// surrounded at the beginning and end by something mandatory that doesn't contribute to the result.
/// It passes the string to parser1, the remaining string after the first parse to parser2,
/// and the remaining string after the second parse to parser3.
/// The result of parser2 will be the result of this combinator,
/// and the remaining string will be the string remaining after parser3.
/// (Similar to delimited in nom)
#[derive(Debug, Clone)]
pub struct DelimitedParser<Prefix, T, Suffix> {
    prefix_to_ignore: Prefix,
    dest_parser: T,
    suffix_to_ignore: Suffix,
}

impl<Prefix, T, Suffix> DelimitedParser<Prefix, T, Suffix>
where
    Prefix: Parser,
    T: Parser,
    Suffix: Parser,
{
    pub fn new(prefix_to_ignore: Prefix, dest_parser: T, suffix_to_ignore: Suffix) -> Self {
        Self {
            prefix_to_ignore,
            dest_parser,
            suffix_to_ignore,
        }
    }
}

impl<Prefix, T, Suffix> Parser for DelimitedParser<Prefix, T, Suffix>
where
    Prefix: Parser,
    T: Parser,
    Suffix: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, _) = self.prefix_to_ignore.parse(input)?;
        let (remaining, result) = self.dest_parser.parse(remaining)?;
        self.suffix_to_ignore
            .parse(remaining)
            .map(|(remaining, _)| (remaining, result))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::std_parse::U32Parser;
    use crate::parse::tag_parse::tag;

    #[test]
    fn test_delimited() {
        assert_eq!(
            DelimitedParser::new(tag("["), U32Parser, tag("]")).parse("[0x32]".into()),
            Ok(("".into(), 0x32))
        );
        assert_eq!(
            DelimitedParser::new(tag("[".into()), U32Parser, tag("]".into()))
                .parse("[0x32] nice".into()),
            Ok((" nice".into(), 0x32))
        );
        assert_eq!(
            DelimitedParser::new(tag("[".into()), U32Parser, tag("]")).parse("0x32]".into()),
            Err(())
        );
        assert_eq!(
            DelimitedParser::new(tag("[".into()), U32Parser, tag("]")).parse("[0x32".into()),
            Err(())
        );
    }
}
