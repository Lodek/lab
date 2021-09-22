use super::{Parser, ParserResult, ParserT};
use crate::try_parsers;
use super::operators::many as many_op;
use super::operators::some as some_op;

// why the crap does it work with impl FnOnce?
// Maybe I should use a trait object for the parser?
// There might be an overhead from the monomorphization process?

/// Returns a new parser that applies the original parser as many times as possible
pub fn many<'a, P, T>(parser: P) -> impl FnOnce(&'a str) -> ParserResult<'a, Vec<T>>
where P: Fn(&'a str) -> ParserResult<'a, T>
{
    |stream: &'a str| -> ParserResult<'a, Vec<T>> {
        many_op(stream, parser)
    }
}

/// Returns a new parser that applies the original parser at least once
pub fn some<'a, P, T>(parser: P) -> impl FnOnce(&'a str) -> ParserResult<'a, Vec<T>>
where P: Fn(&'a str) -> ParserResult<'a, T>
{
    |stream: &'a str| -> ParserResult<'a, Vec<T>> {
        some_op(stream, parser)
    }
}

pub fn alternate<'a, P, T>(first: P, second: P) -> impl Fn(&'a str) -> ParserResult<'a, T>
where P: Fn(&'a str) -> ParserResult<'a, T> {
    move |stream| try_parsers!(stream, first, second)
}

pub fn chain<'a, PA, PB, T, U>(first: PA, second: PB) -> impl Fn(&'a str) -> ParserResult<'a, (T, U)> 
where PA: Fn(&'a str) -> ParserResult<'a, T>,
      PB: Fn(&'a str) -> ParserResult<'a, U>,
{
    move |stream| {
        let (first_result, stream) = (first)(stream)?;
        let (second_result, stream) = (second)(stream)?;
        Ok(((first_result, second_result), stream))
    }
}

/// Chain N parsers
// not sure how to implement it though, the return type would be dynamic,
// how could i build a tuple out of the expression expansion.
// maybe i a need proc macro for that
//macro_rules! chainN

/// Alternate N parsers
// Need to look into token tree macros to build alternateN
//macro_rules! alternateN


#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::{elem, digit};
    use crate::alternate;

    #[test]
    fn test_alternate_parser_tries_all_parsers() {
        //let parser = alternate!(digit, digit, elem);
        //let result = parser("abc");
        //assert_eq!(Ok(('a', "bc")), result);
    }
}
