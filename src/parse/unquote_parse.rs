use crate::parse::Parser;

/// Quote parser
#[derive(Debug, Clone)]
pub struct UnquoteParser;
impl Parser for UnquoteParser {
    type Dest = String;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        do_unquote(input)
    }
}

/// Parse the string that was previously [wrapped in quotes](quote)
// `"abc\"def\\ghi"nice` -> (`abcd"def\ghi`, `nice`)
fn do_unquote(input: &str) -> Result<(&str, String), ()> {
    let mut result = String::new();
    let mut escaped_now = false;
    let mut chars = input.strip_prefix("\"").ok_or(())?.chars();
    while let Some(c) = chars.next() {
        match (c, escaped_now) {
            ('"' | '\\', true) => {
                result.push(c);
                escaped_now = false;
            }
            ('\\', false) => escaped_now = true,
            ('"', false) => return Ok((chars.as_str(), result)),
            (c, _) => {
                result.push(c);
                escaped_now = false;
            }
        }
    }
    Err(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unquote() {
        assert_eq!(
            UnquoteParser.parse(r#""411""#.into()),
            Ok(("".into(), "411".into()))
        );
        assert_eq!(UnquoteParser.parse(r#" "411""#.into()), Err(()));
        assert_eq!(UnquoteParser.parse(r#"411"#.into()), Err(()));

        assert_eq!(
            UnquoteParser.parse(r#""ni\\c\"e""#.into()),
            Ok(("".into(), r#"ni\c"e"#.into()))
        );
    }
}
