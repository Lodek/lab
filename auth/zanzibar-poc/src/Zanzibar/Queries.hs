module Zanzibar.Queries where

import qualified Zanzibar.Model as Mo

findRelation :: [Mo.Relation] -> Mo.RelName -> Maybe Mo.Relation
findRelation rs name 
    = let match = filter (\(Mo.Relation name' _) -> name' == name) rs in
    if length match == 1
    then return $ match !! 0 
    else Nothing

-- |Return all users for an userset as given by (obj, rel).
-- |Expands on userset indirection, does not account for userset rewrites
fetchUsers :: [Mo.Relationship] -> Mo.Object -> Mo.RelName -> [Mo.User]
fetchUsers rels obj rel 
    = let users = map trd $ filterObjRel rels obj rel in
    concat $ map mapUser users
    where trd (_, _, c) = c
          mapUser (Mo.Userset (obj', rel')) = fetchUsers rels obj' rel'
          mapUser u = [u]

-- Filter relationship tuples to an object relation pair
filterObjRel :: [Mo.Relationship] -> Mo.Object -> Mo.RelName -> [Mo.Relationship]
filterObjRel ts obj rel = filter (\(obj', rel', _) -> (obj == obj') && (rel == rel')) ts
