use crate::parse::tag_parse::{Tag, tag};
use crate::parse::{Parser, do_unquote_non_escaped};

/// Парсер [тэга](Tag), обёрнутого в кавычки
#[derive(Debug, Clone)]
pub struct QuotedTag(Tag);
impl Parser for QuotedTag {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, candidate) = do_unquote_non_escaped(input)?;
        if !self.0.parse(candidate)?.0.is_empty() {
            return Err(());
        }
        Ok((remaining, ()))
    }
}
/// Конструктор [QuotedTag]
pub fn quoted_tag(tag_value: &'static str) -> QuotedTag {
    QuotedTag(tag(tag_value))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quoted_tag() {
        assert_eq!(
            quoted_tag("key").parse(r#""key"=value"#.into()),
            Ok(("=value".into(), ()))
        );
        assert_eq!(quoted_tag("key").parse(r#""key:"value"#.into()), Err(()));
        assert_eq!(quoted_tag("key").parse(r#"key=value"#.into()), Err(()));
    }
}
