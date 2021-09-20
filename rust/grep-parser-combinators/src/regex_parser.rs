use parser_combinators::{ParserResult};
use parser_combinators::parsers::{elem, alphanumeric, satisfies, a_char};
use parser_combinators::operators::{many, some};
use parser_combinators::try_parsers;
use std::collections::HashSet;

use crate::syntax_tree::{Repeater, Factor, Term, Expression};

const reserved_characters: &'static str = "()*|";

fn parse_any<'a>(stream: &'a str) -> ParserResult<'a, Repeater> {
    a_char(stream, '*').map(|(_, tail)| (Repeater::Any, tail))
}

fn parse_many<'a>(stream: &'a str) -> ParserResult<'a, Repeater> {
    a_char(stream, '+').map(|(_, tail)| (Repeater::Many, tail))
}

fn parse_repeater<'a>(stream: &'a str) -> ParserResult<'a, Repeater>  {
    try_parsers!(stream, parse_many, parse_any)
}

fn parse_symbol<'a>(stream: &'a str) -> ParserResult<'a, Factor> {
    satisfies(stream, |c| !reserved_characters.contains(c))
        .map(|(c, tail)| (Factor::Symbol(c), tail))
}


fn parse_group<'a>(stream: &'a str) -> ParserResult<'a, Factor>  {
    let (_, tail) = a_char(stream, '(')?;
    //let (expr, tail) = parse_expression(tail)?;
    let (_, tail) = a_char(stream, ')')?;
    //Ok((Factor::Group(expr), tail))
    Ok((Factor::Group(Box::new(Expression::Singleton(Term::Term(Factor::Symbol('a'))))), tail))
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

// how do with expression?
