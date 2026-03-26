#![allow(unused_imports)]

mod util;
use util::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day10;

use super::*;

use std::fmt::Display;
use itertools::{Itertools,assert_equal};

use indoc::indoc;
use aoc_driver::Part;
use peg::{error::ParseError, str::LineCol};

trait Ok<T> {
	fn ok(self) -> anyhow::Result<T>;
}

impl<T> Ok<T> for T {
	fn ok(self) -> anyhow::Result<T> {
		anyhow::Result::Ok(self)
	}
}

trait Solution {

	const DAY: i32;
	const PART: Part;

	// NOTICE: Using impl Diplay causes lifetime issues when calling
	// `aoc_driver::calculate_and_post`
	fn solve(input:&str) -> anyhow::Result<impl Display>;

	fn try_submit() -> Result<(),AppError> {

		let cookie: String = cookie()?;

		let solve = |input: &str| -> String {
			match Self::solve(input) {
				Ok(solution) => solution.to_string(),
				Err(err) => {
					eprintln!("{err}");
					"".to_string()
				}
			}
		};

		aoc_driver::calculate_and_post(
			&cookie, YEAR, Self::DAY, Self::PART,
			Some(format!("inputs/{}.txt",Self::DAY)),
			Some(format!("cache/{}.json",Self::DAY)),
			// NOTICE: Using Solution::solve directly causes lifetimes issue
			solve
		).map_err(|e| {
			let msg = format!("Solution for day {} rejected: {e:?}",Self::DAY);
			AppError::IncorrectSolution(msg)
		})
	}
}

/// A generic parse for multiline input
/// Takes a Rust-peg parse function that is applied to every line
fn parse<'a,T>(input: &'a str, parse:fn(&'a str) -> Result<T,ParseError<LineCol>>) -> impl Iterator<Item=T> + use<'a,T> {
	input.lines().map(move |l| {
		parse(l)
			.inspect_err(|e| eprintln!("Failed parsing {l}: {e}"))
			.expect("Parser should not fail")
	})
}

macro_rules! submit {
	($($part:ty),+) => {
		#[allow(unexpected_cfgs)]
		#[cfg(all(test,feature="solve"))]
		mod submit {
			use super::*;
			$(
				::paste::paste! {
				#[test]
					fn [<test_ $part:lower _submit>]()-> Result<(), $crate::AppError> {
						<$part as $crate::days::Solution>::try_submit()
					}
				}
			)+
		}
	}
}

use submit;
