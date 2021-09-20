use super::{Parser, ParserResult, ParserT};

/// Macro tries every given parser and return the first succesful match
/// or the result of the last parser
#[macro_export]
macro_rules! try_parsers {
    ($stream: expr, $parser:expr, $($parsers:ident),*) => {
        {
            let mut result = ($parser)($stream);
            $(
                if result.is_err() {
                    result = ($parsers)($stream);
                }
            )*
            result
        }
    }
}


/// Applies a parser as many times as possible, return `Vec` of results
/// (Analogous to a Kleene closure)
pub fn many<'a, P, T>(stream: &'a str, parser: P) -> ParserResult<'a, Vec<T>> 
where P: Fn(&'a str) -> ParserResult<'a, T>
{
    fn recursion<'a, P, T>(
        stream: &'a str,
        parser: P,
        mut acc: Vec<T>,
    ) -> ParserResult<'a, Vec<T>> 
    where P: Fn(&'a str) -> ParserResult<'a, T>
    {
        if let Ok((value, tail)) = (parser)(stream) {
            acc.push(value);
            return recursion(tail, parser, acc);
        } else {
            return Ok((acc, stream));
        }
    }

    let mut results = Vec::new();
    recursion(stream, parser, results)
}

/// Applies a parser as many times as possible, but at *least* once, return `Vec` of results
pub fn some<'a, P, T>(stream: &'a str, parser: P) -> ParserResult<'a, Vec<T>> 
where P: Fn(&'a str) -> ParserResult<'a, T>
{
    match (parser)(stream) {
        Ok((value, tail)) => many(tail, parser).map(|(mut vec, tail)| {
            vec.insert(0, value);
            (vec, tail)
        }),
        Err(msg) => Err(format!("No sucessful parse in `some`: {}", msg)),
    }
}
