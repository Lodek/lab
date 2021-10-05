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

impl Expression {
    pub fn into_dfs_iterator(&self) -> EagerDfsExpressionIterator {
        EagerDfsExpressionIterator::new(self)
    }
}

pub struct EagerDfsExpressionIterator<'a>(Vec<&'a Expression>);

impl<'a> EagerDfsExpressionIterator<'a> {

    pub fn new(expression: &'a Expression) -> EagerDfsExpressionIterator<'a> {
        let nodes = Self::generate_nodes(expression);
        EagerDfsExpressionIterator(nodes)
    }

    fn generate_nodes(head: &'a Expression) -> Vec<&'a Expression> {
        let mut stack = Vec::new();
        Self::generate_node_sequence(head, &mut stack);
        stack
    }

    fn generate_node_sequence(head: &'a Expression, stack: &mut Vec<&'a Expression>) {
        if let Expression::Node(ref left, _, ref right) = head {
            Self::generate_node_sequence(left, stack);
            Self::generate_node_sequence(right, stack);
            stack.push(head);
        }
    }
}

impl<'a> Iterator for EagerDfsExpressionIterator<'a> {

    type Item = &'a Expression;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// TODO Make lazy implementation for dfs iterator
//struct DfsExpressionIterator<'a>
