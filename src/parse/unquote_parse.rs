use crate::parse::Parser;

/// Парсер кавычек
#[derive(Debug, Clone)]
pub struct Unquote;
impl Parser for Unquote {
    type Dest = String;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        do_unquote(input)
    }
}

/// Распарсить строку, которую ранее [обернули в кавычки](quote)
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
    Err(()) // строка кончилась, не закрыв кавычку
}

/// Конструктор [Unquote]
pub fn unquote() -> Unquote {
    Unquote
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unquote() {
        assert_eq!(
            Unquote.parse(r#""411""#.into()),
            Ok(("".into(), "411".into()))
        );
        assert_eq!(Unquote.parse(r#" "411""#.into()), Err(()));
        assert_eq!(Unquote.parse(r#"411"#.into()), Err(()));

        assert_eq!(
            Unquote.parse(r#""ni\\c\"e""#.into()),
            Ok(("".into(), r#"ni\c"e"#.into()))
        );
    }
}
