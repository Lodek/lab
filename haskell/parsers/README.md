# About
Study about haskell parsers

# Todos
[x] Read grahams article
[x] Study grammar
[x] Write simple grammar for RTE
[x] Play with parser combinators to implement RTE parser

- Read intro to parsec tutorial
- Rewrite parser with parsec

# QA

## Given the parser primitives, how to handle alternation?
The first instict is clearly `Alternative`.
Just don't know how to use it yet.
Suppose a list [1,2,3,], how to write a parser that doens't freak out with the trailing `,`?

# Parser monad

I got to understand a bit about the parser monad from grahams book.
Or I should say, parser combinators.
It's a rather simple idea whose implementation isn't overly complicated.
Resembles a lot the state monad.

In fact, it seems like the StateT transformer combined with the list monad.

The signature for the parser type is `newtype Parser a = Parser { parse :: (String -> [(a, String)] }`
Well, if tha doesn't look just like `type Parser a = StateT String [] a`

I want to implement it both ways, using monad transformers and the vanilla way.

It;s going to be interesting to work on this as it will help me understand the applicative monad, which is something i struggle with a bit.

SO it's besically you got a context of as, a context of bs, how combine?

# 

# Resources

- [Ben Lynn article](https://crypto.stanford.edu/~blynn/haskell/parse.html)

- Graham chapter 13

- [Graham Article on Parser Combinators](http://www.cs.nott.ac.uk/~pszgmh/parsing.pdf)

- [Article on parsec](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/parsec-paper-letter.pdf)
