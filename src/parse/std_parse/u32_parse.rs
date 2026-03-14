use crate::parse::Parser;
use std::num::NonZeroU32;

/// Unsigned numbers
#[derive(Debug)]
pub struct U32Parser;
impl Parser for U32Parser {
    type Dest = u32;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, is_hex) = input
            .strip_prefix("0x")
            .map_or((input, false), |remaining| (remaining, true));
        let end_idx = remaining
            .char_indices()
            .find_map(|(idx, c)| match (is_hex, c) {
                (true, 'a'..='f' | '0'..='9' | 'A'..='F') => None,
                (false, '0'..='9') => None,
                _ => Some(idx),
            })
            .unwrap_or(remaining.len());
        let value = u32::from_str_radix(&remaining[..end_idx], if is_hex { 16 } else { 10 })
            .map_err(|_| ())?;
        Ok((
            &remaining[end_idx..],
            NonZeroU32::new(value).ok_or(())?.get(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u32() {
        assert_eq!(U32Parser.parse("411".into()), Ok(("".into(), 411)));
        assert_eq!(U32Parser.parse("411ab".into()), Ok(("ab".into(), 411)));
        assert_eq!(U32Parser.parse("".into()), Err(()));
        assert_eq!(U32Parser.parse("-3".into()), Err(()));
        assert_eq!(U32Parser.parse("0x03".into()), Ok(("".into(), 0x3)));
        assert_eq!(U32Parser.parse("0x03abg".into()), Ok(("g".into(), 0x3ab)));
        assert_eq!(U32Parser.parse("0x".into()), Err(()));
        assert_eq!(U32Parser.parse("0".into()), Err(()));
    }
}
