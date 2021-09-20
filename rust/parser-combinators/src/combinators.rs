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

#[macro_export]
macro_rules! alternate {
    ($($parsers: tt),+) => {
        {
            |stream: &'a str| -> ParserResult<'a, T> {
                try_parsers(stream, $parsers)
            }
        }
    }
}


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
