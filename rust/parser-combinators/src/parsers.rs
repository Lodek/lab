use super::{Parser, ParserResult};

/// Base parser, returns one character from the stream
/// if it's not empty, otherwise return an error
pub fn elem<'a>(stream: &'a str) -> ParserResult<'a, char> {
    let mut chars = stream.chars();
    match chars.next() {
        None => Err(String::from("Empty stream")),
        Some(c) => Ok((c, &stream[1..])),
    }
}

pub fn satisfies<'a, F>(stream: &'a str, predicate: F) -> ParserResult<'a, char>
where
    F: Fn(char) -> bool,
{
    match elem(stream) {
        Err(msg) => Err(msg),
        Ok((c, tail)) => match predicate(c) {
            true => Ok((c, tail)),
            _ => Err(String::from("Predicate not satisfied")),
        },
    }
}

pub fn a_char<'a>(stream: &'a str, c: char) -> ParserResult<'a, char> {
    match elem(stream) {
        Err(msg) => Err(msg),
        Ok(('c', tail)) => Ok((c, tail)),
        _ => Err(String::from("Character not found")),
    }
}

pub fn digit<'a>(stream: &'a str) -> ParserResult<'a, char> {
    satisfies(stream, |c| c.is_ascii_digit())
}

pub fn alphabetic<'a>(stream: &'a str) -> ParserResult<'a, char> {
    satisfies(stream, |c| c.is_ascii_alphabetic())
}

pub fn alphanumeric<'a>(stream: &'a str) -> ParserResult<'a, char> {
    satisfies(stream, |c| c.is_ascii_alphanumeric())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alphabetic_consumes_letter() {
        let stream = "abc";

        let parser_result = alphabetic(stream);

        assert_eq!(Ok(('a', "bc")), parser_result);
    }

    #[test]
    fn test_alphabetic_does_not_consume_digit() {
        let stream = "1abc";

        let parser_result = alphabetic(stream);

        assert_eq!(parser_result.is_err(), true);
    }

    #[test]
    fn test_alphabetic_on_empty_stram() {
        let stream = "";

        let parser_result = alphabetic(stream);

        assert_eq!(parser_result.is_err(), true);
    }
}
