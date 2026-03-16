use crate::parsable::Parsable;
use crate::parse::{AltConditionParser, DelimitedParser, MapParser, TagParser, UnquoteParser};

/// Status that can be parsed
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    Err(String),
}
impl Parsable for Status {
    type Parser = AltConditionParser<(
        MapParser<TagParser, fn(()) -> Self>,
        MapParser<DelimitedParser<TagParser, UnquoteParser, TagParser>, fn(String) -> Self>,
    )>;
    fn parser() -> Self::Parser {
        fn to_ok(_: ()) -> Status {
            Status::Ok
        }
        fn to_err(error: String) -> Status {
            Status::Err(error)
        }
        AltConditionParser::<(
            MapParser<TagParser, fn(()) -> Self>,
            MapParser<DelimitedParser<TagParser, UnquoteParser, TagParser>, fn(String) -> Self>,
        )>::new(
            MapParser::new(TagParser::new("Ok"), to_ok),
            MapParser::new(
                DelimitedParser::new(TagParser::new("Err("), UnquoteParser, TagParser::new(")")),
                to_err,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Parser;

    // --- Status::Ok ---

    #[test]
    fn test_ok_parses_ok() {
        assert_eq!(Status::parser().parse("Ok"), Ok(("", Status::Ok)));
    }

    #[test]
    fn test_ok_leaves_remainder() {
        assert_eq!(Status::parser().parse("Ok rest"), Ok((" rest", Status::Ok)));
    }

    // --- Status::Err ---

    #[test]
    fn test_err_parses_simple_message() {
        assert_eq!(
            Status::parser().parse(r#"Err("some error")"#),
            Ok(("", Status::Err("some error".to_string())))
        );
    }

    #[test]
    fn test_err_leaves_remainder() {
        assert_eq!(
            Status::parser().parse(r#"Err("some error") rest"#),
            Ok((" rest", Status::Err("some error".to_string())))
        );
    }

    #[test]
    fn test_err_empty_message() {
        assert_eq!(
            Status::parser().parse(r#"Err("")"#),
            Ok(("", Status::Err("".to_string())))
        );
    }

    #[test]
    fn test_err_escaped_quotes_in_message() {
        assert_eq!(
            Status::parser().parse(r#"Err("with \"quotes\"")"#),
            Ok(("", Status::Err(r#"with "quotes""#.to_string())))
        );
    }

    #[test]
    fn test_err_escaped_backslash_in_message() {
        assert_eq!(
            Status::parser().parse(r#"Err("with \\backslash")"#),
            Ok(("", Status::Err(r#"with \backslash"#.to_string())))
        );
    }

    // --- Негативные кейсы ---

    #[test]
    fn test_empty_input_fails() {
        assert_eq!(Status::parser().parse(""), Err(()));
    }

    #[test]
    fn test_lowercase_ok_fails() {
        assert_eq!(Status::parser().parse("ok"), Err(()));
    }

    #[test]
    fn test_uppercase_ok_fails() {
        assert_eq!(Status::parser().parse("OK"), Err(()));
    }

    #[test]
    fn test_err_without_parens_fails() {
        assert_eq!(Status::parser().parse("Err"), Err(()));
    }

    #[test]
    fn test_err_unquoted_message_fails() {
        assert_eq!(Status::parser().parse("Err(no quotes)"), Err(()));
    }

    #[test]
    fn test_err_missing_closing_paren_fails() {
        assert_eq!(Status::parser().parse(r#"Err("msg""#), Err(()));
    }

    #[test]
    fn test_err_missing_opening_quote_fails() {
        assert_eq!(Status::parser().parse("Err()"), Err(()));
    }

    #[test]
    fn test_garbage_input_fails() {
        assert_eq!(Status::parser().parse("Status::Ok"), Err(()));
    }
}
