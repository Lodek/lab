pub type ParserResult<'a, T> = Result<(T, &'a str), String>;

pub type Parser<'a, T> = fn(&'a str) -> ParserResult<'a, T>;
pub type ParserT<'a, T> = Fn(&'a str) -> ParserResult<'a, T>;

mod combinators;
mod parsers;

#[cfg(test)]
mod test {
    use super::*;

    fn digit_letters_parser<'a>(stream: &'a str) -> ParserResult<'a, String> {
        let ((digit, tail)) = parsers::digit(stream)?;
        let ((letters, tail)) = combinators::many(tail, parsers::alphabetic)?;
        let mut token = String::new();
        token.push(digit);
        letters.iter().for_each(|c| token.push(*c));
        Ok((token, tail))
    }

    fn letters_digit_parser<'a>(stream: &'a str) -> ParserResult<'a, String> {
        let ((letters, tail)) = combinators::some(stream, parsers::alphabetic)?;
        let ((digit, tail)) = parsers::digit(tail)?;
        let mut token = String::new();
        letters.iter().for_each(|c| token.push(*c));
        token.push(digit);
        Ok((token, tail))
    }

    fn test_digit_letters_no_digit() {
        let stream = "abc";
        let result = digit_letters_parser(stream);
        assert_eq!(result.is_err(), true);
    }

    fn test_digit_letters_with_digit() {
        let stream = "1abc";
        let result = digit_letters_parser(stream);
        assert_eq!(Ok((String::from("1abc"), "")), result);
    }

    fn test_letters_digit_no_letter() {
        let stream = "1";
        let result = letters_digit_parser(stream);
        assert_eq!(result.is_err(), true);
    }

    fn test_letters_digit_with_letters() {
        let stream = "ab12";
        let result = digit_letters_parser(stream);
        assert_eq!(Ok((String::from("ab1"), "2")), result);
    }

    #[test]
    fn test_alternation() {
        let stream = "1abc1";

        let result = combinators::alternation(stream, letters_digit_parser, digit_letters_parser);

        assert_eq!(Ok((String::from("1abc"), "1")), result);
    }
}
