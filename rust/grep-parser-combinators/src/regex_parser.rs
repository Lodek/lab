use parser_combinators::{ParserResult};
use parser_combinators::parsers::{elem, alphanumeric, satisfies, a_char};
use parser_combinators::operators::{many};
use parser_combinators::try_parsers;
use std::collections::HashSet;

use crate::syntax_tree::{Repeater, Factor, Term, Expression, Operator};

const RESERVED_CHARACTERS: &'static str = "()*|";

fn parse_repeater<'a>(stream: &'a str) -> ParserResult<'a, Repeater>  {

    fn parse_any<'a>(stream: &'a str) -> ParserResult<'a, Repeater> {
        a_char(stream, '*').map(|(_, stream)| (Repeater::Any, stream))
    }

    fn parse_some<'a>(stream: &'a str) -> ParserResult<'a, Repeater> {
        a_char(stream, '+').map(|(_, stream)| (Repeater::Some, stream))
    }

    try_parsers!(stream, parse_some, parse_any)
}

fn parse_symbol<'a>(stream: &'a str) -> ParserResult<'a, Factor> {
    satisfies(stream, |c| !RESERVED_CHARACTERS.contains(c))
        .map(|(c, stream)| (Factor::Symbol(c), stream))
}


fn parse_group<'a>(stream: &'a str) -> ParserResult<'a, Factor>  {
    let (_, stream) = a_char(stream, '(')?;
    let (expr, stream) = parse_expression(stream)?;
    let (_, stream) = a_char(stream, ')')?;
    Ok((Factor::Group(Box::new(expr)), stream))
}


fn parse_factor<'a>(stream: &'a str) -> ParserResult<'a, Factor>  {
    try_parsers!(stream, parse_symbol, parse_group)
}

fn parse_repetition<'a>(stream: &'a str) -> ParserResult<'a, Term> {
    let (factor, stream) = parse_factor(stream)?;
    let (repeater, stream) = parse_repeater(stream)?;
    Ok((Term::Repetition(factor, repeater), stream))
}

fn parse_simple_term<'a>(stream: &'a str) -> ParserResult<'a, Term> {
    let (factor, stream) = parse_factor(stream)?;
    Ok((Term::Term(factor), stream))
}

fn parse_term<'a>(stream: &'a str) -> ParserResult<'a, Term> {
    try_parsers!(stream, parse_repetition, parse_simple_term)
}

fn parse_operator<'a>(stream: &'a str) -> ParserResult<'a, Operator> {
    fn alternation_parser<'a>(stream: &'a str) -> ParserResult<'a, Operator> {
        a_char(stream, '|').map(|(_, stream)| (Operator::Alternation, stream))
    }
    fn concat_parser<'a>(stream: &'a str) -> ParserResult<'a, Operator> {
        Ok((Operator::Concat, stream))
    }
    try_parsers!(stream, alternation_parser, concat_parser)
}

/// Left recurssive expression parsing for regular expression syntax tree
pub fn parse_expression<'a>(stream: &'a str) -> ParserResult<'a, Expression> {

    fn parse_operator_term<'a>(stream: &'a str) -> ParserResult<'a, (Operator, Expression)>  {
        let (op, stream) = parse_operator(stream)?;
        let (term, stream) = parse_term(stream)?;
        let leaf = Expression::Leaf(term);
        Ok(((op, leaf), stream))
    }

    fn combine_results(left: Expression, pair: (Operator, Expression)) -> Expression {
        let (op, right) = pair;
        Expression::Node(Box::new(left), op, Box::new(right))
    }

    let (first, stream) = parse_term(stream)?;
    let first_leaf = Expression::Leaf(first);
    let (results, stream) = many(stream, &parse_operator_term)?;
    let tree = results.into_iter().fold(first_leaf, combine_results);
    Ok((tree, stream))
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_repeater_any() {
        let stream = "*";

        let result = parse_repeater(stream);

        assert_eq!(Ok((Repeater::Any, "")), result);
    }

    #[test]
    fn test_parse_repeater_some() {
        let stream = "+";

        let result = parse_repeater(stream);

        assert_eq!(Ok((Repeater::Some, "")), result);
    }

    #[test]
    fn test_parse_symbol_does_not_parse_reserved_characters() {
        let stream = "()*|";

        let result = parse_symbol(stream);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_parse_group_test() {
        let stream = "(abc)";

        let result = parse_group(stream);

        let expression_tree = Expression::Node(
            Box::new(Expression::Node(
                Box::new(Expression::Leaf(Term::Term(Factor::Symbol('a')))),
                Operator::Concat,
                Box::new(Expression::Leaf(Term::Term(Factor::Symbol('b'))))
            )),
            Operator::Concat,
            Box::new(Expression::Leaf(Term::Term(Factor::Symbol('c'))))
        );
        let tree = Factor::Group(Box::new(expression_tree));

        assert_eq!(Ok((tree, "")), result);
    }

    #[test]
    fn test_parse_concat_expression() {
        let stream = "ab";

        let result = parse_expression(stream);

        let expression = Expression::Node(
            Box::new(Expression::Leaf(Term::Term(Factor::Symbol('a')))),
            Operator::Concat,
            Box::new(Expression::Leaf(Term::Term(Factor::Symbol('b'))))
        );
        assert_eq!(Ok((expression, "")), result);
    }

}
