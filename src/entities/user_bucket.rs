use crate::entities::Bucket;
use crate::parsable::Parsable;
use crate::parse::{
    AllConditionParser, DelimitedParser, KeyValueParser, MapParser, PermutationParser,
    StripWhitespaceParser, TagParser, UnquoteParser,
};

/// [Bucket] of a specific user
#[derive(Debug, Clone, PartialEq)]
pub struct UserBucket {
    user_id: String,
    bucket: Bucket,
}
impl Parsable for UserBucket {
    type Parser = MapParser<
        DelimitedParser<
            AllConditionParser<(
                StripWhitespaceParser<TagParser>,
                StripWhitespaceParser<TagParser>,
            )>,
            PermutationParser<(
                KeyValueParser<UnquoteParser>,
                KeyValueParser<<Bucket as Parsable>::Parser>,
            )>,
            StripWhitespaceParser<TagParser>,
        >,
        fn((String, Bucket)) -> Self,
    >;
    fn parser() -> Self::Parser {
        MapParser::new(
            DelimitedParser::new(
                AllConditionParser::<(
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<TagParser>,
                )>::new(
                    StripWhitespaceParser::new(TagParser::new("UserBacket")),
                    StripWhitespaceParser::new(TagParser::new("{")),
                ),
                PermutationParser::<(
                    KeyValueParser<UnquoteParser>,
                    KeyValueParser<<Bucket as Parsable>::Parser>,
                )>::new(
                    KeyValueParser::new("user_id", UnquoteParser),
                    KeyValueParser::new("backet", Bucket::parser()),
                ),
                StripWhitespaceParser::new(TagParser::new("}")),
            ),
            |(user_id, bucket)| UserBucket { user_id, bucket },
        )
    }
}

#[cfg(test)]
impl UserBucket {
    pub fn new(user_id: String, bucket: Bucket) -> Self {
        UserBucket { user_id, bucket }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_user_bucket_standard_order() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":42,},}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "alice".into(),
                    bucket: Bucket::new("usd".into(), 42)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_reversed_field_order() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"backet":Backet{"asset_id":"usd","count":42,},"user_id":"alice",}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "alice".into(),
                    bucket: Bucket::new("usd".into(), 42)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_whitespace_around_tag_and_brace() {
        assert_eq!(
            UserBucket::parser().parse(
                r#" UserBacket { "user_id":"alice","backet":Backet{"asset_id":"usd","count":1,},}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "alice".into(),
                    bucket: Bucket::new("usd".into(), 1)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_remaining_text_preserved() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":1,},} trailing"#
                    .into()
            ),
            Ok((
                "trailing".into(),
                UserBucket {
                    user_id: "alice".into(),
                    bucket: Bucket::new("usd".into(), 1)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_empty_user_id() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"user_id":"","backet":Backet{"asset_id":"usd","count":1,},}"#.into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "".into(),
                    bucket: Bucket::new("usd".into(), 1)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_numeric_looking_user_id() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"user_id":"123","backet":Backet{"asset_id":"usd","count":10,},}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "123".into(),
                    bucket: Bucket::new("usd".into(), 10)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_zero_count() {
        assert!(UserBucket::parser()
            .parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":0,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_bucket_max_u32_count() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":4294967295,},}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "alice".into(),
                    bucket: Bucket::new("usd".into(), 4294967295)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_nested_fields_reversed_order() {
        assert_eq!(
            UserBucket::parser().parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"count":42,"asset_id":"usd",},}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBucket {
                    user_id: "alice".into(),
                    bucket: Bucket::new("usd".into(), 42)
                }
            ))
        );
    }

    #[test]
    fn test_user_bucket_wrong_outer_tag() {
        assert!(UserBucket::parser()
            .parse(
                r#"UserBucket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":1,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_bucket_wrong_bucket_field_key() {
        assert!(UserBucket::parser()
            .parse(
                r#"UserBacket{"user_id":"alice","bucket":Backet{"asset_id":"usd","count":1,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_bucket_wrong_nested_type_tag() {
        assert!(UserBucket::parser()
            .parse(
                r#"UserBacket{"user_id":"alice","backet":Bucket{"asset_id":"usd","count":1,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_bucket_missing_user_id_field() {
        assert!(
            UserBucket::parser()
                .parse(r#"UserBacket{"backet":Backet{"asset_id":"usd","count":1,},}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_bucket_missing_backet_field() {
        assert!(
            UserBucket::parser()
                .parse(r#"UserBacket{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_bucket_missing_closing_brace() {
        assert!(
            UserBucket::parser()
                .parse(
                    r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":1,},"#
                        .into()
                )
                .is_err()
        );
    }

    #[test]
    fn test_user_bucket_missing_asset_id_in_nested_bucket() {
        assert!(
            UserBucket::parser()
                .parse(r#"UserBacket{"user_id":"alice","backet":Backet{"count":1,},}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_bucket_missing_count_in_nested_bucket() {
        assert!(
            UserBucket::parser()
                .parse(
                    r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd",},}"#.into()
                )
                .is_err()
        );
    }

    #[test]
    fn test_user_bucket_negative_count_in_nested_bucket() {
        assert!(UserBucket::parser()
            .parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":-1,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_bucket_float_count_in_nested_bucket() {
        assert!(UserBucket::parser()
            .parse(
                r#"UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":1.5,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_bucket_empty_input() {
        assert!(UserBucket::parser().parse("".into()).is_err());
    }
}
