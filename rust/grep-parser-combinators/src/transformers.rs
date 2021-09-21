use parser_combinators::combinators::{many, some};
use parser_combinators::parsers::{a_char};
use parser_combinators::ParserResult;
use crate::syntax_tree::{Expression, Factor, Term, Repeater};

pub fn factor_parser_factory<'a>(factor: Factor) -> impl Fn(&'a str) -> ParserResult<'a, ()> { //?
    match factor {
        Factor::Symbol(c) => |stream| a_char(stream, c),
        Factor::Group(expr) => |stream| Ok((stream, ()) //update
    }
}

pub fn term_parser_factory<'a>(term: Term) -> impl Fn(&'a str) -> ParserResult<'a, ()> {
    match term {
        Term::Repetition(factor, Repeater::Any) => many(factor_parser_factory(factor)),
        Term::Repetition(factor, Repeater::Some) => some(factor_parser_factory(factor)),
        Term::Factor(factor) => factor_parser_factory(factor)
    }
}
