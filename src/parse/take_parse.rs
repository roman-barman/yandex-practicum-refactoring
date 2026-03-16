use crate::parse::Parser;

/// A combinator for applying a child parser N times (analogous to take from nom)
pub struct TakeParser<T> {
    count: usize,
    parser: T,
}

impl<T> TakeParser<T> {
    pub fn new(count: usize, parser: T) -> Self {
        TakeParser { count, parser }
    }
}

impl<T: Parser> Parser for TakeParser<T> {
    type Dest = Vec<T::Dest>;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let mut remaining = input;
        let mut result = Vec::with_capacity(self.count);
        for _ in 0..self.count {
            let (new_remaining, new_result) = self.parser.parse(remaining)?;
            result.push(new_result);
            remaining = new_remaining;
        }
        Ok((remaining, result))
    }
}

#[cfg(test)]
mod tests {
    use super::TakeParser;
    use crate::parse::Parser;
    use crate::parse::std_parse::ByteParser;

    #[test]
    fn test_count_3_full_input() {
        assert_eq!(
            TakeParser::new(3, ByteParser).parse("aabbcc"),
            Ok(("", vec![0xaa, 0xbb, 0xcc]))
        );
    }

    #[test]
    fn test_count_0_returns_empty_vec() {
        assert_eq!(
            TakeParser::new(0, ByteParser).parse("aabb"),
            Ok(("aabb", vec![]))
        );
    }

    #[test]
    fn test_count_1() {
        assert_eq!(
            TakeParser::new(1, ByteParser).parse("aabb"),
            Ok(("bb", vec![0xaa]))
        );
    }

    #[test]
    fn test_remainder_after_parsing() {
        assert_eq!(
            TakeParser::new(3, ByteParser).parse("aabbccdd"),
            Ok(("dd", vec![0xaa, 0xbb, 0xcc]))
        );
    }

    #[test]
    fn test_count_0_empty_input() {
        assert_eq!(TakeParser::new(0, ByteParser).parse(""), Ok(("", vec![])));
    }

    #[test]
    fn test_count_positive_empty_input() {
        assert_eq!(TakeParser::new(1, ByteParser).parse(""), Err(()));
    }

    #[test]
    fn test_input_shorter_than_count() {
        assert_eq!(TakeParser::new(3, ByteParser).parse("aabb"), Err(()));
    }

    #[test]
    fn test_error_on_first_iteration() {
        assert_eq!(TakeParser::new(3, ByteParser).parse("zzbbcc"), Err(()));
    }

    #[test]
    fn test_error_on_middle_iteration() {
        assert_eq!(TakeParser::new(3, ByteParser).parse("aazzcc"), Err(()));
    }

    #[test]
    fn test_error_on_last_iteration() {
        assert_eq!(TakeParser::new(3, ByteParser).parse("aabbzz"), Err(()));
    }
}
