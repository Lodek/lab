use parser_combinators::combinators::{many, some, chain, alternate};
use parser_combinators::parsers::{a_char, elem};
use parser_combinators::ParserResult;

use crate::syntax_tree::{Repeater, Factor, Term, Expression, Operator};

fn factor_parser_factory(factor: &Factor) -> Box<dyn for <'a> Fn(&'a str) -> ParserResult<'a, ()>> 
{
    match factor {
        Factor::Symbol(c) => {
            let c = *c;
            Box::new(move |stream| a_char(stream, c).map(|(_, stream)| ((), stream)))
        },
        //Factor::Group(expr) => expression_parser_factory(&expr),
        _ => Box::new(|stream| elem(stream).map(|(_, stream)| ((), stream))),
    }
}


fn unitify_parser<P, T>(parser: P) -> impl for<'a> Fn(&'a str) -> ParserResult<'a, ()> 
where for<'b> P: Fn(&'b str) -> ParserResult<'b, T> 
{
    move |stream| (parser)(stream).map(|(_, stream)| ((), stream))
}

fn term_parser_factory(term: &Term) -> Box<dyn for<'a> Fn(&'a str) -> ParserResult<'a, ()>> {
    match term {
        Term::Repetition(factor, Repeater::Any) => Box::new(unitify_parser(many(factor_parser_factory(factor)))),
        Term::Repetition(factor, Repeater::Some) => Box::new(unitify_parser(some(factor_parser_factory(factor)))),
        Term::Term(factor) => Box::new(factor_parser_factory(factor))
    }
}

pub fn expression_parser_factory(expression: &Expression) -> Box<dyn for<'a> Fn(&'a str) -> ParserResult<'a, ()>>
{
    match expression {
        Expression::Leaf(term) => {
            Box::new(term_parser_factory(&term))
        },
        Expression::Node(left, operator, right) => {
            let left_parser = expression_parser_factory(left);
            let right_parser = expression_parser_factory(right);

            if let Operator::Concat = operator {
                let parser = chain(left_parser, right_parser);
                Box::new(move |stream| parser(stream).map(|(_, tail)| ((), tail)))
            }
            else {
                let parser = alternate(left_parser, right_parser);
                Box::new(move |stream| parser(stream).map(|(_, tail)| ((), tail)))
            }
        },
    }
}
