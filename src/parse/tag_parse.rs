use crate::parse::Parser;

/// Парсер константных строк
/// (аналог `nom::bytes::complete::tag`)
#[derive(Debug, Clone)]
pub struct Tag {
    tag: &'static str,
}
impl Parser for Tag {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((input.strip_prefix(self.tag).ok_or(())?, ()))
    }
}
/// Конструктор [Tag]
pub fn tag(tag: &'static str) -> Tag {
    Tag { tag }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag() {
        assert_eq!(
            tag("key=").parse("key=value".into()),
            Ok(("value".into(), ()))
        );
        assert_eq!(tag("key=").parse("key:value".into()), Err(()));
    }
}
