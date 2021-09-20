use parser_combinator::{ParserResult};
use parser_combinator::parsers::{elem, alphanumeric, satisfies, a_char};
use parser_combinator::combinators::{many, alternation, some};
use std::collections::HashSet;

use crate::syntax_tree::{Repeater, Factor, Term, Expression};

const reserved_characters = "()*|";

fn parse_any<'a>(stream: &'a str) -> ParseResult<'a, Repeater> {
    a_char(stream, '*').map(|_| Repeater::Any)
}

fn parse_many<'a>(stream: &'a str) -> ParseResult<'a, Repeater> {
    a_char(stream, '+').map(|_| Repeater::Many)
}

fn parse_repeater<'a>(stream: &'a str) -> ParseResult<'a, Repeater>  {
    alternation(stream, parse_many, parse_any)
}

fn parse_symbol<'a>(stream: &'a str) -> ParseResult<'a, Factor> {
    satisfies(stream, |c| !reserved_characters.contains(c))
        .map(|c| Factor::Symbol(c))
}

fn parse_group<'a>(stream: &'a str) -> ParseResult<'a, Factor>  {
    let (_, tail) = a_char(stream, '(')?;
    let (expr, tail) = parse_expression(tail)?;
    let (_, tail) = a_char(stream, ')')?;
    Ok((Factor::Group(expr), tail))
}

fn parse_factor<'a>(stream: &'a str) -> ParseResult<'a, Factor>  {
    alternation(stream, parse_symbol, parse_group)
}

fn parse_repetition<'a>(stream: &'a str) -> ParseResult<'a, Term> {
    let (factor, tail) = parse_factor(stream)?;
    let (repetition, tail) = parse_repetition(tail);
    Ok((Term::Repetition(factor, repetition), tail))
}

fn parse_simple_term<'a>(stream: &'a str) -> ParseResult<'a, Term> {
    let (factor, tail) = parse_factor(stream)?;
    Ok((Term::Term(factor), tail))
}

fn parse_term<'a>(stream: &'a str) -> ParseResult<'a, Term> {
    alternation(stream, parse_repetition, parse_simple_term)
}

// how do with expression?
