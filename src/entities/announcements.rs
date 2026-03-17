use crate::entities::UserBuckets;
use crate::parsable::Parsable;
use crate::parse::{ListParser, MapParser};

/// List of published buckets
#[derive(Debug, Clone, PartialEq)]
pub struct Announcements(Vec<UserBuckets>);
impl Parsable for Announcements {
    type Parser =
        MapParser<ListParser<<UserBuckets as Parsable>::Parser>, fn(Vec<UserBuckets>) -> Self>;
    fn parser() -> Self::Parser {
        fn from_vec(vec: Vec<UserBuckets>) -> Announcements {
            Announcements(vec)
        }
        MapParser::new(ListParser::new(UserBuckets::parser()), from_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Bucket;
    use crate::parse::Parser;

    #[test]
    fn test_announcements_single_element() {
        assert_eq!(
            Announcements::parser().parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],},]"#
                    .into()
            ),
            Ok((
                "".into(),
                Announcements(vec![UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }])
            ))
        );
    }

    #[test]
    fn test_announcements_multiple_elements() {
        assert_eq!(
            Announcements::parser().parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],},UserBackets{"user_id":"bob","backets":[Backet{"asset_id":"eur","count":2,},],},]"#
                    .into()
            ),
            Ok((
                "".into(),
                Announcements(vec![
                    UserBuckets {
                        user_id: "alice".into(),
                        buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                    },
                    UserBuckets {
                        user_id: "bob".into(),
                        buckets: vec![Bucket { asset_id: "eur".into(), count: 2 }]
                    },
                ])
            ))
        );
    }

    #[test]
    fn test_announcements_empty_list() {
        assert_eq!(
            Announcements::parser().parse(r#"[]"#.into()),
            Ok(("".into(), Announcements(vec![])))
        );
    }

    #[test]
    fn test_announcements_element_with_multiple_buckets() {
        assert_eq!(
            Announcements::parser().parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},Backet{"asset_id":"eur","count":2,},],},]"#
                    .into()
            ),
            Ok((
                "".into(),
                Announcements(vec![UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![
                        Bucket { asset_id: "usd".into(), count: 1 },
                        Bucket { asset_id: "eur".into(), count: 2 },
                    ]
                }])
            ))
        );
    }

    #[test]
    fn test_announcements_element_with_empty_bucket_list() {
        assert_eq!(
            Announcements::parser()
                .parse(r#"[UserBackets{"user_id":"alice","backets":[],},]"#.into()),
            Ok((
                "".into(),
                Announcements(vec![UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![]
                }])
            ))
        );
    }

    #[test]
    fn test_announcements_remaining_text_preserved() {
        assert_eq!(
            Announcements::parser().parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],},] trailing"#
                    .into()
            ),
            Ok((
                "trailing".into(),
                Announcements(vec![UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }])
            ))
        );
    }

    #[test]
    fn test_announcements_whitespace_around_brackets_and_elements() {
        assert_eq!(
            Announcements::parser().parse(
                r#" [ UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],} , ] "#
                    .into()
            ),
            Ok((
                "".into(),
                Announcements(vec![UserBuckets {
                    user_id: "alice".into(),
                    buckets: vec![Bucket { asset_id: "usd".into(), count: 1 }]
                }])
            ))
        );
    }

    #[test]
    fn test_announcements_missing_opening_bracket() {
        assert!(Announcements::parser()
            .parse(
                r#"UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],}]"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_announcements_missing_closing_bracket() {
        assert!(Announcements::parser()
            .parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],},"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_announcements_element_missing_trailing_comma() {
        assert!(Announcements::parser()
            .parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],}]"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_announcements_wrong_element_tag() {
        assert!(Announcements::parser()
            .parse(
                r#"[UserBuckets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],},]"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_announcements_element_missing_user_id() {
        assert!(
            Announcements::parser()
                .parse(
                    r#"[UserBackets{"backets":[Backet{"asset_id":"usd","count":1,},],},]"#.into()
                )
                .is_err()
        );
    }

    #[test]
    fn test_announcements_element_missing_backets_field() {
        assert!(
            Announcements::parser()
                .parse(r#"[UserBackets{"user_id":"alice",},]"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_announcements_zero_count_in_nested_bucket() {
        assert!(Announcements::parser()
            .parse(
                r#"[UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":0,},],},]"#
                    .into()
            )
            .is_err());
    }

    #[test]
    fn test_announcements_empty_input() {
        assert!(Announcements::parser().parse("".into()).is_err());
    }
}
