use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, MapParser, PrecededParser, StripWhitespaceParser, TagParser, UnquoteParser,
};

/// Trace [system](SystemLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum SystemLogTraceKind {
    SendRequest(String),
    GetResponse(String),
}

impl Parsable for SystemLogTraceKind {
    type Parser = PrecededParser<
        TagParser,
        AltConditionParser<(
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> SystemLogTraceKind,
            >,
            MapParser<
                PrecededParser<
                    StripWhitespaceParser<TagParser>,
                    StripWhitespaceParser<UnquoteParser>,
                >,
                fn(String) -> SystemLogTraceKind,
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
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> SystemLogTraceKind,
                >,
                MapParser<
                    PrecededParser<
                        StripWhitespaceParser<TagParser>,
                        StripWhitespaceParser<UnquoteParser>,
                    >,
                    fn(String) -> SystemLogTraceKind,
                >,
            )>::new(
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("SendRequest")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    SystemLogTraceKind::SendRequest,
                ),
                MapParser::new(
                    PrecededParser::new(
                        StripWhitespaceParser::new(TagParser::new("GetResponse")),
                        StripWhitespaceParser::new(UnquoteParser),
                    ),
                    SystemLogTraceKind::GetResponse,
                ),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_send_request() {
        assert_eq!(
            SystemLogTraceKind::parser()
                .parse(r#"Trace SendRequest "https://api.example.com""#.into()),
            Ok((
                "".into(),
                SystemLogTraceKind::SendRequest("https://api.example.com".into())
            ))
        );
    }

    #[test]
    fn test_get_response() {
        assert_eq!(
            SystemLogTraceKind::parser().parse(r#"Trace GetResponse "200 OK""#.into()),
            Ok(("".into(), SystemLogTraceKind::GetResponse("200 OK".into())))
        );
    }

    #[test]
    fn test_multiple_spaces_between_parts() {
        assert_eq!(
            SystemLogTraceKind::parser().parse(r#"Trace   SendRequest   "url""#.into()),
            Ok(("".into(), SystemLogTraceKind::SendRequest("url".into())))
        );
    }

    #[test]
    fn test_no_space_between_trace_and_action() {
        // StripWhitespaceParser strips 0 chars, so no separator is required
        assert_eq!(
            SystemLogTraceKind::parser().parse(r#"TraceSendRequest "url""#.into()),
            Ok(("".into(), SystemLogTraceKind::SendRequest("url".into())))
        );
    }

    #[test]
    fn test_empty_string_value() {
        assert_eq!(
            SystemLogTraceKind::parser().parse(r#"Trace SendRequest """#.into()),
            Ok(("".into(), SystemLogTraceKind::SendRequest("".into())))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            SystemLogTraceKind::parser().parse(r#"Trace SendRequest "url" trailing"#.into()),
            Ok((
                "trailing".into(),
                SystemLogTraceKind::SendRequest("url".into())
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_fails() {
        // Unlike SystemLogKind, there is no StripWhitespaceParser at the top level
        assert!(
            SystemLogTraceKind::parser()
                .parse(r#" Trace SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            SystemLogTraceKind::parser()
                .parse(r#"Error SendRequest "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_action_fails() {
        assert!(
            SystemLogTraceKind::parser()
                .parse(r#"Trace UnknownAction "url""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unquoted_value_fails() {
        assert!(
            SystemLogTraceKind::parser()
                .parse(r#"Trace SendRequest url"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(SystemLogTraceKind::parser().parse("Trace".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(SystemLogTraceKind::parser().parse("".into()).is_err());
    }
}
