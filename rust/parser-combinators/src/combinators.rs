use super::{Parser, ParserResult, ParserT};
use super::operators::{many, some};

/// Returns a new parser that applies the original parser as many times as possible
pub fn manyP<'a, P, T>(parser: P) -> impl FnOnce(&'a str) -> ParserResult<'a, Vec<T>>
where P: Fn(&'a str) -> ParserResult<'a, T>
{
    |stream: &'a str| -> ParserResult<'a, Vec<T>> {
        many(stream, parser)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::{elem, digit};

    #[test]
    fn test_alternate_macro_one_param() {
        let result = alternate!("abc", elem,);
        assert_eq!(result, Ok(('a', "bc")));
    }

    #[test]
    fn test_alternate_macro_many_params() {
        let result = alternate!("abc", digit, digit, elem);
        assert_eq!(result, Ok(('a', "bc")));
    }

}
