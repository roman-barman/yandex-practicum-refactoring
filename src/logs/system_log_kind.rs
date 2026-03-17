use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, MapParser, PrecededParser, StripWhitespaceParser, SystemLogErrorKind,
    SystemLogTraceKind, TagParser,
};

/// All types of [system](LogKind) logs
#[derive(Debug, Clone, PartialEq)]
pub enum SystemLogKind {
    Error(SystemLogErrorKind),
    Trace(SystemLogTraceKind),
}

impl Parsable for SystemLogKind {
    type Parser = StripWhitespaceParser<
        PrecededParser<
            TagParser,
            AltConditionParser<(
                MapParser<
                    <SystemLogTraceKind as Parsable>::Parser,
                    fn(SystemLogTraceKind) -> SystemLogKind,
                >,
                MapParser<
                    <SystemLogErrorKind as Parsable>::Parser,
                    fn(SystemLogErrorKind) -> SystemLogKind,
                >,
            )>,
        >,
    >;
    fn parser() -> Self::Parser {
        StripWhitespaceParser::new(PrecededParser::new(
            TagParser::new("System::"),
            AltConditionParser::<(
                MapParser<
                    <SystemLogTraceKind as Parsable>::Parser,
                    fn(SystemLogTraceKind) -> SystemLogKind,
                >,
                MapParser<
                    <SystemLogErrorKind as Parsable>::Parser,
                    fn(SystemLogErrorKind) -> SystemLogKind,
                >,
            )>::new(
                MapParser::new(SystemLogTraceKind::parser(), |trace| {
                    SystemLogKind::Trace(trace)
                }),
                MapParser::new(SystemLogErrorKind::parser(), |error| {
                    SystemLogKind::Error(error)
                }),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{Parser, SystemLogErrorKind, SystemLogTraceKind};

    #[test]
    fn test_trace_send_request() {
        assert_eq!(
            SystemLogKind::parser()
                .parse(r#"System::Trace SendRequest "https://api.example.com""#.into()),
            Ok((
                "".into(),
                SystemLogKind::Trace(SystemLogTraceKind::SendRequest(
                    "https://api.example.com".into()
                ))
            ))
        );
    }

    #[test]
    fn test_trace_get_response() {
        assert_eq!(
            SystemLogKind::parser().parse(r#"System::Trace GetResponse "200 OK""#.into()),
            Ok((
                "".into(),
                SystemLogKind::Trace(SystemLogTraceKind::GetResponse("200 OK".into()))
            ))
        );
    }

    #[test]
    fn test_error_network_error() {
        assert_eq!(
            SystemLogKind::parser()
                .parse(r#"System::Error NetworkError "connection refused""#.into()),
            Ok((
                "".into(),
                SystemLogKind::Error(SystemLogErrorKind::NetworkError(
                    "connection refused".into()
                ))
            ))
        );
    }

    #[test]
    fn test_error_access_denied() {
        assert_eq!(
            SystemLogKind::parser().parse(r#"System::Error AccessDenied "admin only""#.into()),
            Ok((
                "".into(),
                SystemLogKind::Error(SystemLogErrorKind::AccessDenied("admin only".into()))
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_is_stripped() {
        assert_eq!(
            SystemLogKind::parser().parse(r#"   System::Error NetworkError "timeout""#.into()),
            Ok((
                "".into(),
                SystemLogKind::Error(SystemLogErrorKind::NetworkError("timeout".into()))
            ))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            SystemLogKind::parser().parse(r#"System::Trace SendRequest "url" trailing"#.into()),
            Ok((
                "trailing".into(),
                SystemLogKind::Trace(SystemLogTraceKind::SendRequest("url".into()))
            ))
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            SystemLogKind::parser()
                .parse(r#"App::Trace SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_space_after_system_prefix_fails() {
        assert!(
            SystemLogKind::parser()
                .parse(r#"System:: Trace SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_subtype_fails() {
        assert!(
            SystemLogKind::parser()
                .parse(r#"System::Metrics SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(SystemLogKind::parser().parse("".into()).is_err());
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(SystemLogKind::parser().parse("System::".into()).is_err());
    }
}
