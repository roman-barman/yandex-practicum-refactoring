use crate::parse::Parser;

/// A combinator that requires all child parsers to run,
/// (analogous to `all` from `nom`)
#[derive(Debug, Clone)]
pub struct AllConditionParser<T> {
    parser: T,
}

impl<A0, A1> AllConditionParser<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    pub fn new(a0: A0, a1: A1) -> Self {
        AllConditionParser { parser: (a0, a1) }
    }
}

impl<A0, A1> Parser for AllConditionParser<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    type Dest = (A0::Dest, A1::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        self.parser
            .1
            .parse(remaining)
            .map(|(remaining, a1)| (remaining, (a0, a1)))
    }
}

impl<A0, A1, A2> AllConditionParser<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    #[allow(dead_code)]
    pub fn new(a0: A0, a1: A1, a2: A2) -> Self {
        AllConditionParser {
            parser: (a0, a1, a2),
        }
    }
}

impl<A0, A1, A2> Parser for AllConditionParser<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        let (remaining, a1) = self.parser.1.parse(remaining)?;
        self.parser
            .2
            .parse(remaining)
            .map(|(remaining, a2)| (remaining, (a0, a1, a2)))
    }
}

impl<A0, A1, A2, A3> AllConditionParser<(A0, A1, A2, A3)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
    A3: Parser,
{
    #[allow(dead_code)]
    pub fn new(a0: A0, a1: A1, a2: A2, a3: A3) -> Self {
        AllConditionParser {
            parser: (a0, a1, a2, a3),
        }
    }
}

impl<A0, A1, A2, A3> Parser for AllConditionParser<(A0, A1, A2, A3)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
    A3: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest, A3::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        let (remaining, a1) = self.parser.1.parse(remaining)?;
        let (remaining, a2) = self.parser.2.parse(remaining)?;
        self.parser
            .3
            .parse(remaining)
            .map(|(remaining, a3)| (remaining, (a0, a1, a2, a3)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::std_parse::U32Parser;
    use crate::parse::tag_parse::{Tag, tag};

    // --- 2-tuple ---

    #[test]
    fn test_two_parsers_success_empty_remainder() {
        let parser = AllConditionParser::<(Tag, U32Parser)>::new(tag("hello"), U32Parser);
        assert_eq!(parser.parse("hello42"), Ok(("", ((), 42))));
    }

    #[test]
    fn test_two_parsers_success_with_remainder() {
        let parser = AllConditionParser::<(Tag, U32Parser)>::new(tag("hello"), U32Parser);
        assert_eq!(parser.parse("hello42rest"), Ok(("rest", ((), 42))));
    }

    #[test]
    fn test_two_parsers_remainder_passed_correctly() {
        let parser = AllConditionParser::<(Tag, Tag)>::new(tag("prefix"), tag("suffix"));
        assert_eq!(parser.parse("prefixsuffix"), Ok(("", ((), ()))));
        assert_eq!(parser.parse("prefixsuffixEXTRA"), Ok(("EXTRA", ((), ()))));
    }

    #[test]
    fn test_two_parsers_first_fails() {
        let parser = AllConditionParser::<(Tag, U32Parser)>::new(tag("hello"), U32Parser);
        assert_eq!(parser.parse("world42"), Err(()));
    }

    #[test]
    fn test_two_parsers_second_fails() {
        let parser = AllConditionParser::<(Tag, U32Parser)>::new(tag("hello"), U32Parser);
        assert_eq!(parser.parse("helloworld"), Err(()));
    }

    #[test]
    fn test_two_parsers_empty_input() {
        let parser = AllConditionParser::<(Tag, U32Parser)>::new(tag("hello"), U32Parser);
        assert_eq!(parser.parse(""), Err(()));
    }

    #[test]
    fn test_two_parsers_first_consumes_all_second_gets_empty() {
        let parser = AllConditionParser::<(Tag, U32Parser)>::new(tag("hello"), U32Parser);
        assert_eq!(parser.parse("hello"), Err(()));
    }

    // --- 3-tuple ---

    #[test]
    fn test_three_parsers_success_empty_remainder() {
        let parser = AllConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("abc"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn test_three_parsers_success_with_remainder() {
        let parser =
            AllConditionParser::<(Tag, Tag, U32Parser)>::new(tag("a"), tag("b"), U32Parser);
        assert_eq!(parser.parse("ab99rest"), Ok(("rest", ((), (), 99))));
    }

    #[test]
    fn test_three_parsers_first_fails() {
        let parser = AllConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("xbc"), Err(()));
    }

    #[test]
    fn test_three_parsers_second_fails() {
        let parser = AllConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("axc"), Err(()));
    }

    #[test]
    fn test_three_parsers_third_fails() {
        let parser = AllConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("abx"), Err(()));
    }

    #[test]
    fn test_three_parsers_empty_input() {
        let parser = AllConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse(""), Err(()));
    }

    // --- 4-tuple ---

    #[test]
    fn test_four_parsers_success_empty_remainder() {
        let parser =
            AllConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("abcd"), Ok(("", ((), (), (), ()))));
    }

    #[test]
    fn test_four_parsers_success_with_remainder() {
        let parser = AllConditionParser::<(Tag, Tag, Tag, U32Parser)>::new(
            tag("a"),
            tag("b"),
            tag("c"),
            U32Parser,
        );
        assert_eq!(parser.parse("abc5rest"), Ok(("rest", ((), (), (), 5))));
    }

    #[test]
    fn test_four_parsers_first_fails() {
        let parser =
            AllConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("xbcd"), Err(()));
    }

    #[test]
    fn test_four_parsers_second_fails() {
        let parser =
            AllConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("axcd"), Err(()));
    }

    #[test]
    fn test_four_parsers_third_fails() {
        let parser =
            AllConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("abxd"), Err(()));
    }

    #[test]
    fn test_four_parsers_fourth_fails() {
        let parser =
            AllConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("abcx"), Err(()));
    }

    #[test]
    fn test_four_parsers_empty_input() {
        let parser =
            AllConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse(""), Err(()));
    }
}
