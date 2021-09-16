
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
        fn parser<'a>(stream: &'a str) -> ParserResult<'a, String> {
            let ((digit, tail)) = parsers::digit(stream)?;
            let ((letters, tail)) = combinators::many(tail, parsers::alphabetic)?;
            let mut token = String::new();
            token.push(digit);
            letters.iter().for_each(|c| token.push(*c));
            Ok((token, tail))
        }

        let stream = "1abc1";

        let parser_result = parser(stream);

        assert_eq!(Ok((String::from("1abc"), "1")), parser_result);
    }


}
