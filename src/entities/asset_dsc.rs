use crate::parsable::Parsable;
use crate::parse::{
    AllConditionParser, DelimitedParser, KeyValueParser, MapParser, PermutationParser,
    StripWhitespaceParser, TagParser, UnquoteParser,
};

/// The pair 'abbreviated name of an object' - 'its description'
#[derive(Debug, Clone, PartialEq)]
pub struct AssetDsc {
    // `dsc` aka `description`
    id: String,
    dsc: String,
}
impl Parsable for AssetDsc {
    type Parser = MapParser<
        DelimitedParser<
            AllConditionParser<(
                StripWhitespaceParser<TagParser>,
                StripWhitespaceParser<TagParser>,
            )>,
            PermutationParser<(KeyValueParser<UnquoteParser>, KeyValueParser<UnquoteParser>)>,
            StripWhitespaceParser<TagParser>,
        >,
        fn((String, String)) -> Self,
    >;
    fn parser() -> Self::Parser {
        MapParser::new(
            DelimitedParser::new(
                AllConditionParser::<(
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<TagParser>,
                )>::new(
                    StripWhitespaceParser::new(TagParser::new("AssetDsc")),
                    StripWhitespaceParser::new(TagParser::new("{")),
                ),
                PermutationParser::<(KeyValueParser<UnquoteParser>, KeyValueParser<UnquoteParser>)>::new(
                    KeyValueParser::new("id", UnquoteParser),
                    KeyValueParser::new("dsc", UnquoteParser),
                ),
                StripWhitespaceParser::new(TagParser::new("}")),
            ),
            |(id, dsc)| AssetDsc { id, dsc },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_asset_dsc() {
        assert_eq!(
            AllConditionParser::<(
                StripWhitespaceParser<TagParser>,
                StripWhitespaceParser<TagParser>
            )>::new(
                StripWhitespaceParser::new(TagParser::new("AssetDsc")),
                StripWhitespaceParser::new(TagParser::new("{"))
            )
            .parse(" AssetDsc { ".into()),
            Ok(("".into(), ((), ())))
        );

        assert_eq!(
            AssetDsc::parser().parse(r#"AssetDsc{"id":"usd","dsc":"USA dollar",}"#.into()),
            Ok((
                "".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );
        assert_eq!(
            AssetDsc::parser()
                .parse(r#" AssetDsc { "id" : "usd" , "dsc" : "USA dollar" , } "#.into()),
            Ok((
                "".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );
        assert_eq!(
            AssetDsc::parser()
                .parse(r#" AssetDsc { "id" : "usd" , "dsc" : "USA dollar" , } nice "#.into()),
            Ok((
                "nice ".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );

        assert_eq!(
            AssetDsc::parser().parse(r#"AssetDsc{"dsc":"USA dollar","id":"usd",}"#.into()),
            Ok((
                "".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );
    }
}
