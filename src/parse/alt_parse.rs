use crate::parse::Parser;

/// A combinator that will return the result that will be successful
/// received by the first of the child combinators
/// (analogous to `alt` from `nom`)
#[derive(Debug, Clone)]
pub struct AltConditionParser<T> {
    parser: T,
}

impl<A0, A1, Dest> AltConditionParser<(A0, A1)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
{
    pub fn new(a0: A0, a1: A1) -> Self {
        AltConditionParser { parser: (a0, a1) }
    }
}

impl<A0, A1, Dest> Parser for AltConditionParser<(A0, A1)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        self.parser.1.parse(input)
    }
}

impl<A0, A1, A2, Dest> AltConditionParser<(A0, A1, A2)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
{
    pub fn new(a0: A0, a1: A1, a2: A2) -> Self {
        AltConditionParser {
            parser: (a0, a1, a2),
        }
    }
}

impl<A0, A1, A2, Dest> Parser for AltConditionParser<(A0, A1, A2)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        // match вместо тут не подойдёт - нужно лениво
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.1.parse(input) {
            return Ok(ok);
        }
        self.parser.2.parse(input)
    }
}

impl<A0, A1, A2, A3, Dest> AltConditionParser<(A0, A1, A2, A3)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
{
    pub fn new(a0: A0, a1: A1, a2: A2, a3: A3) -> Self {
        AltConditionParser {
            parser: (a0, a1, a2, a3),
        }
    }
}

impl<A0, A1, A2, A3, Dest> Parser for AltConditionParser<(A0, A1, A2, A3)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.1.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.2.parse(input) {
            return Ok(ok);
        }
        self.parser.3.parse(input)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, Dest> AltConditionParser<(A0, A1, A2, A3, A4, A5, A6, A7)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
    A4: Parser<Dest = Dest>,
    A5: Parser<Dest = Dest>,
    A6: Parser<Dest = Dest>,
    A7: Parser<Dest = Dest>,
{
    pub fn new(a0: A0, a1: A1, a2: A2, a3: A3, a4: A4, a5: A5, a6: A6, a7: A7) -> Self {
        AltConditionParser {
            parser: (a0, a1, a2, a3, a4, a5, a6, a7),
        }
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, Dest> Parser
    for AltConditionParser<(A0, A1, A2, A3, A4, A5, A6, A7)>
where
    A0: Parser<Dest = Dest>,
    A1: Parser<Dest = Dest>,
    A2: Parser<Dest = Dest>,
    A3: Parser<Dest = Dest>,
    A4: Parser<Dest = Dest>,
    A5: Parser<Dest = Dest>,
    A6: Parser<Dest = Dest>,
    A7: Parser<Dest = Dest>,
{
    type Dest = Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        if let Ok(ok) = self.parser.0.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.1.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.2.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.3.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.4.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.5.parse(input) {
            return Ok(ok);
        }
        if let Ok(ok) = self.parser.6.parse(input) {
            return Ok(ok);
        }
        self.parser.7.parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::tag_parse::{Tag, tag};

    // --- 2-tuple ---

    #[test]
    fn test_two_first_matches() {
        let parser = AltConditionParser::<(Tag, Tag)>::new(tag("foo"), tag("bar"));
        assert_eq!(parser.parse("foorest"), Ok(("rest", ())));
    }

    #[test]
    fn test_two_second_matches() {
        let parser = AltConditionParser::<(Tag, Tag)>::new(tag("foo"), tag("bar"));
        assert_eq!(parser.parse("barrest"), Ok(("rest", ())));
    }

    #[test]
    fn test_two_all_fail() {
        let parser = AltConditionParser::<(Tag, Tag)>::new(tag("foo"), tag("bar"));
        assert_eq!(parser.parse("baz"), Err(()));
    }

    #[test]
    fn test_two_first_has_priority() {
        // "fo" is a prefix of "foo", so first parser wins with "obar" remaining
        let parser = AltConditionParser::<(Tag, Tag)>::new(tag("fo"), tag("foo"));
        assert_eq!(parser.parse("foobar"), Ok(("obar", ())));
    }

    #[test]
    fn test_two_full_input_consumed() {
        let parser = AltConditionParser::<(Tag, Tag)>::new(tag("foo"), tag("bar"));
        assert_eq!(parser.parse("foo"), Ok(("", ())));
    }

    #[test]
    fn test_two_empty_input() {
        let parser = AltConditionParser::<(Tag, Tag)>::new(tag("foo"), tag("bar"));
        assert_eq!(parser.parse(""), Err(()));
    }

    // --- 3-tuple ---

    #[test]
    fn test_three_only_last_matches() {
        let parser = AltConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("crest"), Ok(("rest", ())));
    }

    #[test]
    fn test_three_second_matches_skips_third() {
        let parser = AltConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("brest"), Ok(("rest", ())));
    }

    #[test]
    fn test_three_all_fail() {
        let parser = AltConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse("x"), Err(()));
    }

    #[test]
    fn test_three_empty_input() {
        let parser = AltConditionParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(parser.parse(""), Err(()));
    }

    // --- 4-tuple ---

    #[test]
    fn test_four_only_last_matches() {
        let parser =
            AltConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("drest"), Ok(("rest", ())));
    }

    #[test]
    fn test_four_third_matches() {
        let parser =
            AltConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("crest"), Ok(("rest", ())));
    }

    #[test]
    fn test_four_all_fail() {
        let parser =
            AltConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse("x"), Err(()));
    }

    #[test]
    fn test_four_empty_input() {
        let parser =
            AltConditionParser::<(Tag, Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"), tag("d"));
        assert_eq!(parser.parse(""), Err(()));
    }

    // --- 8-tuple ---

    #[test]
    fn test_eight_only_last_matches() {
        let parser = AltConditionParser::<(Tag, Tag, Tag, Tag, Tag, Tag, Tag, Tag)>::new(
            tag("a"),
            tag("b"),
            tag("c"),
            tag("d"),
            tag("e"),
            tag("f"),
            tag("g"),
            tag("h"),
        );
        assert_eq!(parser.parse("hrest"), Ok(("rest", ())));
    }

    #[test]
    fn test_eight_fifth_matches() {
        let parser = AltConditionParser::<(Tag, Tag, Tag, Tag, Tag, Tag, Tag, Tag)>::new(
            tag("a"),
            tag("b"),
            tag("c"),
            tag("d"),
            tag("e"),
            tag("f"),
            tag("g"),
            tag("h"),
        );
        assert_eq!(parser.parse("erest"), Ok(("rest", ())));
    }

    #[test]
    fn test_eight_all_fail() {
        let parser = AltConditionParser::<(Tag, Tag, Tag, Tag, Tag, Tag, Tag, Tag)>::new(
            tag("a"),
            tag("b"),
            tag("c"),
            tag("d"),
            tag("e"),
            tag("f"),
            tag("g"),
            tag("h"),
        );
        assert_eq!(parser.parse("x"), Err(()));
    }

    #[test]
    fn test_eight_empty_input() {
        let parser = AltConditionParser::<(Tag, Tag, Tag, Tag, Tag, Tag, Tag, Tag)>::new(
            tag("a"),
            tag("b"),
            tag("c"),
            tag("d"),
            tag("e"),
            tag("f"),
            tag("g"),
            tag("h"),
        );
        assert_eq!(parser.parse(""), Err(()));
    }
}
