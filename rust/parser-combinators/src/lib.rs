
type ParserResult<'a, T> = Result<(T, &'a str), String>;

type Parser<'a, T> = fn(&'a str) -> ParserResult<'a, T>;

/// Base parser, returns one character from the stream
/// if it's not empty, otherwise return an error
pub fn elem<'a>(stream: &'a str) -> ParserResult<'a, char> {
    let mut chars = stream.chars();
    match chars.next() {
        None => Err(String::from("Empty stream")),
        Some(c) => Ok((c, &stream[1..]))
    }
}

pub fn satisfies<'a, F>(stream: &'a str, predicate: F) -> ParserResult<'a, char> 
where F: Fn(char) -> bool 
{
    match elem(stream) {
        Err(msg) => Err(msg),
        Ok((c, tail)) => {
            match predicate(c) {
                true => Ok((c, tail)),
                _ => Err(String::from("Predicate not satisfied"))
            }
        }
    }
}


// Possible solution for sequence: write a macro to accept an arbitrary ammount of arguments

pub fn sequence<'a, T, U>(stream: &'a str, main: Parser<'a, T>, other: Parser<'a, U>) -> ParserResult<'a, (T, U)> {
    let first_result = (main)(stream);
    match (main)(stream) {
        Ok((first, parse_remainder)) => {
            match (other)(parse_remainder) {
                Err(msg) => Err(msg),
                Ok((second, tail)) => Ok(((first, second), tail))
            }
        }
        Err(msg) => Err(msg)
    }
}


// FIXME alternation should take an arbitrary number of parser
pub fn alternation<'a, T>(stream: &'a str, main: Parser<'a, T>, other: Parser<'a, T>) -> ParserResult<'a, T> {
    match (main)(stream) {
        Err(_) => (other)(stream),
        result => result,
    }
}

pub fn map_result<'a, T, U>(parser_result: ParserResult<'a, T>, parser: Parser<'a, U>) -> ParserResult<'a, (T, U)>
{
    match parser_result {
        Ok((first, tail)) => {
            match parser(tail) {
                Ok((second, tail)) => Ok(((first, second), tail)),
                Err(msg) => Err(msg)
            }
        }
        Err(msg) => Err(msg)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_elem() {
        let stream = "abc";

        let result = elem(stream);

        assert_eq!(result, Ok(('a', "bc")));
    }


    #[test]
    fn test_parse_sequencing() {
        let stream = "abc";

        let result = sequence(stream, elem, elem);

        assert_eq!(result, Ok((('a', 'b'), "c")));
    }

    #[test]
    fn test_parse_sequencing() {
        let stream = "abc";

        let result = alternative(stream, elem, elem);

        assert_eq!(result, Ok((('a', 'b'), "c")));
    }
}

