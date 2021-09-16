use super::{Parser, ParserResult, ParserT};

// TODO implement sequence macro

// FIXME alternation should take an arbitrary number of parser

/// Tries `main` if the parsing fails, return the result of `other`
pub fn alternation<'a, T>(
    stream: &'a str,
    main: Parser<'a, T>,
    other: Parser<'a, T>,
) -> ParserResult<'a, T> {
    match (main)(stream) {
        Err(_) => (other)(stream),
        result => result,
    }
}

/// Applies a parser as many times as possible, return `Vec` of results
/// (Analogous to a Kleene closure)
pub fn many<'a, T>(stream: &'a str, parser: Parser<'a, T>) -> ParserResult<'a, Vec<T>> {
    fn recursion<'b, U>(
        stream: &'b str,
        parser: Parser<'b, U>,
        mut acc: Vec<U>,
    ) -> ParserResult<'b, Vec<U>> {
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
pub fn some<'a, T>(stream: &'a str, parser: Parser<'a, T>) -> ParserResult<'a, Vec<T>> {
    match (parser)(stream) {
        Ok((value, tail)) => many(tail, parser).map(|(mut vec, tail)| {
            vec.insert(0, value);
            (vec, tail)
        }),
        Err(msg) => Err(format!("No sucessful parse in `some`: {}", msg)),
    }
}
