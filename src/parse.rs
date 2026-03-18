pub(crate) use crate::parse::all_parse::AllConditionParser;
pub(crate) use crate::parse::alt_parse::AltConditionParser;
pub(crate) use crate::parse::delimited_parse::DelimitedParser;
pub(crate) use crate::parse::key_value_parse::KeyValueParser;
pub(crate) use crate::parse::list_parse::ListParser;
pub(crate) use crate::parse::map_parse::MapParser;
pub(crate) use crate::parse::permutation_parse::PermutationParser;
pub(crate) use crate::parse::preceded_parse::PrecededParser;
pub(crate) use crate::parse::strip_whitespace_parse::StripWhitespaceParser;
pub(crate) use crate::parse::tag_parse::TagParser;
pub(crate) use crate::parse::unquote_parse::UnquoteParser;
pub(crate) use std_parse::*;
pub(crate) use take_parse::TakeParser;

mod all_parse;
mod alt_parse;
mod as_is_parse;
mod delimited_parse;
mod key_value_parse;
mod list_parse;
mod map_parse;
mod permutation_parse;
mod preceded_parse;
mod quoted_tag_parse;
mod std_parse;
mod strip_whitespace_parse;
mod tag_parse;
mod take_parse;
mod unquote_parse;

/// Trait to implement and require the 'parse and show what remains to be parsed' method
pub trait Parser {
    type Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()>;
}
