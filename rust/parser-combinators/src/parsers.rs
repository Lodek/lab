use super::{Parser, ParserResult};

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

pub fn digit<'a>(stream: &'a str) -> ParserResult<'a, char> 
{
    satisfies(stream, |c| c.is_ascii_digit())
}

pub fn alphabetic<'a>(stream: &'a str) -> ParserResult<'a, char> 
{
    satisfies(stream, |c| c.is_ascii_alphabetic())
}

pub fn alphanumeric<'a>(stream: &'a str) -> ParserResult<'a, char> 
{
    satisfies(stream, |c| c.is_ascii_alphanumeric())
}
