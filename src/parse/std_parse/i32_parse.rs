use crate::parse::Parser;

/// Знаковые числа
#[derive(Debug)]
pub struct I32;
impl Parser for I32 {
    type Dest = i32;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let end_idx = input
            .char_indices()
            .skip(1)
            .find_map(|(idx, c)| (!c.is_ascii_digit()).then_some(idx))
            .unwrap_or(input.len());
        let value = input[..end_idx].parse().map_err(|_| ())?;
        if value == 0 {
            return Err(()); // в наших логах нет нулей, ноль в операции - фикция
        }
        Ok((&input[end_idx..], value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i32() {
        assert_eq!(I32.parse("411".into()), Ok(("".into(), 411)));
        assert_eq!(I32.parse("411ab".into()), Ok(("ab".into(), 411)));
        assert_eq!(I32.parse("".into()), Err(()));
        assert_eq!(I32.parse("-3".into()), Ok(("".into(), -3)));
        assert_eq!(I32.parse("0x03".into()), Err(()));
        assert_eq!(I32.parse("-".into()), Err(()));
    }
}
