use crate::parsable::Parsable;
pub(crate) use crate::parse::all_parse::AllConditionParser;
pub(crate) use crate::parse::alt_parse::AltConditionParser;
pub(crate) use crate::parse::delimited_parse::DelimitedParser;
pub(crate) use crate::parse::key_value_parse::KeyValueParser;
pub(crate) use crate::parse::list_parse::ListParser;
pub(crate) use crate::parse::map_parse::MapParser;
pub(crate) use crate::parse::permutation_parse::PermutationParser;
pub(crate) use crate::parse::preceded_parse::PrecededParser;
pub(crate) use crate::parse::std_parse::U32Parser;
pub(crate) use crate::parse::strip_whitespace_parse::StripWhitespaceParser;
pub(crate) use crate::parse::tag_parse::TagParser;
pub(crate) use crate::parse::unquote_parse::UnquoteParser;

mod all_parse;
mod alt_parse;
mod as_is_parse;
mod delimited_parse;
mod key_value_parse;
mod list_parse;
mod map_parse;
mod permutation_parse;
mod preceded_parse;
mod quoted_tag_parse;
mod std_parse;
mod strip_whitespace_parse;
mod tag_parse;
mod take_parse;
mod unquote_parse;

use crate::entities::{Announcements, AuthData, UserBucket, UserCash};
use crate::logs::LogKind;
pub(crate) use std_parse::*;
pub(crate) use take_parse::TakeParser;

/// Trait to implement and require the 'parse and show what remains to be parsed' method
pub trait Parser {
    type Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()>;
}

/// Error [системы](SystemLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum SystemLogErrorKind {
    NetworkError(String),
    AccessDenied(String),
}
/// Все виды [логов приложения](LogKind) логов
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogKind {
    Error(AppLogErrorKind),
    Trace(AppLogTraceKind),
    Journal(AppLogJournalKind),
}
/// Error [приложения](AppLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogErrorKind {
    LackOf(String),
    SystemError(String),
}
// подсказка: а поля не слишком много места на стэке занимают?
/// Trace [приложения](AppLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogTraceKind {
    Connect(AuthData),
    SendRequest(String),
    Check(Announcements),
    GetResponse(String),
}
/// Журнал [приложения](AppLogKind), самые высокоуровневые события
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
impl Parsable for SystemLogErrorKind {
    type Parser = PrecededParser<
        TagParser,
        AltConditionParser<(
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> SystemLogErrorKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> SystemLogErrorKind,
            >,
        )>,
    >;
    fn parser() -> Self::Parser {
        PrecededParser::new(
            TagParser::new("Error"),
            AltConditionParser::<(
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> SystemLogErrorKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> SystemLogErrorKind,
                >,
            )>::new(
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("NetworkError")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    |error| SystemLogErrorKind::NetworkError(error),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("AccessDenied")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    |error| SystemLogErrorKind::AccessDenied(error),
                ),
            ),
        )
    }
}

