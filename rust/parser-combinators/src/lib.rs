
pub type ParserResult<'a, T> = Result<(T, &'a str), String>;

pub type Parser<'a, T> = fn(&'a str) -> ParserResult<'a, T>;
pub type ParserT<'a, T> = Fn(&'a str) -> ParserResult<'a, T>;

mod parsers;
mod combinators;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

    }


}
