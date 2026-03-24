# Advent of Code 2025

Solutions to the **[Advent of Code 2025](https://adventofcode.com/2025)** challenges, using the Rust programming language.

> This document and the solutions are a work in progress,
> and may often be reviewed and reworked.

## About these solutions

Instead of atacking the problems with quick and dirty solutions,
the intention is to choose approches that are interesting to me.

## Submission

The solutions submission are performed by test functions gated with the "solve" feature.

```
# All
cargo test --release --features solve
# One
cargo test --release --features solve day\d{2}::submit
```
