use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, MapParser, PrecededParser, StripWhitespaceParser, TagParser, UnquoteParser,
};

/// Error [system](SystemLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum SystemLogErrorKind {
    NetworkError(String),
    AccessDenied(String),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_network_error() {
        assert_eq!(
            SystemLogErrorKind::parser().parse(r#"Error NetworkError "connection refused""#.into()),
            Ok((
                "".into(),
                SystemLogErrorKind::NetworkError("connection refused".into())
            ))
        );
    }

    #[test]
    fn test_access_denied() {
        assert_eq!(
            SystemLogErrorKind::parser().parse(r#"Error AccessDenied "admin only""#.into()),
            Ok((
                "".into(),
                SystemLogErrorKind::AccessDenied("admin only".into())
            ))
        );
    }

    #[test]
    fn test_multiple_spaces_between_parts() {
        assert_eq!(
            SystemLogErrorKind::parser().parse(r#"Error   NetworkError   "msg""#.into()),
            Ok(("".into(), SystemLogErrorKind::NetworkError("msg".into())))
        );
    }

    #[test]
    fn test_no_space_between_error_and_variant() {
        // StripWhitespaceParser strips 0 chars, so no separator is required
        assert_eq!(
            SystemLogErrorKind::parser().parse(r#"ErrorNetworkError "msg""#.into()),
            Ok(("".into(), SystemLogErrorKind::NetworkError("msg".into())))
        );
    }

    #[test]
    fn test_empty_string_value() {
        assert_eq!(
            SystemLogErrorKind::parser().parse(r#"Error NetworkError """#.into()),
            Ok(("".into(), SystemLogErrorKind::NetworkError("".into())))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            SystemLogErrorKind::parser().parse(r#"Error AccessDenied "msg" trailing"#.into()),
            Ok((
                "trailing".into(),
                SystemLogErrorKind::AccessDenied("msg".into())
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_fails() {
        // Unlike SystemLogKind, there is no StripWhitespaceParser at the top level
        assert!(
            SystemLogErrorKind::parser()
                .parse(r#" Error NetworkError "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unquoted_value_fails() {
        assert!(
            SystemLogErrorKind::parser()
                .parse(r#"Error NetworkError connection_refused"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            SystemLogErrorKind::parser()
                .parse(r#"Trace NetworkError "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_variant_fails() {
        assert!(
            SystemLogErrorKind::parser()
                .parse(r#"Error Timeout "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(SystemLogErrorKind::parser().parse("Error".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(SystemLogErrorKind::parser().parse("".into()).is_err());
    }
}
