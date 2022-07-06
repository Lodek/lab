module Zanzibar.Model where

-- |Relationship tuple as defined in paper.
-- |The relation triplet expresses an entry in ACL which is expanded and operated
-- |by Zanzibar's API.
type Relationship = (Object, RelName, User)

type Object = (Namespace, ObjectId)
type Namespace = String
type ObjectId = String

-- |Defines users or group of users in Zanzibar.
data User 
    -- |Identifies a single concrete user
    = UID String
    -- |Userset enables grouping users by specifying an obj, relation pair.
    -- |The resulting user set is defined as:
    -- |Let 'r' be a relationship, 'o' be an object and T be the set of relationship tuples,
    -- |the resulting userset is composed of all users 'u' such that
    -- |(o, r, u) is in T
    | Userset (Object, RelName) 
    -- |All is a special value used to indicate that the permission is global to all users.
    -- |Although omitted in the official paper, it is mentioned in: https://youtu.be/mstZT431AeQ?t=300
    | All 
    deriving (Eq, Show, Ord)

-- |Model a Zanzibar Relation, whose name is a member of the relation triplet.
-- |Relations are related through 'UsersetRewrite', which enables relationships such as
-- | relation "owner" implies "viewer"
data Relation = Relation RelName UsersetRewrite deriving (Eq, Show)
type RelName = String

-- |UsersetRewrite defines an expression a Relation.
-- |By leveraging rewrites it's possible to define a hiearchy between relations.
-- |A simple scenario is when one relation implies another, such as "owner" implying "editor".
-- |This hieararchy can be modelled through rewrite rules under a userset rewerite tree.
-- |
-- |The chosen data model is slightly different from Zanzibar's in that
-- |the `_this` rule is implied for every Relation. 
-- |Optional relationships can be expressed through the Rule list,
-- |which will be comined to the implied `_this` rule through `NodeOperation`
data UsersetRewrite = Rewrite NodeOperation [Rule]
    deriving (Eq, Show)

-- |Zanzibar Userset rewrite rules are combined through set operations.
data NodeOperation = Union | Intersection | Difference deriving (Eq, Show)

-- |Userset rewrite rules as specified in the paper
data Rule
    -- |Return a new userset for the given relation name and the current object.
    = ComputeUserset RelName
    -- Omitted tuple_to_userset to reduce scope
    deriving (Eq, Show)
