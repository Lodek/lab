/// Regex language:
/// expression = expr term | expr \| term | term
/// term = factor repetition | factor
/// factor = symbol | ( expression )
/// symbols are any valid ascii character except for ()*|.

pub enum Repeater {
    Any,
    Many
}

pub enum Factor {
    Symbol(char),
    Group(Expression)
}

pub enum Term {
    Repetition(Factor, Repeater),
    Term(Factor),
}

pub enum Expression {
    Concat(Expression, Term),
    Alternation(Expression, Term),
    Singleton(Term)
}
