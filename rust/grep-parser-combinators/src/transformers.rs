use parser_combinators::combinators::{many, some};
use parser_combinators::parsers::{a_char};
use parser_combinators::ParserResult;
use crate::syntax_tree::{Expression, Factor, Term, Repeater};

//pub fn expression_parser_factory(
