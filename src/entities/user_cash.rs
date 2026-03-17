use crate::parsable::Parsable;
use crate::parse::{
    AllConditionParser, DelimitedParser, KeyValueParser, MapParser, PermutationParser,
    StripWhitespaceParser, TagParser, U32Parser, UnquoteParser,
};

/// Fiat money of a specific user
#[derive(Debug, Clone, PartialEq)]
pub struct UserCash {
    pub user_id: String,
    pub count: u32,
}
impl Parsable for UserCash {
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
                    StripWhitespaceParser::new(TagParser::new("UserCash")),
                    StripWhitespaceParser::new(TagParser::new("{")),
                ),
                PermutationParser::<(KeyValueParser<UnquoteParser>, KeyValueParser<U32Parser>)>::new(
                    KeyValueParser::new("user_id", UnquoteParser),
                    KeyValueParser::new("count", U32Parser),
                ),
                StripWhitespaceParser::new(TagParser::new("}")),
            ),
            |(user_id, count)| UserCash { user_id, count },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_user_cash_standard_order() {
        assert_eq!(
            UserCash::parser().parse(r#"UserCash{"user_id":"alice","count":100,}"#.into()),
            Ok((
                "".into(),
                UserCash {
                    user_id: "alice".into(),
                    count: 100
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_reversed_field_order() {
        assert_eq!(
            UserCash::parser().parse(r#"UserCash{"count":100,"user_id":"alice",}"#.into()),
            Ok((
                "".into(),
                UserCash {
                    user_id: "alice".into(),
                    count: 100
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_empty_user_id() {
        assert_eq!(
            UserCash::parser().parse(r#"UserCash{"user_id":"","count":1,}"#.into()),
            Ok((
                "".into(),
                UserCash {
                    user_id: "".into(),
                    count: 1
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_numeric_looking_user_id() {
        assert_eq!(
            UserCash::parser().parse(r#"UserCash{"user_id":"123","count":10,}"#.into()),
            Ok((
                "".into(),
                UserCash {
                    user_id: "123".into(),
                    count: 10
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_max_u32_count() {
        assert_eq!(
            UserCash::parser().parse(r#"UserCash{"user_id":"alice","count":4294967295,}"#.into()),
            Ok((
                "".into(),
                UserCash {
                    user_id: "alice".into(),
                    count: 4294967295
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_whitespace_around_tag_and_brace() {
        assert_eq!(
            UserCash::parser()
                .parse(r#" UserCash { "user_id" : "alice" , "count" : 1 , } "#.into()),
            Ok((
                "".into(),
                UserCash {
                    user_id: "alice".into(),
                    count: 1
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_remaining_text_preserved() {
        assert_eq!(
            UserCash::parser().parse(r#"UserCash{"user_id":"alice","count":1,} trailing"#.into()),
            Ok((
                "trailing".into(),
                UserCash {
                    user_id: "alice".into(),
                    count: 1
                }
            ))
        );
    }

    #[test]
    fn test_user_cash_wrong_tag() {
        assert!(
            UserCash::parser()
                .parse(r#"Cash{"user_id":"alice","count":1,}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_missing_closing_brace() {
        assert!(
            UserCash::parser()
                .parse(r#"UserCash{"user_id":"alice","count":1,"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_missing_user_id_field() {
        assert!(
            UserCash::parser()
                .parse(r#"UserCash{"count":1,}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_missing_count_field() {
        assert!(
            UserCash::parser()
                .parse(r#"UserCash{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_wrong_field_name() {
        assert!(
            UserCash::parser()
                .parse(r#"UserCash{"userId":"alice","count":1,}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_negative_count() {
        assert!(
            UserCash::parser()
                .parse(r#"UserCash{"user_id":"alice","count":-1,}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_float_count() {
        assert!(
            UserCash::parser()
                .parse(r#"UserCash{"user_id":"alice","count":1.5,}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_cash_empty_input() {
        assert!(UserCash::parser().parse("".into()).is_err());
    }
}
