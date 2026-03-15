use crate::parse::Parser;
use std::num::NonZeroI32;

/// Signed numbers
#[derive(Debug)]
#[allow(dead_code)]
pub struct I32Parser;
impl Parser for I32Parser {
    type Dest = i32;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let end_idx = input
            .char_indices()
            .skip(1)
            .find_map(|(idx, c)| (!c.is_ascii_digit()).then_some(idx))
            .unwrap_or(input.len());
        let value = input[..end_idx].parse().map_err(|_| ())?;
        Ok((&input[end_idx..], NonZeroI32::new(value).ok_or(())?.get()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i32() {
        assert_eq!(I32Parser.parse("411".into()), Ok(("".into(), 411)));
        assert_eq!(I32Parser.parse("411ab".into()), Ok(("ab".into(), 411)));
        assert_eq!(I32Parser.parse("".into()), Err(()));
        assert_eq!(I32Parser.parse("-3".into()), Ok(("".into(), -3)));
        assert_eq!(I32Parser.parse("0x03".into()), Err(()));
        assert_eq!(I32Parser.parse("-".into()), Err(()));
        assert_eq!(I32Parser.parse("0".into()), Err(()));
    }
}
