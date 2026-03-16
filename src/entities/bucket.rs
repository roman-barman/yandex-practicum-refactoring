use crate::parsable::Parsable;
use crate::parse::{
    AllConditionParser, DelimitedParser, KeyValueParser, MapParser, PermutationParser,
    StripWhitespaceParser, TagParser, U32Parser, UnquoteParser,
};

/// Information about an item in some quantity
#[derive(Debug, Clone, PartialEq)]
pub struct Bucket {
    pub asset_id: String,
    pub count: u32,
}
impl Parsable for Bucket {
    type Parser = MapParser<
        DelimitedParser<
            AllConditionParser<(
                StripWhitespaceParser<TagParser>,
                StripWhitespaceParser<TagParser>,
            )>,
            PermutationParser<(KeyValueParser<UnquoteParser>, KeyValueParser<U32Parser>)>,
            StripWhitespaceParser<TagParser>,
        >,
        fn((String, u32)) -> Self,
    >;
    fn parser() -> Self::Parser {
        MapParser::new(
            DelimitedParser::new(
                AllConditionParser::<(
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<TagParser>,
                )>::new(
                    StripWhitespaceParser::new(TagParser::new("Backet")),
                    StripWhitespaceParser::new(TagParser::new("{")),
                ),
                PermutationParser::<(KeyValueParser<UnquoteParser>, KeyValueParser<U32Parser>)>::new(
                    KeyValueParser::new("asset_id", UnquoteParser),
                    KeyValueParser::new("count", U32Parser),
                ),
                StripWhitespaceParser::new(TagParser::new("}")),
            ),
            |(asset_id, count)| Bucket { asset_id, count },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_bucket() {
        assert_eq!(
            Bucket::parser().parse(r#"Backet{"asset_id":"usd","count":42,}"#.into()),
            Ok((
                "".into(),
                Bucket {
                    asset_id: "usd".into(),
                    count: 42
                }
            ))
        );
        assert_eq!(
            Bucket::parser().parse(r#"Backet{"count":42,"asset_id":"usd",}"#.into()),
            Ok((
                "".into(),
                Bucket {
                    asset_id: "usd".into(),
                    count: 42
                }
            ))
        );
    }
}
