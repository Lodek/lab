# Usersets Rewrite

In the relationship based access control model, relationship entries state that some user has a relation to some object.
A simple example of a relationship would be `Bob` is a `reader` of `readme.md`.

It's common however for relations to have relationships with themselves.
In other words, relations are often hierarchical.

Consider the relations `reader` and `editor`.
It's often the case where being an `editor` implies being a `reader` of some object.
Using Zanzibar's 3-tuple relationship model, there is no way to model this hierarchy.
That's the gap userset rewrites fill.

## Rewrite Model

A Zanzibar relation may optionally define userset rewrite rules.
Rewrite rules are expressions which modify the effective userset computed by a relation.
The mental model for a rewrite rule is that they're functions which receives an `object` and returns an userset.
The resulting usersets are combined with the relation's userset through set operations.

In Zanzibar's original design, the previous example of `editor` implying `reader` would be modeled through a `Computed Userset` rule.
Therefore, the relation `viewer` would define an userset rewrite rule of type `ComputedUserset` for the relation `editor`.
If these usersets are joined through an union operation, the net userset would be the union of all viewers and all editors for a given object.

The original paper defines two other rules:
- `_this` which returns the userset for the current relation. The default behavior when no rules are specified
- `tuple_to_userset` which allows for complex object hierarchy definitions.


## Implementation

For this implementation, the Userset Rewrites were modeled in a simplified manner when compared to the paper.
A `Relation`s definition is translated to an Expression Tree, where leaves are userset rewrite rules and the nodes are two trees and a set operation, indicating how to merge the trees.

Consider the following zanzibar relation definitions, which expressed our editor-viewer relationship:

```
relation: { name: "editor" }
relation: { 
    name: "viewer"
    userset_rewrite {
        union {
            child { _this {} }
            child { computed_userset { relation: "editor" } }
        }
    }
}
```


For the purpose of this implementation, `_this` can be omitted in the Relation definition as it was understood to be implied.
A nice consequence of the implied `_this` is that `computed_userset` can be merged with `_this` to define a single rule, the `FetchUser` rule.

For this implementation, the expression tree for the previous definition is translated as:

```
FetchUser "viewer" 
                    \
                      Union
                    /
FetchUser "editor" 
```

Since by Zanzibar's definition, userset rewrite rules are functions that receives an object and return an userset, evaluating a `FetchUser` expression is equivalent to the `read` API call.
Therefore, to expand an expression tree, one must input an `object`, which will yield a tree of usersets.

The tree can be returned as is - like in the `expand` API call - or get merged into a single set of users in order to reason about which users have are part of the resulting (object, relation) pair.

A more complicated example would be: owner implies editor and editor implies viewer, however the same model applies:

```
    FetchUser "viewer" 
                     \
                      \
                       \
                        \
                         \
                          \
                           \
                            Union
                            /
    FetchUser "editor"     /
                    \     /
                      Union
                    /
    FetchUser "owner" 
```
