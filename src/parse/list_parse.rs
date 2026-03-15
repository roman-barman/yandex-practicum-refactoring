use crate::parse::Parser;

/// A list combinator of any number of elements to be read by a nested parser.
/// The list boundary is defined by square brackets ([&]).
/// For ease of implementation, each list element must be followed by a comma.
#[derive(Debug, Clone)]
pub struct ListParser<T> {
    parser: T,
}

impl<T> ListParser<T>
where
    T: Parser,
{
    pub fn new(parser: T) -> ListParser<T> {
        ListParser { parser }
    }
}

impl<T> Parser for ListParser<T>
where
    T: Parser,
{
    type Dest = Vec<T::Dest>;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let mut remaining = input.trim_start().strip_prefix('[').ok_or(())?.trim_start();
        let mut result = Vec::new();
        while !remaining.is_empty() {
            match remaining.strip_prefix(']') {
                Some(remaining) => return Ok((remaining.trim_start(), result)),
                None => {
                    let (new_remaining, item) = self.parser.parse(remaining)?;
                    let new_remaining = new_remaining
                        .trim_start()
                        .strip_prefix(',')
                        .ok_or(())?
                        .trim_start();
                    result.push(item);
                    remaining = new_remaining;
                }
            }
        }
        Err(()) // строка кончилась, не закрыв скобку
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::Parser;
    use crate::parse::list_parse::ListParser;
    use crate::parse::std_parse::U32Parser;

    #[test]
    fn test_list() {
        assert_eq!(
            ListParser::new(U32Parser).parse("[1,2,3,4,]".into()),
            Ok(("".into(), vec![1, 2, 3, 4,]))
        );
        assert_eq!(
            ListParser::new(U32Parser).parse(" [ 1 , 2 , 3 , 4 , ] nice".into()),
            Ok(("nice".into(), vec![1, 2, 3, 4,]))
        );
        assert_eq!(ListParser::new(U32Parser).parse("1,2,3,4,".into()), Err(()));
        assert_eq!(
            ListParser::new(U32Parser).parse("[]".into()),
            Ok(("".into(), vec![]))
        );
    }
}
