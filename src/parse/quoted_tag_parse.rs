use crate::parse::Parser;
use crate::parse::tag_parse::TagParser;

/// Parser for a [tag](Tag) wrapped in quotes
#[derive(Debug, Clone)]
pub struct QuotedTagParser(TagParser);

impl QuotedTagParser {
    pub fn new(tag_value: &'static str) -> Self {
        QuotedTagParser(TagParser::new(tag_value))
    }
}

impl Parser for QuotedTagParser {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, candidate) = do_unquote_non_escaped(input)?;
        if !self.0.parse(candidate)?.0.is_empty() {
            return Err(());
        }
        Ok((remaining, ()))
    }
}

/// Parse a quoted string (a shortened version of do_unquote that doesn't allow for nested quotes)
fn do_unquote_non_escaped(input: &str) -> Result<(&str, &str), ()> {
    let input = input.strip_prefix("\"").ok_or(())?;
    let quote_byte_idx = input.find('"').ok_or(())?;
    if 0 == quote_byte_idx || Some("\\") == input.get(quote_byte_idx - 1..quote_byte_idx) {
        return Err(());
    }
    Ok((&input[1 + quote_byte_idx..], &input[..quote_byte_idx]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quoted_tag() {
        assert_eq!(
            QuotedTagParser::new("key").parse(r#""key"=value"#.into()),
            Ok(("=value".into(), ()))
        );
        assert_eq!(
            QuotedTagParser::new("key").parse(r#""key:"value"#.into()),
            Err(())
        );
        assert_eq!(
            QuotedTagParser::new("key").parse(r#"key=value"#.into()),
            Err(())
        );
    }

    #[test]
    fn test_do_unquote_non_escaped() {
        assert_eq!(
            do_unquote_non_escaped(r#""411""#.into()),
            Ok(("".into(), "411".into()))
        );
        assert_eq!(do_unquote_non_escaped(r#" "411""#.into()), Err(()));
        assert_eq!(do_unquote_non_escaped(r#"411"#.into()), Err(()));
    }
}
