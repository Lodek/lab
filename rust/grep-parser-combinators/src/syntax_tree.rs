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
    Group(Box<Expression>)
}

pub enum Term {
    Repetition(Factor, Repeater),
    Term(Factor),
}

pub enum Expression {
    Concat(Box<Expression>, Term),
    Alternation(Box<Expression>, Term),
    Singleton(Term)
}

