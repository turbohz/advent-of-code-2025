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
 Nextest run ID 9dc6c4d1-97f6-4bbc-8414-51c22255cff0 with nextest profile: default
    Starting 18 tests across 2 binaries (30 tests skipped)
        PASS [   0.009s] aoc_2025 days::day03::submit::test_part1_submit
        PASS [   0.011s] aoc_2025 days::day01::submit::test_part1_submit
        PASS [   0.015s] aoc_2025 days::day03::submit::test_part2_submit
        PASS [   0.017s] aoc_2025 days::day01::submit::test_part2_submit
        PASS [   0.008s] aoc_2025 days::day05::submit::test_part1_submit
        PASS [   0.006s] aoc_2025 days::day05::submit::test_part2_submit
        PASS [   0.007s] aoc_2025 days::day06::submit::test_part1_submit
        PASS [   0.009s] aoc_2025 days::day07::submit::test_part1_submit
        PASS [   0.009s] aoc_2025 days::day07::submit::test_part2_submit
        PASS [   0.027s] aoc_2025 days::day04::submit::test_part1_submit
        PASS [   0.011s] aoc_2025 days::day06::submit::test_part2_submit
        PASS [   0.007s] aoc_2025 days::day10::submit::test_part1_submit
        PASS [   0.045s] aoc_2025 days::day10::submit::test_part2_submit
        PASS [   0.273s] aoc_2025 days::day02::submit::test_part1_submit
        PASS [   0.355s] aoc_2025 days::day04::submit::test_part2_submit
        PASS [   0.476s] aoc_2025 days::day08::submit::test_part2_submit
        PASS [   0.669s] aoc_2025 days::day02::submit::test_part2_submit
        PASS [   0.748s] aoc_2025 days::day08::submit::test_part1_submit
```
