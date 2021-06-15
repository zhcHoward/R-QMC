# R-QMC

A implementation for simplifying boolean algebra expressions using [Quine–McCluskey algorithm](https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm) and [Petrick's method](https://en.wikipedia.org/wiki/Petrick%27s_method) in Rust.

# Introduction

This binary takes minterms and not cares as inputs and prints simplified minterms. Sometimes, the result is not unique. So, you may get different results with the same input. For example, for expression `f(A,B,C,D) = Σm(4,8,10,11,12,15) + d(9,14)`, the result can be `BC'D' + AB' + AC` or `BC'D' + AD' + AC'`.

# Algorithm & Implementation

The details of the 2 main algorithm can be found in the wiki links metioned before.

- `qmc.rs` implements Quine–McCluskey algorithm.
- `petrick.rs` implements petrick's method.
- `term.rs` defines `Term` which forms minterms.

# Example

To simplify boolean algebra expressions of minterms `4,8,10,11,12,15` and not cares `9,14`, simply run command:

```
rqmc --min 4 8 10 11 12 15 --not-care 9 14
```

The output will be:

```
Result: [*100, 1**0, 1*1*]
```

In the output, '\*' means the variable on that position has been simplified. So `*100` means `BC'D'` and the result is `BC'D' + AD' + AC`.
