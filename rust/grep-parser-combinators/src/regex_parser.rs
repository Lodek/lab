use parser_combinator::{ParserResult};
use parser_combinator::parsers::{elem, alphanumeric, satisfies};
use parser_combinator::combinators::{many, alternation, some};

use crate::syntax_tree::Tree;

fn kleene_node_parser<'a>(stream: &'a str) -> ParserResult<'a, Tree> {
    let (symbol, tail) = elem(stream)?;
    let (star, tail) = satisfies(tail, |c| c == '*');
    Ok((Tree::KleeneLeaf(symbol), tail))
}

fn leaf_parser<'a>(stream: &'a str) -> ParserResult<'a, Tree> {
    let (symbol, tail) = elem(stream)?;
    Ok((Tree::Leaf(symbol), tail))
}

fn token_parser<'a>(stream: &'a str) -> ParserResult<'a, Tree> {
    alternation(stream, kleene_node_parser, leaf_parser)
}

pub fn parse_regex(stream: &'a str) -> Result<Tree, String> {
    let (trees, stream) = some(stream, token_parser)?;
}
