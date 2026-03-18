use crate::logs::LogKind;
use crate::parsable::Parsable;
use crate::parse::{
    AllConditionParser, MapParser, PrecededParser, StripWhitespaceParser, TagParser, U32Parser,
};

/// Log line, [log](AppLogKind) with `requestid`
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logs::system_log_error_kind::SystemLogErrorKind;
    use crate::logs::{AppLogErrorKind, AppLogKind, SystemLogKind};
    use crate::parse::Parser;

    #[test]
    fn test_app_log() {
        assert_eq!(
            LogLine::parser().parse(r#"App::Error LackOf "msg" requestid=42"#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 42,
                }
            ))
        );
    }

    #[test]
    fn test_system_log() {
        assert_eq!(
            LogLine::parser().parse(r#"System::Error NetworkError "url" requestid=7"#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::System(SystemLogKind::Error(SystemLogErrorKind::NetworkError(
                        "url".into()
                    ))),
                    request_id: 7,
                }
            ))
        );
    }

    #[test]
    fn test_request_id_hex() {
        assert_eq!(
            LogLine::parser().parse(r#"App::Error LackOf "msg" requestid=0x2a"#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 42,
                }
            ))
        );
    }

    #[test]
    fn test_request_id_max_u32() {
        assert_eq!(
            LogLine::parser().parse(r#"App::Error LackOf "msg" requestid=4294967295"#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 4294967295,
                }
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_is_allowed() {
        // LogKind has StripWhitespaceParser at its top level
        assert_eq!(
            LogLine::parser().parse(r#"   App::Error LackOf "msg" requestid=1"#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 1,
                }
            ))
        );
    }

    #[test]
    fn test_multiple_spaces_before_requestid() {
        assert_eq!(
            LogLine::parser().parse(r#"App::Error LackOf "msg"   requestid=1"#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 1,
                }
            ))
        );
    }

    #[test]
    fn test_trailing_whitespace_is_consumed() {
        assert_eq!(
            LogLine::parser().parse(r#"App::Error LackOf "msg" requestid=1   "#.into()),
            Ok((
                "".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 1,
                }
            ))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            LogLine::parser().parse(r#"App::Error LackOf "msg" requestid=1 trailing"#.into()),
            Ok((
                "trailing".into(),
                LogLine {
                    kind: LogKind::App(AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))),
                    request_id: 1,
                }
            ))
        );
    }

    #[test]
    fn test_request_id_zero_fails() {
        // U32Parser uses NonZeroU32 — zero is rejected
        assert!(
            LogLine::parser()
                .parse(r#"App::Error LackOf "msg" requestid=0"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_missing_requestid_fails() {
        assert!(
            LogLine::parser()
                .parse(r#"App::Error LackOf "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_missing_requestid_prefix_fails() {
        // a bare number without "requestid="
        assert!(
            LogLine::parser()
                .parse(r#"App::Error LackOf "msg" 42"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_requestid_non_numeric_fails() {
        assert!(
            LogLine::parser()
                .parse(r#"App::Error LackOf "msg" requestid=abc"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_requestid_only_fails() {
        assert!(LogLine::parser().parse("requestid=42".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(LogLine::parser().parse("".into()).is_err());
    }
}
