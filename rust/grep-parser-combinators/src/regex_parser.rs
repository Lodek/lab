use parser_combinators::{ParserResult};
use parser_combinators::parsers::{elem, alphanumeric, satisfies, a_char};
use parser_combinators::operators::{many, some};
use parser_combinators::try_parsers;
use std::collections::HashSet;

use crate::syntax_tree::{Repeater, Factor, Term, Expression, Operator};

const reserved_characters: &'static str = "()*|";

fn parse_any<'a>(stream: &'a str) -> ParserResult<'a, Repeater> {
    a_char(stream, '*').map(|(_, tail)| (Repeater::Any, tail))
}

fn parse_some<'a>(stream: &'a str) -> ParserResult<'a, Repeater> {
    a_char(stream, '+').map(|(_, tail)| (Repeater::Some, tail))
}

fn parse_repeater<'a>(stream: &'a str) -> ParserResult<'a, Repeater>  {
    try_parsers!(stream, parse_some, parse_any)
}

fn parse_symbol<'a>(stream: &'a str) -> ParserResult<'a, Factor> {
    satisfies(stream, |c| !reserved_characters.contains(c))
        .map(|(c, tail)| (Factor::Symbol(c), tail))
}


fn parse_group<'a>(stream: &'a str) -> ParserResult<'a, Factor>  {
    let (_, tail) = a_char(stream, '(')?;
    let (expr, tail) = parse_expression(tail)?;
    let (_, tail) = a_char(stream, ')')?;
    Ok((Factor::Group(Box::new(expr)), tail))
}


fn parse_factor<'a>(stream: &'a str) -> ParserResult<'a, Factor>  {
    try_parsers!(stream, parse_symbol, parse_group)
}

fn parse_repetition<'a>(stream: &'a str) -> ParserResult<'a, Term> {
    let (factor, tail) = parse_factor(stream)?;
    let (repeater, tail) = parse_repeater(tail)?;
    Ok((Term::Repetition(factor, repeater), tail))
}

fn parse_simple_term<'a>(stream: &'a str) -> ParserResult<'a, Term> {
    let (factor, tail) = parse_factor(stream)?;
    Ok((Term::Term(factor), tail))
}

fn parse_term<'a>(stream: &'a str) -> ParserResult<'a, Term> {
    try_parsers!(stream, parse_repetition, parse_simple_term)
}

fn parse_operator<'a>(stream: &'a str) -> ParserResult<'a, Operator> {
    fn alternation_parser<'a>(stream: &'a str) -> ParserResult<'a, Operator> {
        a_char(stream, '|').map(|(_, tail)| (Operator::Alternation, tail))
    }
    fn concat_parser<'a>(stream: &'a str) -> ParserResult<'a, Operator> {
        Ok((Operator::Concat, stream))
    }
    try_parsers!(stream, alternation_parser, concat_parser)
}

/// Left recurssive expression parsing for regular expression syntax tree
pub fn parse_expression<'a>(stream: &'a str) -> ParserResult<'a, Expression> {

    fn parse_operator_term<'a>(stream: &'a str) -> ParserResult<'a, (Operator, Expression)>  {
        let (op, tail) = parse_operator(stream)?;
        let (term, tail) = parse_term(tail)?;
        let leaf = Expression::Leaf(term);
        Ok(((op, leaf), tail))
    }

    fn combine_results(left: Expression, pair: (Operator, Expression)) -> Expression {
        let (op, right) = pair;
        Expression::Node(Box::new(left), op, Box::new(right))
    }

    let (first, tail) = parse_term(stream)?;
    let first_leaf = Expression::Leaf(first);
    let (results, tail) = many(stream, parse_operator_term)?;
    let tree = results.into_iter().fold(first_leaf, combine_results);
    Ok((tree, tail))
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let regex = "abc(defg)*daf+";

        let (expr, tail) = parse_expression(regex).unwrap();

        println!("{:?}", expr);
        println!("{}", tail);
    }

}
