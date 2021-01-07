# Scicalc-rs

Rust crate for parsing and doing calculations with measurements, typically used in scientific contexts.


# TODO

## Lexing [OK]
Transform a string(i.e. the input as a sequence of characters) into a sequence of *tokens*, which can then be fed into the *parser*.

## Parsing [WIP]
Read a sequence of tokens — which has a linear structure — and transform it into a **tree** structure.

## Evaluating [WIP]
Read the tree structure of the expression and *fold* it, reducing it into it's final value.

## Calculator
- Parse and perform basic operations with measurements
  - For example, addition `(23.0 ± 0.1) + (1.5 ± 0.5)`
- Add support for exponentiation, logarithms, squareroots, n-th roots and many other functions

## Significant figures & Scientific notation
- Parse and verify if a measured quantity has the correct representation, i.e. with corresponding amount of significant figures
- Parse different kinds of scientific notation, such as `(23.0E+7 ± 1.0E6)`, `(2.00 ± 0.01)E-10` and `2.00*10^9`

## Miscellaneous
- Add support for numeric constants with no uncertainty, such as `42`, `e`, `π`, etc
- Add support for digraphs(e.g 'pi' for `π` and '+-' for `+-`)


# BNF grammar for the expressions

```
      Expression ::= Value | UnaryExpression | BinaryExpression | Grouping
        Grouping ::= "(" Expression ")"
           Value ::= Constant | Number | Measurement
     Measurement ::= Number "±" PosNumber
          Number ::= PosNumber | UnaryMinus PosNumber
       PosNumber ::= (\d+)(\.\d+)?|(\.\d+)
        Constant ::= "e" | "π"
BinaryExpression ::= Expression BinaryOperator Expression
 UnaryExpression ::= UnaryOperator Expression
  BinaryOperator ::= "+" | "-" | "*" | "/"
   UnaryOperator ::= UnaryMinus
      UnaryMinus ::= "-"
```

Note: As observed by [Pratt's paper on "Top Down Operator Precedence"](https://web.archive.org/web/20151223215421/http://hall.org.ua/halls/wizzard/pdf/Vaughan.Pratt.TDOP.pdf), a Backus-Naur Form(for which BNF is a shorthand) is very inept at capturing the precedence of infix operators. Even then, I still think that specifying a grammar with BNF is useful for providing a quick-and-easy guide, with which you can see the recursive structure of the language at a glance.

# Acknowledgements & Further reading

These are some resources that I used to learn about programming language theory, algorithms and their implementations:

- [Brief introduction to recursive descent parsing](http://web.archive.org/web/20170712044658/https://ryanflannery.net/teaching/common/recursive-descent-parsing/), by Ryan Flannery

- [Crafting Interpreters](http://craftinginterpreters.com/representing-code.html), by Bob Nystrom

- [Pratt parsing and precedence climbing are the same algorithm](https://www.oilshell.org/blog/2016/11/01.html), by Oilshell

- [Programming Language Theory](https://steshaw.org/plt/), a huge list of resources about PLT maintained by Steven Shaw

- [Simple but powerful Pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html), by Aleksey Kladov(matklad)