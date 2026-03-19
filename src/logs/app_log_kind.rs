use crate::logs::{AppLogErrorKind, AppLogJournalKind, AppLogTraceKind};
use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, MapParser, PrecededParser, StripWhitespaceParser, TagParser,
};

/// All types of [application logs](LogKind) logs
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogKind {
    Error(AppLogErrorKind),
    Trace(AppLogTraceKind),
    Journal(AppLogJournalKind),
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
                MapParser::new(AppLogErrorKind::parser(), AppLogKind::Error),
                MapParser::new(AppLogTraceKind::parser(), AppLogKind::Trace),
                MapParser::new(AppLogJournalKind::parser(), AppLogKind::Journal),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_error() {
        assert_eq!(
            AppLogKind::parser().parse(r#"App::Error LackOf "some resource""#.into()),
            Ok((
                "".into(),
                AppLogKind::Error(AppLogErrorKind::LackOf("some resource".into()))
            ))
        );
    }

    #[test]
    fn test_trace() {
        assert_eq!(
            AppLogKind::parser()
                .parse(r#"App::Trace SendRequest "https://api.example.com""#.into()),
            Ok((
                "".into(),
                AppLogKind::Trace(AppLogTraceKind::SendRequest(
                    "https://api.example.com".into()
                ))
            ))
        );
    }

    #[test]
    fn test_journal() {
        assert_eq!(
            AppLogKind::parser().parse(r#"App::Journal DeleteUser{"user_id":"alice",}"#.into()),
            Ok((
                "".into(),
                AppLogKind::Journal(AppLogJournalKind::DeleteUser {
                    user_id: "alice".into(),
                })
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_is_allowed() {
        assert_eq!(
            AppLogKind::parser().parse(r#"   App::Error LackOf "msg""#.into()),
            Ok((
                "".into(),
                AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))
            ))
        );
    }

    #[test]
    fn test_trailing_whitespace_is_consumed() {
        assert_eq!(
            AppLogKind::parser().parse(r#"App::Error LackOf "msg"   "#.into()),
            Ok((
                "".into(),
                AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))
            ))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            AppLogKind::parser().parse(r#"App::Error LackOf "msg" trailing"#.into()),
            Ok((
                "trailing".into(),
                AppLogKind::Error(AppLogErrorKind::LackOf("msg".into()))
            ))
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            AppLogKind::parser()
                .parse(r#"Sys::Error LackOf "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_missing_colon_in_prefix_fails() {
        assert!(
            AppLogKind::parser()
                .parse(r#"AppError LackOf "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_kind_fails() {
        assert!(
            AppLogKind::parser()
                .parse(r#"App::UnknownKind LackOf "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(AppLogKind::parser().parse("App::".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(AppLogKind::parser().parse("".into()).is_err());
    }
}
