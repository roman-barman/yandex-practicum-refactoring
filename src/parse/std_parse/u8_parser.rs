use crate::parse::Parser;

/// Hexadecimal bytes (useful for parsing blobs)
#[derive(Debug, Clone)]
pub struct ByteParser;
impl Parser for ByteParser {
    type Dest = u8;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (to_parse, remaining) = input.split_at_checked(2).ok_or(())?;
        if !to_parse.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(());
        }
        let value = u8::from_str_radix(to_parse, 16).map_err(|_| ())?;
        Ok((remaining, value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_byte_parser() {
        assert_eq!(ByteParser.parse("00").unwrap(), ("", 0),);
        assert_eq!(ByteParser.parse("ff").unwrap(), ("", 255),);
        assert_eq!(ByteParser.parse("00xs").unwrap(), ("xs", 0));
        assert_eq!(ByteParser.parse("0"), Err(()));
        assert_eq!(ByteParser.parse(""), Err(()));
    }
}
