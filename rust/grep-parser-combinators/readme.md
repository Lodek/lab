# About
This project implements a simple `grep` like cli tool.

The CLI interface is very primitive and doesn't contains helpful messages but it work as expected.
Provide a regular expression as the first argument and the program will do a linewise match with the provided regex.

Eg

```
cargo run "a(b|c)*d" < test
```

# Regex description
The regex defined here is extremely simplistic but features "main" suspects.

TLDR:
- Grouping is supported through `( expression )`
- Repetition is supported through `*`
- Alternation is supported by `|`

Any other character will be taken literally.

Note that if you'd like to match against `(`, `)`, `*` or `|`, you're out of luck.
