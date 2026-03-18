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

use crate::logs::LogKind;
pub(crate) use std_parse::*;
pub(crate) use take_parse::TakeParser;

/// Trait to implement and require the 'parse and show what remains to be parsed' method
pub trait Parser {
    type Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()>;
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
