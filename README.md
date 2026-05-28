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

## Next-test report

Output for `cargo nextest run --release --features solve submit`

```
 Nextest run ID 83732d69-f212-4d53-bdf7-e275ac623e39 with nextest profile: default
    Starting 23 tests across 2 binaries (45 tests skipped)
        PASS [   0.011s] aoc_2025 days::day01::submit::test_part1_submit
        PASS [   0.011s] aoc_2025 days::day03::submit::test_part1_submit
        PASS [   0.011s] aoc_2025 days::day03::submit::test_part2_submit
        PASS [   0.011s] aoc_2025 days::day01::submit::test_part2_submit
        PASS [   0.008s] aoc_2025 days::day05::submit::test_part2_submit
        PASS [   0.022s] aoc_2025 days::day04::submit::test_part1_submit
        PASS [   0.010s] aoc_2025 days::day06::submit::test_part1_submit
        PASS [   0.011s] aoc_2025 days::day06::submit::test_part2_submit
        PASS [   0.012s] aoc_2025 days::day05::submit::test_part1_submit
        PASS [   0.010s] aoc_2025 days::day07::submit::test_part2_submit
        PASS [   0.011s] aoc_2025 days::day07::submit::test_part1_submit
        PASS [   0.026s] aoc_2025 days::day09::submit::test_part1_submit
        PASS [   0.023s] aoc_2025 days::day10::submit::test_part1_submit
        PASS [   0.012s] aoc_2025 days::day11::submit::test_part1_submit
        PASS [   0.088s] aoc_2025 days::day09::submit::test_part2_submit
        PASS [   0.071s] aoc_2025 days::day10::submit::test_part2_submit
        PASS [   0.011s] aoc_2025 days::day12::submit::test_part1_submit
        PASS [   0.311s] aoc_2025 days::day02::submit::test_part1_submit
        PASS [   0.445s] aoc_2025 days::day04::submit::test_part2_submit
        PASS [   0.587s] aoc_2025 days::day08::submit::test_part2_submit
        PASS [   0.781s] aoc_2025 days::day02::submit::test_part2_submit
        PASS [   0.791s] aoc_2025 days::day08::submit::test_part1_submit
        PASS [  56.673s] aoc_2025 days::day11::submit::test_part2_submit
────────────
     Summary [  56.744s] 23 tests run: 23 passed, 45 skipped
```
