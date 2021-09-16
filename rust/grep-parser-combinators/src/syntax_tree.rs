pub enum Operation {
    Concat,
    Alternation
}

pub enum Tree {
    Leaf(char),
    KleeneLeaf(char),
    Node(Tree, Operation),
}
