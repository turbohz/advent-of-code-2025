// https://adventofcode.com/2025/day/6


use super::*;

#[derive(Debug,Clone,Copy,PartialEq)]
enum Op {
	Sum,
	Prod,
}

impl Op {
	fn compute(&self,lhs:usize,rhs:usize) -> usize {
		match self {
			Op::Sum  => lhs+rhs,
			Op::Prod => lhs*rhs,
		}
	}
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum Col {
	Sep,
	Opnd(usize),
	OpndOptr(usize,Op)
}

peg::parser! {

	grammar parser() for str {

		rule _ = [' ']
		rule __ = _+
		rule EOL() = ![_]

		rule digit() -> char =
			[c if c.is_ascii_digit()]

		rule number() -> usize =
			ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		rule op() -> Op =
			"+" { Op::Sum  } /
			"*" { Op::Prod }

		pub rule operands() -> Vec<usize> =
			__? operands:(number() ++ __) __? { operands }

		pub rule operators() -> Vec<Op> =
			operators:(op() ++ __ ) __? { operators }

		pub rule col() -> Col =
			_+                      EOL() { Col::Sep } /
			_* n:number() _+        EOL() { Col::Opnd(n) } /
			_* n:number() _* o:op() EOL() { Col::OpndOptr(n,o) }

	}
}

fn parse(input:&str) -> (Vec<Vec<usize>>, Vec<Op>) {

	let mut iter = input.lines().peekable();

	let mut all_opnds:Vec<Vec<usize>> = Default::default();
	let mut operators:Vec<Op> = Default::default();

	loop {

		let Some(line) = iter.next() else {
			assert!(!operators.is_empty());
			break (all_opnds,operators)
		};

		if iter.peek().is_some() {

			// Parse operands line
			let operands = parser::operands(line).unwrap();
			all_opnds.push(operands);

		} else {

			// Last line, parse operators
			operators = parser::operators(line).unwrap();
		}
	}
}

fn compute_cols(operands:Vec<Vec<usize>>,operators:Vec<Op>) -> Vec<usize> {

	operators.iter()
		.enumerate()
		.map(|(i,optr)| {
			let opnds = operands.iter().map(|v| v[i]);
			opnds.reduce(|a,b| optr.compute(a,b)).unwrap()
		})
		.collect_vec()
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 6;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let (operands,operators) = parse(input);

		compute_cols(operands, operators).iter().sum::<usize>()
	}
}

fn transposed_chunks(lines:Vec<&str>)-> Vec<String> {

	let cols = lines[0].len();
	let rows = lines.len();

	let coords = (0..cols).rev().cartesian_product(0..rows);

	coords
		.map(|(x,y)| unsafe { lines[y].get_unchecked(x..=x) })
		.chunks(rows)
		.into_iter()
		.map(|c| c.collect_vec().join(""))
		.collect_vec()
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 6;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let lines = input.lines().collect_vec();

		let cols = transposed_chunks(lines).into_iter()
			.map(|ref c| parser::col(c).expect("Column should parse into a Col instance"))
			.collect_vec().into_iter();

		cols
			.chunk_by(|col| matches!(col,Col::Sep))
			.into_iter()
			.map(|(is_sep,group)| {

				if is_sep { return 0 };

				// Process Operation Group

				let (opnds,optrs):(Vec<_>, Vec<_>) = group.into_iter()
					.map(|col:Col|
						match col {
							Col::Opnd(opnd) => (opnd,None),
							Col::OpndOptr(opnd,optr) => (opnd,Some(optr)),
							_ => unreachable!()
						}
					)
					.unzip();

				let optr = optrs.into_iter().flatten().next().unwrap();

				opnds.into_iter().reduce(|a,b| optr.compute(a,b)).unwrap()
			})
			.sum::<usize>()
	}
}

#[cfg(test)]
mod test {

	use super::*;

	const EXAMPLE_INPUT:&str = concat!(
		"123 328  51 64 \n",
		" 45 64  387 23 \n",
		"  6 98  215 314\n",
		"*   +   *   +  "
	);

	#[test]
	fn test_parser() {

		let (operands,operators) = parse(EXAMPLE_INPUT);

		assert_eq!(operands[0], vec![123, 328,  51,  64]);
		assert_eq!(operands[1], vec![ 45,  64, 387,  23]);
		assert_eq!(operands[2], vec![  6,  98, 215, 314]);

		assert_eq!(operators, vec![Op::Prod, Op::Sum, Op::Prod, Op::Sum]);
	}

	#[test]
	fn test_compute() {

		let (operands,operators) = parse(EXAMPLE_INPUT);
		let mut computed = compute_cols(operands, operators).into_iter();

		assert_eq!(computed.next().unwrap(),33210);
		assert_eq!(computed.next().unwrap(),490);
		assert_eq!(computed.next().unwrap(),4243455);
		assert_eq!(computed.next().unwrap(),401);
	}

	#[test]
	fn test_transpose() {

		let lines = EXAMPLE_INPUT.lines().collect_vec();
		let mut cols = transposed_chunks(lines).into_iter();

		assert_eq!(cols.next().unwrap(), "  4 ");
		assert_eq!(cols.next().unwrap(), "431 ");
		assert_eq!(cols.next().unwrap(), "623+");
		assert_eq!(cols.next().unwrap(), "    ");
		assert_eq!(cols.next().unwrap(), "175 ");
		assert_eq!(cols.next().unwrap(), "581 ");
		assert_eq!(cols.next().unwrap(), " 32*");
		assert_eq!(cols.next().unwrap(), "    ");
		assert_eq!(cols.next().unwrap(), "8   ");
		assert_eq!(cols.next().unwrap(), "248 ");
		assert_eq!(cols.next().unwrap(), "369+");
		assert_eq!(cols.next().unwrap(), "    ");
		assert_eq!(cols.next().unwrap(), "356 ");
		assert_eq!(cols.next().unwrap(), "24  ");
		assert_eq!(cols.next().unwrap(), "1  *");
	}

	#[test]
	fn test_parse_col() {

		use parser::col;

		assert_eq!(col("  4 ").unwrap(),Col::Opnd(4));
		assert_eq!(col("431 ").unwrap(),Col::Opnd(431));
		assert_eq!(col("623+").unwrap(),Col::OpndOptr(623,Op::Sum));
	}

	#[test]
	fn test_example() {

		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "4277556";

		assert_eq!(actual,expected);

		let actual = Part2::solve(EXAMPLE_INPUT).to_string();
		let expected = "3263827";

		assert_eq!(actual,expected);

	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
