use super::{Parser, ParserResult, ParserT};
use crate::try_parsers;
use super::operators::many as many_op;
use super::operators::some as some_op;


/// Returns a new parser that applies the original parser as many times as possible
pub fn many<P, T>(parser: P) -> impl for <'b> Fn(&'b str) -> ParserResult<'b, Vec<T>>
where for<'a> P: Fn(&'a str) -> ParserResult<'a, T>
{
    move |stream| many_op(stream, &parser)
}

/// Returns a new parser that applies the original parser at least once
pub fn some<P, T>(parser: P) -> impl for<'b> Fn(&'b str) -> ParserResult<'b, Vec<T>>
where for<'a> P: Fn(&'a str) -> ParserResult<'a, T>
{
    move |stream| some_op(stream, &parser)
}


pub fn alternate<PA, PB, T>(first: PA, second: PB) -> impl for<'c> Fn(&'c str) -> ParserResult<'c, T>
where for<'a> PA: Fn(&'a str) -> ParserResult<'a, T>,
      for<'b> PB: Fn(&'b str) -> ParserResult<'b, T> 
{
    move |stream| try_parsers!(stream, first, second)
}

pub fn chain<PA, PB, T, U>(first: PA, second: PB) -> impl for<'a> Fn(&'a str) -> ParserResult<'a, (T, U)> 
where for<'b> PA: Fn(&'b str) -> ParserResult<'b, T>,
      for<'c> PB: Fn(&'c str) -> ParserResult<'c, U>,
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
}
