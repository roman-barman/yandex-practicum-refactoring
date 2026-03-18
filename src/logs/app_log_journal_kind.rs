use crate::entities::{UserBucket, UserCash};
use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, DelimitedParser, KeyValueParser, MapParser, PermutationParser,
    PrecededParser, StripWhitespaceParser, TagParser, U32Parser, UnquoteParser,
};

/// [Application]Log(AppLogKind), highest-level events
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogJournalKind {
    CreateUser {
        user_id: String,
        authorized_capital: u32,
    },
    DeleteUser {
        user_id: String,
    },
    RegisterAsset {
        asset_id: String,
        user_id: String,
        liquidity: u32,
    },
    UnregisterAsset {
        asset_id: String,
        user_id: String,
    },
    DepositCash(UserCash),
    WithdrawCash(UserCash),
    BuyAsset(UserBucket),
    SellAsset(UserBucket),
}

impl Parsable for AppLogJournalKind {
    type Parser = PrecededParser<
        TagParser,
        AltConditionParser<(
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    DelimitedParser<
                        TagParser,
                        PermutationParser<(
                            KeyValueParser<UnquoteParser>,
                            KeyValueParser<U32Parser>,
                        )>,
                        TagParser,
                    >,
                >,
                fn((String, u32)) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    DelimitedParser<TagParser, KeyValueParser<UnquoteParser>, TagParser>,
                >,
                fn(String) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    DelimitedParser<
                        TagParser,
                        PermutationParser<(
                            KeyValueParser<UnquoteParser>,
                            KeyValueParser<UnquoteParser>,
                            KeyValueParser<U32Parser>,
                        )>,
                        TagParser,
                    >,
                >,
                fn((String, String, u32)) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    DelimitedParser<
                        TagParser,
                        PermutationParser<(
                            KeyValueParser<UnquoteParser>,
                            KeyValueParser<UnquoteParser>,
                        )>,
                        TagParser,
                    >,
                >,
                fn((String, String)) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<StripWhitespaceParser<TagParser>, <UserCash as Parsable>::Parser>,
                fn(UserCash) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<StripWhitespaceParser<TagParser>, <UserCash as Parsable>::Parser>,
                fn(UserCash) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<StripWhitespaceParser<TagParser>, <UserBucket as Parsable>::Parser>,
                fn(UserBucket) -> AppLogJournalKind,
            >,
            MapParser<
                PrecededParser<StripWhitespaceParser<TagParser>, <UserBucket as Parsable>::Parser>,
                fn(UserBucket) -> AppLogJournalKind,
            >,
        )>,
    >;
    fn parser() -> Self::Parser {
        PrecededParser::new(
            TagParser::new("Journal"),
            AltConditionParser::<(
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        DelimitedParser<
                            TagParser,
                            PermutationParser<(
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<U32Parser>,
                            )>,
                            TagParser,
                        >,
                    >,
                    fn((String, u32)) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        DelimitedParser<TagParser, KeyValueParser<UnquoteParser>, TagParser>,
                    >,
                    fn(String) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        DelimitedParser<
                            TagParser,
                            PermutationParser<(
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<U32Parser>,
                            )>,
                            TagParser,
                        >,
                    >,
                    fn((String, String, u32)) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        DelimitedParser<
                            TagParser,
                            PermutationParser<(
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<UnquoteParser>,
                            )>,
                            TagParser,
                        >,
                    >,
                    fn((String, String)) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        <UserCash as Parsable>::Parser,
                    >,
                    fn(UserCash) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        <UserCash as Parsable>::Parser,
                    >,
                    fn(UserCash) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        <UserBucket as Parsable>::Parser,
                    >,
                    fn(UserBucket) -> AppLogJournalKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        <UserBucket as Parsable>::Parser,
                    >,
                    fn(UserBucket) -> AppLogJournalKind,
                >,
            )>::new(
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("CreateUser")),
                        DelimitedParser::new(
                            TagParser::new("{"),
                            PermutationParser::<(
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<U32Parser>,
                            )>::new(
                                KeyValueParser::new("user_id", UnquoteParser),
                                KeyValueParser::new("authorized_capital", U32Parser),
                            ),
                            TagParser::new("}"),
                        ),
                    ),
                    |(user_id, authorized_capital)| AppLogJournalKind::CreateUser {
                        user_id,
                        authorized_capital,
                    },
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("DeleteUser")),
                        DelimitedParser::new(
                            TagParser::new("{"),
                            KeyValueParser::new("user_id", UnquoteParser),
                            TagParser::new("}"),
                        ),
                    ),
                    |user_id| AppLogJournalKind::DeleteUser { user_id },
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("RegisterAsset")),
                        DelimitedParser::new(
                            TagParser::new("{"),
                            PermutationParser::<(
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<U32Parser>,
                            )>::new(
                                KeyValueParser::new("asset_id", UnquoteParser),
                                KeyValueParser::new("user_id", UnquoteParser),
                                KeyValueParser::new("liquidity", U32Parser),
                            ),
                            TagParser::new("}"),
                        ),
                    ),
                    |(asset_id, user_id, liquidity)| AppLogJournalKind::RegisterAsset {
                        asset_id,
                        user_id,
                        liquidity,
                    },
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("UnregisterAsset")),
                        DelimitedParser::new(
                            TagParser::new("{"),
                            PermutationParser::<(
                                KeyValueParser<UnquoteParser>,
                                KeyValueParser<UnquoteParser>,
                            )>::new(
                                KeyValueParser::new("asset_id", UnquoteParser),
                                KeyValueParser::new("user_id", UnquoteParser),
                            ),
                            TagParser::new("}"),
                        ),
                    ),
                    |(asset_id, user_id)| AppLogJournalKind::UnregisterAsset { asset_id, user_id },
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("DepositCash")),
                        UserCash::parser(),
                    ),
                    |user_cash| AppLogJournalKind::DepositCash(user_cash),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("WithdrawCash")),
                        UserCash::parser(),
                    ),
                    |user_cash| AppLogJournalKind::WithdrawCash(user_cash),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("BuyAsset")),
                        UserBucket::parser(),
                    ),
                    |user_backet| AppLogJournalKind::BuyAsset(user_backet),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("SellAsset")),
                        UserBucket::parser(),
                    ),
                    |user_backet| AppLogJournalKind::SellAsset(user_backet),
                ),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Bucket, UserBucket, UserCash};
    use crate::parse::Parser;

    #[test]
    fn test_create_user() {
        assert_eq!(
            AppLogJournalKind::parser().parse(
                r#"Journal CreateUser{"user_id":"alice","authorized_capital":1000,}"#.into()
            ),
            Ok((
                "".into(),
                AppLogJournalKind::CreateUser {
                    user_id: "alice".into(),
                    authorized_capital: 1000,
                }
            ))
        );
    }

    #[test]
    fn test_create_user_reversed_fields() {
        assert_eq!(
            AppLogJournalKind::parser().parse(
                r#"Journal CreateUser{"authorized_capital":1000,"user_id":"alice",}"#.into()
            ),
            Ok((
                "".into(),
                AppLogJournalKind::CreateUser {
                    user_id: "alice".into(),
                    authorized_capital: 1000,
                }
            ))
        );
    }

    #[test]
    fn test_delete_user() {
        assert_eq!(
            AppLogJournalKind::parser().parse(r#"Journal DeleteUser{"user_id":"alice",}"#.into()),
            Ok((
                "".into(),
                AppLogJournalKind::DeleteUser {
                    user_id: "alice".into(),
                }
            ))
        );
    }

    #[test]
    fn test_register_asset() {
        assert_eq!(
            AppLogJournalKind::parser().parse(
                r#"Journal RegisterAsset{"asset_id":"usd","user_id":"alice","liquidity":500,}"#
                    .into()
            ),
            Ok((
                "".into(),
                AppLogJournalKind::RegisterAsset {
                    asset_id: "usd".into(),
                    user_id: "alice".into(),
                    liquidity: 500,
                }
            ))
        );
    }

    #[test]
    fn test_register_asset_reversed_fields() {
        assert_eq!(
            AppLogJournalKind::parser().parse(
                r#"Journal RegisterAsset{"liquidity":500,"user_id":"alice","asset_id":"usd",}"#
                    .into()
            ),
            Ok((
                "".into(),
                AppLogJournalKind::RegisterAsset {
                    asset_id: "usd".into(),
                    user_id: "alice".into(),
                    liquidity: 500,
                }
            ))
        );
    }

    #[test]
    fn test_unregister_asset() {
        assert_eq!(
            AppLogJournalKind::parser()
                .parse(r#"Journal UnregisterAsset{"asset_id":"usd","user_id":"alice",}"#.into()),
            Ok((
                "".into(),
                AppLogJournalKind::UnregisterAsset {
                    asset_id: "usd".into(),
                    user_id: "alice".into(),
                }
            ))
        );
    }

    #[test]
    fn test_deposit_cash() {
        assert_eq!(
            AppLogJournalKind::parser()
                .parse(r#"Journal DepositCash UserCash{"user_id":"alice","count":100,}"#.into()),
            Ok((
                "".into(),
                AppLogJournalKind::DepositCash(UserCash::new("alice".into(), 100))
            ))
        );
    }

    #[test]
    fn test_withdraw_cash() {
        assert_eq!(
            AppLogJournalKind::parser()
                .parse(r#"Journal WithdrawCash UserCash{"user_id":"alice","count":50,}"#.into()),
            Ok((
                "".into(),
                AppLogJournalKind::WithdrawCash(UserCash::new("alice".into(), 50))
            ))
        );
    }

    #[test]
    fn test_buy_asset() {
        assert_eq!(
            AppLogJournalKind::parser().parse(
                r#"Journal BuyAsset UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":1,},}"#.into()
            ),
            Ok((
                "".into(),
                AppLogJournalKind::BuyAsset(UserBucket::new(
                    "alice".into(),
                    Bucket::new("usd".into(), 1)
                ))
            ))
        );
    }

    #[test]
    fn test_sell_asset() {
        assert_eq!(
            AppLogJournalKind::parser().parse(
                r#"Journal SellAsset UserBacket{"user_id":"alice","backet":Backet{"asset_id":"usd","count":1,},}"#.into()
            ),
            Ok((
                "".into(),
                AppLogJournalKind::SellAsset(UserBucket::new(
                    "alice".into(),
                    Bucket::new("usd".into(), 1)
                ))
            ))
        );
    }

    #[test]
    fn test_multiple_spaces_between_parts() {
        assert_eq!(
            AppLogJournalKind::parser().parse(r#"Journal   DeleteUser{"user_id":"alice",}"#.into()),
            Ok((
                "".into(),
                AppLogJournalKind::DeleteUser {
                    user_id: "alice".into(),
                }
            ))
        );
    }

    #[test]
    fn test_no_space_between_journal_and_action() {
        // StripWhitespaceParser strips 0 chars, so no separator is required
        assert_eq!(
            AppLogJournalKind::parser().parse(r#"JournalDeleteUser{"user_id":"alice",}"#.into()),
            Ok((
                "".into(),
                AppLogJournalKind::DeleteUser {
                    user_id: "alice".into(),
                }
            ))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            AppLogJournalKind::parser()
                .parse(r#"Journal DeleteUser{"user_id":"alice",} trailing"#.into()),
            Ok((
                " trailing".into(),
                AppLogJournalKind::DeleteUser {
                    user_id: "alice".into(),
                }
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_fails() {
        assert!(
            AppLogJournalKind::parser()
                .parse(r#" Journal DeleteUser{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            AppLogJournalKind::parser()
                .parse(r#"Trace DeleteUser{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_action_fails() {
        assert!(
            AppLogJournalKind::parser()
                .parse(r#"Journal UnknownAction{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_create_user_missing_authorized_capital_fails() {
        assert!(
            AppLogJournalKind::parser()
                .parse(r#"Journal CreateUser{"user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_create_user_missing_closing_brace_fails() {
        assert!(
            AppLogJournalKind::parser()
                .parse(r#"Journal CreateUser{"user_id":"alice","authorized_capital":1000,"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_register_asset_missing_liquidity_fails() {
        assert!(
            AppLogJournalKind::parser()
                .parse(r#"Journal RegisterAsset{"asset_id":"usd","user_id":"alice",}"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(AppLogJournalKind::parser().parse("Journal".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(AppLogJournalKind::parser().parse("".into()).is_err());
    }
}