impl Parsable for AppLogErrorKind {
    type Parser = PrecededParser<
        TagParser,
        AltConditionParser<(
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> AppLogErrorKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> AppLogErrorKind,
            >,
        )>,
    >;
    fn parser() -> Self::Parser {
        PrecededParser::new(
            TagParser::new("Error"),
            AltConditionParser::<(
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> AppLogErrorKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> AppLogErrorKind,
                >,
            )>::new(
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("LackOf")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    |error| AppLogErrorKind::LackOf(error),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("SystemError")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    |error| AppLogErrorKind::SystemError(error),
                ),
            ),
        )
    }
}
impl Parsable for AppLogTraceKind {
    type Parser = PrecededParser<
        TagParser,
        AltConditionParser<(
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<<AuthData as Parsable>::Parser>,
                >,
                fn(AuthData) -> AppLogTraceKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> AppLogTraceKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<<Announcements as Parsable>::Parser>,
                >,
                fn(Announcements) -> AppLogTraceKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> AppLogTraceKind,
            >,
        )>,
    >;
    fn parser() -> Self::Parser {
        PrecededParser::new(
            TagParser::new("Trace"),
            AltConditionParser::<(
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<<AuthData as Parsable>::Parser>,
                    >,
                    fn(AuthData) -> AppLogTraceKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> AppLogTraceKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<<Announcements as Parsable>::Parser>,
                    >,
                    fn(Announcements) -> AppLogTraceKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> AppLogTraceKind,
                >,
            )>::new(
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("Connect")),
                        StripWhitespaceParser::new(AuthData::parser()),
                    ),
                    |authdata| AppLogTraceKind::Connect(authdata),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("SendRequest")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    |trace| AppLogTraceKind::SendRequest(trace),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("Check")),
                        StripWhitespaceParser::new(Announcements::parser()),
                    ),
                    |announcements| AppLogTraceKind::Check(announcements),
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("GetResponse")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    |trace| AppLogTraceKind::GetResponse(trace),
                ),
            ),
        )
    }
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
                    |user_cash| AppLogJournalKind::DepositCash(user_cash),
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
impl Parsable for AppLogKind {
    type Parser = StripWhitespaceParser<
        PrecededParser<
            TagParser,
            AltConditionParser<(
                MapParser<<AppLogErrorKind as Parsable>::Parser, fn(AppLogErrorKind) -> AppLogKind>,
                MapParser<<AppLogTraceKind as Parsable>::Parser, fn(AppLogTraceKind) -> AppLogKind>,
                MapParser<
                    <AppLogJournalKind as Parsable>::Parser,
                    fn(AppLogJournalKind) -> AppLogKind,
                >,
            )>,
        >,
    >;
    fn parser() -> Self::Parser {
        StripWhitespaceParser::new(PrecededParser::new(
            TagParser::new("App::"),
            AltConditionParser::<(
                MapParser<<AppLogErrorKind as Parsable>::Parser, fn(AppLogErrorKind) -> AppLogKind>,
                MapParser<<AppLogTraceKind as Parsable>::Parser, fn(AppLogTraceKind) -> AppLogKind>,
                MapParser<
                    <AppLogJournalKind as Parsable>::Parser,
                    fn(AppLogJournalKind) -> AppLogKind,
                >,
            )>::new(
                MapParser::new(AppLogErrorKind::parser(), |error| AppLogKind::Error(error)),
                MapParser::new(AppLogTraceKind::parser(), |trace| AppLogKind::Trace(trace)),
                MapParser::new(AppLogJournalKind::parser(), |journal| {
                    AppLogKind::Journal(journal)
                }),
            ),
        ))
    }
}
/// Строка логов, [лог](AppLogKind) с `request_id`
#[derive(Debug, Clone, PartialEq)]
pub struct LogLine {
    pub kind: LogKind,
    pub request_id: u32,
}
impl Parsable for LogLine {
    type Parser = MapParser<
        AllConditionParser<(
            <LogKind as Parsable>::Parser,
            StripWhitespaceParser<PrecededParser<TagParser, U32Parser>>,
        )>,
        fn((LogKind, u32)) -> Self,
    >;
    fn parser() -> Self::Parser {
        MapParser::new(
            AllConditionParser::<(
                <LogKind as Parsable>::Parser,
                StripWhitespaceParser<PrecededParser<TagParser, U32Parser>>,
            )>::new(
                LogKind::parser(),
                StripWhitespaceParser::new(PrecededParser::new(
                    TagParser::new("requestid="),
                    U32Parser,
                )),
            ),
            |(kind, request_id)| LogLine { kind, request_id },
        )
    }
}

/// Парсер строки логов
pub struct LogLineParser {
    parser: std::sync::OnceLock<<LogLine as Parsable>::Parser>,
}
impl LogLineParser {
    pub fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, LogLine), ()> {
        self.parser
            .get_or_init(|| <LogLine as Parsable>::parser())
            .parse(input)
    }
}
// подсказка: singleton, без которого можно обойтись
// парсеры не страшно вытащить в pub
/// Единожды собранный парсер логов
pub static LOG_LINE_PARSER: LogLineParser = LogLineParser {
    parser: std::sync::OnceLock::new(),
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_log_kind() {
        assert_eq!(
            PrecededParser::new(
                StripWhitespaceParser::new(TagParser::new("NetworkError")),
                StripWhitespaceParser::new(UnquoteParser)
            )
            .parse(r#"NetworkError "url unknown""#.into()),
            Ok(("".into(), "url unknown".into()))
        );
    }
}
