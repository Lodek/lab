use super::{Parser, ParserResult, ParserT};


pub fn sequence<'a, P, T>(stream: &'a str, parsers: P) -> ParserResult<'a, Vec<T>>
where P: Iterator<Item=Parser<'a, T>>
{
    fn chaining_function<T>(mut vec: Vec<T>, parsed_value: T) -> Vec<T> {
        vec.push(parsed_value);
        vec
    }

    parsers.fold(Ok((Vec::new(), stream)), |result, parser| {
        chain(result, parser, chaining_function)
    })
}


// FIXME alternation should take an arbitrary number of parser
pub fn alternation<'a, T>(stream: &'a str, main: Parser<'a, T>, other: Parser<'a, T>) -> ParserResult<'a, T> {
    match (main)(stream) {
        Err(_) => (other)(stream),
        result => result,
    }
}

pub fn chain<'a, T, U, F, V>(parser_result: ParserResult<'a, T>, parser: Parser<'a, U>, combiner: F) -> ParserResult<'a, V>
where F: Fn(T, U) -> V
{
    match parser_result {
        Ok((first, tail)) => {
            match parser(tail) {
                Ok((second, tail)) => Ok((combiner(first, second), tail)),
                Err(msg) => Err(msg)
            }
        }
        Err(msg) => Err(msg)
    }
}

pub fn many<'a, T>(stream: &'a str, parser: Parser<'a, T>) -> ParserResult<'a, Vec<T>> {
    let mut results = Vec::new();
    let mut tail = stream;
    loop {
        match (parser)(stream) {
            Ok((value, rest)) => {
                tail = rest;
                results.push(value);
            },
            _ => break
        }
    }
    Ok((results, tail))
}

pub fn some<'a, T>(stream: &'a str, parser: Parser<'a, T>) -> ParserResult<'a, Vec<T>> {
    match (parser)(stream) {
        Ok((value, tail)) => {
            many(tail, parser).map(|(mut vec, tail)| {
                    vec.insert(0, value);
                    (vec, tail)
            })
        }
        Err(msg) => Err(format!("No sucessful parse in `some`: {}", msg))
    }
}
