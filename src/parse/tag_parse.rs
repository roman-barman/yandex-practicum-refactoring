use crate::parse::Parser;

/// Constant string parser (similar to nom::bytes::complete::tag)
#[derive(Debug, Clone)]
pub struct TagParser {
    tag: &'static str,
}

impl TagParser {
    pub fn new(tag: &'static str) -> Self {
        TagParser { tag }
    }
}

impl Parser for TagParser {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((input.strip_prefix(self.tag).ok_or(())?, ()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag() {
        assert_eq!(
            TagParser::new("key=").parse("key=value".into()),
            Ok(("value".into(), ()))
        );
        assert_eq!(TagParser::new("key=").parse("key:value".into()), Err(()));
    }
}
