use crate::parse::Parser;
use crate::parse::all_parse::AllConditionParser;
use crate::parse::delimited_parse::DelimitedParser;
use crate::parse::quoted_tag_parse::QuotedTagParser;
use crate::parse::strip_whitespace_parse::StripWhitespaceParser;
use crate::parse::tag_parse::TagParser;

/// A combinator that extracts values from a key:value pair.
/// For ease of implementation, a comma is always required at the end of a key-value pair;
/// a simple 'key:value' will not be read.
#[derive(Debug, Clone)]
pub struct KeyValueParser<T> {
    parser: DelimitedParser<
        AllConditionParser<(
            StripWhitespaceParser<QuotedTagParser>,
            StripWhitespaceParser<TagParser>,
        )>,
        StripWhitespaceParser<T>,
        StripWhitespaceParser<TagParser>,
    >,
}

impl<T> KeyValueParser<T>
where
    T: Parser,
{
    pub fn new(key: &'static str, value_parser: T) -> KeyValueParser<T> {
        KeyValueParser {
            parser: DelimitedParser::new(
                AllConditionParser::<(
                    StripWhitespaceParser<QuotedTagParser>,
                    StripWhitespaceParser<TagParser>,
                )>::new(
                    StripWhitespaceParser::new(QuotedTagParser::new(key)),
                    StripWhitespaceParser::new(TagParser::new(":")),
                ),
                StripWhitespaceParser::new(value_parser),
                StripWhitespaceParser::new(TagParser::new(",")),
            ),
        }
    }
}

impl<T> Parser for KeyValueParser<T>
where
    T: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser.parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::std_parse::U32Parser;

    #[test]
    fn test_key_value() {
        assert_eq!(
            KeyValueParser::new("key", U32Parser).parse(r#""key":32,"#.into()),
            Ok(("".into(), 32))
        );
        assert_eq!(
            KeyValueParser::new("key", U32Parser).parse(r#"key:32,"#.into()),
            Err(())
        );
        assert_eq!(
            KeyValueParser::new("key", U32Parser).parse(r#""key":32"#.into()),
            Err(())
        );
        assert_eq!(
            KeyValueParser::new("key", U32Parser).parse(r#" "key" : 32 , nice"#.into()),
            Ok(("nice".into(), 32))
        );
    }
}
