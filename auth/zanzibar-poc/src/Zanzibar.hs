module Zanzibar where

import qualified Zanzibar.Rewrite as Rewrite
import qualified Zanzibar.Queries as Queries
import qualified Zanzibar.Model as M
import qualified Data.Set as Set

check :: [M.Relationship] -> [M.Relation] -> M.Object -> M.RelName -> String -> Maybe Bool
check tuples rels obj rel user = do
    xpTree <- Rewrite.buildExprTree rels rel
    utree <- return $ Rewrite.expandNodes tuples obj xpTree
    users <- return $ Rewrite.eval utree
    return $ (M.UID user `Set.member` users) || (M.All `Set.member` users)
