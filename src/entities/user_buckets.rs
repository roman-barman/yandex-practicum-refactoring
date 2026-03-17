use crate::entities::Bucket;
use crate::parsable::Parsable;
use crate::parse::{
    AllConditionParser, DelimitedParser, KeyValueParser, ListParser, MapParser, PermutationParser,
    StripWhitespaceParser, TagParser, UnquoteParser,
};

/// [Bucket] of a specific user
#[derive(Debug, Clone, PartialEq)]
pub struct UserBuckets {
    pub user_id: String,
    pub buckets: Vec<Bucket>,
}

impl Parsable for UserBuckets {
    type Parser = MapParser<
        DelimitedParser<
            AllConditionParser<(
                StripWhitespaceParser<TagParser>,
                StripWhitespaceParser<TagParser>,
            )>,
            PermutationParser<(
                KeyValueParser<UnquoteParser>,
                KeyValueParser<ListParser<<Bucket as Parsable>::Parser>>,
            )>,
            StripWhitespaceParser<TagParser>,
        >,
        fn((String, Vec<Bucket>)) -> Self,
    >;
    fn parser() -> Self::Parser {
        MapParser::new(
            DelimitedParser::new(
                AllConditionParser::<(
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<TagParser>,
                )>::new(
                    StripWhitespaceParser::new(TagParser::new("UserBackets")),
                    StripWhitespaceParser::new(TagParser::new("{")),
                ),
                PermutationParser::<(
                    KeyValueParser<UnquoteParser>,
                    KeyValueParser<ListParser<<Bucket as Parsable>::Parser>>,
                )>::new(
                    KeyValueParser::new("user_id", UnquoteParser),
                    KeyValueParser::new("backets", ListParser::new(Bucket::parser())),
                ),
                StripWhitespaceParser::new(TagParser::new("}")),
            ),
            |(user_id, buckets)| UserBuckets { user_id, buckets },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_user_buckets_single_bucket() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_multiple_buckets() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},Backet{"asset_id":"eur","count":2,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![
                        Bucket { asset_id: "usd".into(), count: 1 },
                        Bucket { asset_id: "eur".into(), count: 2 },
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_empty_list() {
        assert_eq!(
            UserBuckets::parser().parse(r#"UserBackets{"user_id":"alice","backets":[],}"#.into()),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_reversed_field_order() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"backets":[Backet{"asset_id":"usd","count":1,},],"user_id":"alice",}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_whitespace_around_tag_and_brace() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#" UserBackets { "user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_remaining_text_preserved() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],} trailing"#
                    .into()
            ),
            Ok((
                "trailing".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_empty_user_id() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"","backets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "".into(),
                    buckets: vec![Bucket {
                        asset_id: "usd".into(),
                        count: 1
                    }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_numeric_looking_user_id() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"123","backets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "123".into(),
                    buckets: vec![Bucket {
                        asset_id: "usd".into(),
                        count: 1
                    }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_nested_fields_reversed_inside_element() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"count":1,"asset_id":"usd",},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_duplicate_asset_ids_allowed() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},Backet{"asset_id":"usd","count":2,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![
                        Bucket { asset_id: "usd".into(), count: 1 },
                        Bucket { asset_id: "usd".into(), count: 2 },
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_max_u32_count_in_element() {
        assert_eq!(
            UserBuckets::parser().parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":4294967295,},],}"#
                    .into()
            ),
            Ok((
                "".into(),
                UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 4294967295 }]
                }
            ))
        );
    }

    #[test]
    fn test_user_buckets_wrong_outer_tag() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBuckets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_wrong_buckets_field_key() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","buckets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_wrong_user_id_field_name() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"userId":"alice","backets":[Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_missing_user_id_field() {
        assert!(
            UserBuckets::parser()
                .parse(r#"UserBackets{"backets":[Backet{"asset_id":"usd","count":1,},],}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_buckets_missing_backets_field() {
        assert!(
            UserBuckets::parser()
                .parse(r#"UserBackets{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_user_buckets_missing_closing_outer_brace() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_missing_opening_bracket() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":Backet{"asset_id":"usd","count":1,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_missing_closing_bracket() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_element_missing_trailing_comma() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,}],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_wrong_element_tag() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Bucket{"asset_id":"usd","count":1,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_zero_count_in_element() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":0,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_negative_count_in_element() {
        assert!(UserBuckets::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":-1,},],}"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_user_buckets_empty_input() {
        assert!(UserBuckets::parser().parse("".into()).is_err());
    }
}
