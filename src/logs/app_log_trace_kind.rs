use crate::entities::{Announcements, AuthData};
use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, MapParser, PrecededParser, StripWhitespaceParser, TagParser, UnquoteParser,
};

/// Trace [app](AppLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogTraceKind {
    Connect(AuthData),
    SendRequest(String),
    Check(Announcements),
    GetResponse(String),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Announcements, AuthData, Bucket, UserBuckets};
    use crate::parse::Parser;

    #[test]
    fn test_connect() {
        let auth = "aa".repeat(1024);
        let input = format!("Trace Connect {auth}");
        assert_eq!(
            AppLogTraceKind::parser().parse(input.as_str().into()),
            Ok((
                "".into(),
                AppLogTraceKind::Connect(AuthData::new(vec![0xaa; 1024]))
            ))
        );
    }

    #[test]
    fn test_send_request() {
        assert_eq!(
            AppLogTraceKind::parser()
                .parse(r#"Trace SendRequest "https://api.example.com""#.into()),
            Ok((
                "".into(),
                AppLogTraceKind::SendRequest("https://api.example.com".into())
            ))
        );
    }

    #[test]
    fn test_check() {
        assert_eq!(
            AppLogTraceKind::parser().parse(
                r#"Trace Check [UserBackets{"user_id":"alice","backets":[Backet{"asset_id":"usd","count":1,},],},]"#.into()
            ),
            Ok((
                "".into(),
                AppLogTraceKind::Check(Announcements::new(vec![UserBuckets::new(
                    "alice".into(),
                    vec![Bucket::new("usd".into(), 1)]
                )]))
            ))
        );
    }

    #[test]
    fn test_get_response() {
        assert_eq!(
            AppLogTraceKind::parser().parse(r#"Trace GetResponse "200 OK""#.into()),
            Ok(("".into(), AppLogTraceKind::GetResponse("200 OK".into())))
        );
    }

    #[test]
    fn test_multiple_spaces_between_parts() {
        assert_eq!(
            AppLogTraceKind::parser().parse(r#"Trace   SendRequest   "url""#.into()),
            Ok(("".into(), AppLogTraceKind::SendRequest("url".into())))
        );
    }

    #[test]
    fn test_no_space_between_trace_and_action() {
        // StripWhitespaceParser strips 0 chars, so no separator is required
        assert_eq!(
            AppLogTraceKind::parser().parse(r#"TraceSendRequest "url""#.into()),
            Ok(("".into(), AppLogTraceKind::SendRequest("url".into())))
        );
    }

    #[test]
    fn test_send_request_empty_string() {
        assert_eq!(
            AppLogTraceKind::parser().parse(r#"Trace SendRequest """#.into()),
            Ok(("".into(), AppLogTraceKind::SendRequest("".into())))
        );
    }

    #[test]
    fn test_get_response_empty_string() {
        assert_eq!(
            AppLogTraceKind::parser().parse(r#"Trace GetResponse """#.into()),
            Ok(("".into(), AppLogTraceKind::GetResponse("".into())))
        );
    }

    #[test]
    fn test_check_empty_announcements() {
        assert_eq!(
            AppLogTraceKind::parser().parse("Trace Check []".into()),
            Ok((
                "".into(),
                AppLogTraceKind::Check(Announcements::new(vec![]))
            ))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            AppLogTraceKind::parser().parse(r#"Trace SendRequest "url" trailing"#.into()),
            Ok((
                "trailing".into(),
                AppLogTraceKind::SendRequest("url".into())
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_fails() {
        // Unlike AppLogKind, there is no StripWhitespaceParser at the top level
        assert!(
            AppLogTraceKind::parser()
                .parse(r#" Trace SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            AppLogTraceKind::parser()
                .parse(r#"Error SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_action_fails() {
        assert!(
            AppLogTraceKind::parser()
                .parse(r#"Trace UnknownAction "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unquoted_value_fails() {
        assert!(
            AppLogTraceKind::parser()
                .parse("Trace SendRequest url".into())
                .is_err()
        );
    }

    #[test]
    fn test_connect_too_short_auth_data_fails() {
        // AuthData requires exactly 1024 bytes
        assert!(
            AppLogTraceKind::parser()
                .parse("Trace Connect short".into())
                .is_err()
        );
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(AppLogTraceKind::parser().parse("Trace".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(AppLogTraceKind::parser().parse("".into()).is_err());
    }
}
