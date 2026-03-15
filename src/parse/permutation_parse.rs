use crate::parse::Parser;

/// A combinator that returns the results of child parsers
/// if they were applied successively in any order.
/// The result is returned in the order in which the Permutation was constructed
/// (analogous to permutation in nom).
#[derive(Debug, Clone)]
pub struct PermutationParser<T> {
    parsers: T,
}

impl<A0, A1> PermutationParser<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    pub fn new(a0: A0, a1: A1) -> PermutationParser<(A0, A1)> {
        PermutationParser { parsers: (a0, a1) }
    }
}

impl<A0, A1> Parser for PermutationParser<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    type Dest = (A0::Dest, A1::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        match self.parsers.0.parse(input) {
            Ok((remaining, a0)) => self
                .parsers
                .1
                .parse(remaining)
                .map(|(remaining, a1)| (remaining, (a0, a1))),
            Err(()) => self.parsers.1.parse(input).and_then(|(remaining, a1)| {
                self.parsers
                    .0
                    .parse(remaining)
                    .map(|(remaining, a0)| (remaining, (a0, a1)))
            }),
        }
    }
}

impl<A0, A1, A2> PermutationParser<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    pub fn new(a0: A0, a1: A1, a2: A2) -> PermutationParser<(A0, A1, A2)> {
        PermutationParser {
            parsers: (a0, a1, a2),
        }
    }
}

impl<A0, A1, A2> Parser for PermutationParser<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        match self.parsers.0.parse(input) {
            Ok((remaining, a0)) => match self.parsers.1.parse(remaining) {
                Ok((remaining, a1)) => self
                    .parsers
                    .2
                    .parse(remaining)
                    .map(|(remaining, a2)| (remaining, (a0, a1, a2))),
                Err(()) => self.parsers.2.parse(remaining).and_then(|(remaining, a2)| {
                    self.parsers
                        .1
                        .parse(remaining)
                        .map(|(remaining, a1)| (remaining, (a0, a1, a2)))
                }),
            },
            Err(()) => match self.parsers.1.parse(input) {
                Ok((remaining, a1)) => match self.parsers.0.parse(remaining) {
                    Ok((remaining, a0)) => self
                        .parsers
                        .2
                        .parse(remaining)
                        .map(|(remaining, a2)| (remaining, (a0, a1, a2))),
                    Err(()) => self.parsers.2.parse(remaining).and_then(|(remaining, a2)| {
                        self.parsers
                            .0
                            .parse(remaining)
                            .map(|(remaining, a0)| (remaining, (a0, a1, a2)))
                    }),
                },
                Err(()) => self.parsers.2.parse(input).and_then(|(remaining, a2)| {
                    match self.parsers.0.parse(remaining) {
                        Ok((remaining, a0)) => self
                            .parsers
                            .1
                            .parse(remaining)
                            .map(|(remaining, a1)| (remaining, (a0, a1, a2))),
                        Err(()) => self.parsers.1.parse(remaining).and_then(|(remaining, a1)| {
                            self.parsers
                                .0
                                .parse(remaining)
                                .map(|(remaining, a0)| (remaining, (a0, a1, a2)))
                        }),
                    }
                }),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::key_value_parse::KeyValueParser;
    use crate::parse::std_parse::U32Parser;
    use crate::parse::tag_parse::{Tag, tag};
    use crate::parse::unquote_parse::{Unquote, unquote};

    // ── PermutationParser<(A0, A1)> ──────────────────────────────────────

    #[test]
    fn two_parsers_direct_order() {
        let p = PermutationParser::<(Tag, Tag)>::new(tag("a"), tag("b"));
        assert_eq!(p.parse("ab"), Ok(("", ((), ()))));
    }

    #[test]
    fn two_parsers_reversed_order() {
        let p = PermutationParser::<(KeyValueParser<Unquote>, KeyValueParser<U32Parser>)>::new(
            KeyValueParser::new("id", unquote()),
            KeyValueParser::new("count", U32Parser),
        );
        assert_eq!(
            p.parse(r#""count":42, "id":"foo","#),
            Ok(("", ("foo".to_string(), 42)))
        );
    }

    #[test]
    fn two_parsers_both_fail() {
        let p = PermutationParser::<(Tag, Tag)>::new(tag("a"), tag("b"));
        assert_eq!(p.parse("cd"), Err(()));
    }

    #[test]
    fn two_parsers_first_ok_second_fails() {
        let p = PermutationParser::<(Tag, Tag)>::new(tag("a"), tag("b"));
        assert_eq!(p.parse("ac"), Err(()));
    }

    #[test]
    fn two_parsers_second_ok_first_fails() {
        let p = PermutationParser::<(Tag, Tag)>::new(tag("a"), tag("b"));
        assert_eq!(p.parse("bc"), Err(()));
    }

    #[test]
    fn two_parsers_leaves_remaining() {
        let p = PermutationParser::<(Tag, Tag)>::new(tag("a"), tag("b"));
        assert_eq!(p.parse("abXYZ"), Ok(("XYZ", ((), ()))));
    }

    #[test]
    fn two_parsers_empty_input() {
        let p = PermutationParser::<(Tag, Tag)>::new(tag("a"), tag("b"));
        assert_eq!(p.parse(""), Err(()));
    }

    // ── PermutationParser<(A0, A1, A2)> ─────────────────────────────────

    #[test]
    fn three_parsers_order_a0_a1_a2() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("abc"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn three_parsers_order_a0_a2_a1() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("acb"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn three_parsers_order_a1_a0_a2() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("bac"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn three_parsers_order_a1_a2_a0() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("bca"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn three_parsers_order_a2_a0_a1() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("cab"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn three_parsers_order_a2_a1_a0() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("cba"), Ok(("", ((), (), ()))));
    }

    #[test]
    fn three_parsers_result_order_preserved() {
        let p = PermutationParser::<(
            KeyValueParser<Unquote>,
            KeyValueParser<U32Parser>,
            KeyValueParser<Unquote>,
        )>::new(
            KeyValueParser::new("a", unquote()),
            KeyValueParser::new("b", U32Parser),
            KeyValueParser::new("c", unquote()),
        );
        assert_eq!(
            p.parse(r#""c":"z", "b":7, "a":"x","#),
            Ok(("", ("x".to_string(), 7, "z".to_string())))
        );
    }

    #[test]
    fn three_parsers_all_fail() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("xyz"), Err(()));
    }

    #[test]
    fn three_parsers_partial_match() {
        let p = PermutationParser::<(Tag, Tag, Tag)>::new(tag("a"), tag("b"), tag("c"));
        assert_eq!(p.parse("ab"), Err(()));
    }
}
