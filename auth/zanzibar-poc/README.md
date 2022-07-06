# Zanzibar

A toy implementation of Google's Zanzibar authentication service. 

The implementation consists of a Haskell library with a subset of Zanzibar's API methods.
The design prioritized understanding over performance / fidelity.
The goal was to provide an implementation that captures the nuances of Zanzibar's data model and relation hierarchy through "userset rewrites".

Some notes on Zanzibar's model and the implementation can be found in the [docs directory](./docs)


## Running 

To spin up a simple test environment run:

```sh
make run
```

The `make` recipe will build a container with the Zanzibar lib, run an ephemeral docker container (ie with --rm) and spin up a ghci instance with some definitions and imports.
Checkout [`example/runner.hs`](example/runner.hs) to see the defined relations and relationship tuples.


## Limitations

Zanzibars data model is best understood as directed graph where nodes are (`object`, `relation`) pairs and leaves are usually `user_id`s.
Note that there's no constraint in its data model against Cyclical Relations.

This implementation does not check nor deals with Cyclical Relations and assume that the relations will form a Directed Acyclic Graph.
Defining cyclic relationships will throw Haskell into an infinite recursion.

Likewise, there is no constraint against building circular `relation` hierarchy through userset rewrites.
Doing so will also cause an infinite recursion.
