/// Regex language:
/// expression = expr term | expr \| term | term
/// term = factor repetition | factor
/// factor = symbol | ( expression )
/// symbols are any valid ascii character except for ()*|.

#[derive(Debug, PartialEq)]
pub enum Repeater {
    Any,
    Some
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Symbol(char),
    Group(Box<Expression>)
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Repetition(Factor, Repeater),
    Term(Factor),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Concat,
    Alternation
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Node(Box<Expression>, Operator, Box<Expression>),
    Leaf(Term)
}
