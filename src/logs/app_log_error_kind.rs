use crate::parsable::Parsable;
use crate::parse::{
    AltConditionParser, MapParser, PrecededParser, StripWhitespaceParser, TagParser, UnquoteParser,
};

/// Error [app](AppLogKind)
#[derive(Debug, Clone, PartialEq)]
pub enum AppLogErrorKind {
    LackOf(String),
    SystemError(String),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    #[test]
    fn test_lack_of() {
        assert_eq!(
            AppLogErrorKind::parser().parse(r#"Error LackOf "some resource""#.into()),
            Ok(("".into(), AppLogErrorKind::LackOf("some resource".into())))
        );
    }

    #[test]
    fn test_system_error() {
        assert_eq!(
            AppLogErrorKind::parser().parse(r#"Error SystemError "disk full""#.into()),
            Ok(("".into(), AppLogErrorKind::SystemError("disk full".into())))
        );
    }

    #[test]
    fn test_multiple_spaces_between_parts() {
        assert_eq!(
            AppLogErrorKind::parser().parse(r#"Error   LackOf   "msg""#.into()),
            Ok(("".into(), AppLogErrorKind::LackOf("msg".into())))
        );
    }

    #[test]
    fn test_no_space_between_error_and_variant() {
        // StripWhitespaceParser strips 0 chars, so no separator is required
        assert_eq!(
            AppLogErrorKind::parser().parse(r#"ErrorLackOf "msg""#.into()),
            Ok(("".into(), AppLogErrorKind::LackOf("msg".into())))
        );
    }

    #[test]
    fn test_empty_string_value() {
        assert_eq!(
            AppLogErrorKind::parser().parse(r#"Error LackOf """#.into()),
            Ok(("".into(), AppLogErrorKind::LackOf("".into())))
        );
    }

    #[test]
    fn test_remaining_input_is_preserved() {
        assert_eq!(
            AppLogErrorKind::parser().parse(r#"Error SystemError "msg" trailing"#.into()),
            Ok((
                "trailing".into(),
                AppLogErrorKind::SystemError("msg".into())
            ))
        );
    }

    #[test]
    fn test_leading_whitespace_fails() {
        // Unlike AppLogKind, there is no StripWhitespaceParser at the top level
        assert!(
            AppLogErrorKind::parser()
                .parse(r#" Error LackOf "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unquoted_value_fails() {
        assert!(
            AppLogErrorKind::parser()
                .parse(r#"Error LackOf disk"#.into())
                .is_err()
        );
    }

    #[test]
    fn test_wrong_prefix_fails() {
        assert!(
            AppLogErrorKind::parser()
                .parse(r#"Trace LackOf "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_unknown_variant_fails() {
        assert!(
            AppLogErrorKind::parser()
                .parse(r#"Error Timeout "msg""#.into())
                .is_err()
        );
    }

    #[test]
    fn test_prefix_only_fails() {
        assert!(AppLogErrorKind::parser().parse("Error".into()).is_err());
    }

    #[test]
    fn test_empty_input_fails() {
        assert!(AppLogErrorKind::parser().parse("".into()).is_err());
    }
}
