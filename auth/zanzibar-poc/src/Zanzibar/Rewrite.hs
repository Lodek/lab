module Zanzibar.Rewrite where

import qualified Zanzibar.Model as M
import qualified Zanzibar.Queries as Q
import qualified Data.Set as Set


-- |Binary tree whose nodes are combined through Set operations,
-- |(union, intersect, difference).
data OpTree n
    = Node (OpTree n) M.NodeOperation (OpTree n)
    | Leaf n
    deriving (Eq, Show)

instance Functor OpTree where
    --fmap :: (a -> b) -> OpTree a -> OpTree b
    fmap f (Node left op right) = Node (fmap f left) op (fmap f right)
    fmap f (Leaf n) = Leaf $ f n

-- |Evaluate an operation tree by folding its leaves with the
-- |appropriate Set operation as defined in each node.
eval :: Ord a => OpTree (Set.Set a) -> Set.Set a
eval (Leaf n) = n
eval (Node left  M.Union right) = (eval left) `Set.union`  (eval right)
eval (Node left  M.Intersection right) = (eval left) `Set.intersection` (eval right)
eval (Node left  M.Difference right) = (eval left) `Set.difference` (eval right)


-- |Function defines which rewrite function to be performed while
-- |expanding a node in an userset rewrite expression tree
data Function 
    = FetchUserset M.RelName
    deriving (Eq, Show)

-- |Expression Tree as defined in Zanzibar paper.
-- |Expression trees can be expanded to resolve userset rewrite
-- |definitions.
type ExprTree = OpTree Function

-- |User / Userset tree which results from evaluating an Expression Tree
type UserTree = OpTree (Set.Set M.User)

-- |Build an userset rewrite expression tree for a given Relation
-- |Rules are left associative
buildExprTree :: [M.Relation] -> M.RelName -> Maybe ExprTree
buildExprTree rels name = do 
    (M.Relation _ (M.Rewrite op rules))  <- Q.findRelation rels name
    first  <- return $ Leaf (FetchUserset name)
    trees  <- sequence $ fmap (ruleToOpTree rels) rules
    foldTrees op (first:trees)


-- |Left associative folding function for OpTrees
foldTrees :: M.NodeOperation -> [ExprTree] -> Maybe ExprTree
foldTrees op [] = Nothing
foldTrees op (t:[]) = return t
foldTrees op (t:ts) = return $ foldl (\acc tree -> Node acc op tree) t ts

-- |Map a Rule to an ExpressionTree. 
ruleToOpTree :: [M.Relation] -> M.Rule -> Maybe ExprTree
ruleToOpTree rs (M.ComputeUserset rel) = buildExprTree rs rel


-- |Expand a Function expression tree by 
expandNodes :: [M.Relationship] -> M.Object -> ExprTree -> UserTree
expandNodes rels obj tree = fmap (\func -> funcMap func obj) tree
    where funcMap (FetchUserset rel) obj = Set.fromList $ Q.fetchUsers rels obj rel
