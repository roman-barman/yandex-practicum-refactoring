use crate::parse::Parser;
use crate::parse::all_parse::{All, all2};
use crate::parse::delimited_parse::{Delimited, delimited};
use crate::parse::quoted_tag_parse::{QuotedTag, quoted_tag};
use crate::parse::strip_whitespace_parse::{StripWhitespace, strip_whitespace};
use crate::parse::tag_parse::{Tag, tag};

/// Комбинатор, который вытаскивает значения из пары `"ключ":значение,`.
/// Для простоты реализации, запятая всегда нужна в конце пары ключ-значение,
/// простое '"ключ":значение' читаться не будет
#[derive(Debug, Clone)]
pub struct KeyValue<T> {
    parser: Delimited<
        All<(StripWhitespace<QuotedTag>, StripWhitespace<Tag>)>,
        StripWhitespace<T>,
        StripWhitespace<Tag>,
    >,
}
impl<T> Parser for KeyValue<T>
where
    T: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        self.parser.parse(input)
    }
}
/// Конструктор [KeyValue]
pub fn key_value<T: Parser>(key: &'static str, value_parser: T) -> KeyValue<T> {
    KeyValue {
        parser: delimited(
            all2(
                strip_whitespace(quoted_tag(key)),
                strip_whitespace(tag(":")),
            ),
            strip_whitespace(value_parser),
            strip_whitespace(tag(",")),
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::std_parse::U32Parser;

    #[test]
    fn test_key_value() {
        assert_eq!(
            key_value("key", U32Parser).parse(r#""key":32,"#.into()),
            Ok(("".into(), 32))
        );
        assert_eq!(
            key_value("key", U32Parser).parse(r#"key:32,"#.into()),
            Err(())
        );
        assert_eq!(
            key_value("key", U32Parser).parse(r#""key":32"#.into()),
            Err(())
        );
        assert_eq!(
            key_value("key", U32Parser).parse(r#" "key" : 32 , nice"#.into()),
            Ok(("nice".into(), 32))
        );
    }
}
