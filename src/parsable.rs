use crate::parse::Parser;

/// An auxiliary trait for writing your own deserializer
/// (for the task at hand, it's a distant analogue of serde::Deserialize)
pub trait Parsable: Sized {
    type Parser: Parser<Dest = Self>;
    fn parser() -> Self::Parser;
}
