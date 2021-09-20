pub type ParserResult<'a, T> = Result<(T, &'a str), String>;

pub type Parser<'a, T> = fn(&'a str) -> ParserResult<'a, T>;
pub type ParserT<'a, T> = Fn(&'a str) -> ParserResult<'a, T>;

pub mod combinators;
pub mod parsers;
pub mod operators;
