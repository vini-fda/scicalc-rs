# Scicalc-rs

Rust crate for parsing and doing calculations with measurements, typically used in scientific contexts.


# TODO

## Calculator
- Parse and perform basic operations with measurements
  - For example, addition `(23.0 ± 0.1) + (1.5 ± 0.5)`
- Add support for exponentiation, logarithms, squareroots, n-th roots and many other functions

## Significant figures & Scientific notation
- Parse and verify if a measured quantity has the correct representation, i.e. with corresponding amount of sig figs
- Parse different kinds of scientific notation, such as `(23.0E+7 ± 1.0E6)`, `(2.00 ± 0.01)E-10` and `2.00*10^9`

## Miscellaneous
- Add support for numeric constants with no uncertainty, such as `42`, `e`, `π`, etc