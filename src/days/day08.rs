// https://adventofcode.com/2025/day/8

// The problem as described is unclear about if cycles are to be constructed.
// However, only by allowing them we get the expected example result

use super::*;

use derive_more::{Deref, Display, From, Into};

use petgraph::{
	prelude::*,
	graph::EdgeReference,
};

#[derive(Debug,Clone,Copy,From,Into,PartialEq,Eq,Deref)]
struct Pair<T>([T;2]);

impl<T:Copy> IntoIterator for Pair<T> {
	type Item = T;
	type IntoIter = <[T;2] as IntoIterator>::IntoIter;
	fn into_iter(self) -> Self::IntoIter {
		[self.0[0],self.0[1]].into_iter()
	}
}

#[derive(Debug,Clone,Copy,From,Into,Hash)]
struct Triple<T>([T;3]);


impl<T:Copy> IntoIterator for Triple<T> {
	type Item = T;
	type IntoIter = <[T;3] as IntoIterator>::IntoIter;
	fn into_iter(self) -> Self::IntoIter {
		[self.0[0],self.0[1],self.0[2]].into_iter()
	}
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash,Display,Default,PartialOrd,Ord)]
#[display("{x},{y},{z}")]
struct Location3 {
	pub x: usize,
	pub y: usize,
	pub z: usize,
}

impl Location3 {
	#[allow(dead_code)]
	const ORIGIN:Self = Self { x:0 , y:0 , z:0 };
}

impl<T:Into<[usize;3]>> From<T> for Location3 {
	fn from(value: T) -> Self {
		let [x,y,z] = value.into();
		Self { x,y,z }
	}
}

type Distance = usize;

peg::parser! {

	grammar parser() for str {

		rule digit() -> char =
			[c if c.is_ascii_digit()]

		rule number() -> usize =
			ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		pub rule triple<'a>() -> Triple<usize> =
			l:(number() **<3> ",") { [l[0],l[1],l[2]].into() }
	}
}

fn parse(input:&str) -> Vec<Location3> {
	crate::days::parse(input, parser::triple).map_into().collect_vec()
}

// Given that we only need to know the relative distances,
// we can just use the distance squared, and save computing
// the square root.
fn distance_squared(l:Location3, r:Location3) -> usize {

	let d:Location3 = (
			l.x.abs_diff(r.x),
			l.y.abs_diff(r.y),
			l.z.abs_diff(r.z),
		).into();

	d.x * d.x + d.y * d.y + d.z * d.z
}

mod group {

	use std::num::NonZero;

	#[derive(Debug,Clone,Copy,Eq,PartialEq)]
	pub enum GroupId {
		None,
		Some(NonZero<u16>)
	}

	impl Ord for GroupId {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			let a:u16 = (*self).into();
			let b:u16 = (*other).into();
			a.cmp(&b)
		}
	}

	impl PartialOrd for GroupId {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl From<u16> for GroupId {
		fn from(value: u16) -> Self {
			match value {
				0 => Self::None,
				// SAFETY: Value 0 has been excluded in the previous match arm.
				v => unsafe { Self::Some(NonZero::new_unchecked(v)) }
			}
		}
	}

	impl From<GroupId> for u16 {
		fn from(val: GroupId) -> Self {
			match val {
				GroupId::None => 0,
				GroupId::Some(v) => v.get()
			}
		}
	}
}

use group::GroupId;

// Using a graph map we avoid duplicated nodes (same weight but different id)
fn distance_graph<T:Sized+Clone+Iterator<Item=Location3>>(items:T) -> UnGraphMap<Location3,Distance> {

	// A GraphMap will deduplicate nodes and edges
	let mut graph:UnGraphMap<Location3,Distance> = UnGraphMap::default();

	for [l3a,l3b] in items.array_combinations() {
		let a = graph.add_node(l3a);
		let b = graph.add_node(l3b);
		graph.add_edge(a, b, distance_squared(l3a,l3b));
	}

	graph
}

/// Creates a subgraph, keeping the amount of edges specified by `limit`
fn proximity_graph<const N:usize>(distance_graph:UnGraph<Location3,Distance>) -> UnGraph<(),()> {

	let sorted_edge_refs_by_weight:[EdgeReference<_>;N] = distance_graph
		.edge_references()
		.sorted_unstable_by_key(EdgeReference::weight)
		.take(N)
		.collect_array()
		.unwrap();

	let nodes = sorted_edge_refs_by_weight
		.iter()
		.flat_map(|e| [e.source(),e.target()])
		.unique()
		.collect_vec();

	let edges:[EdgeIndex;N] = sorted_edge_refs_by_weight
		.iter()
		.map(EdgeReference::id)
		.collect_array()
		.unwrap();

	distance_graph.filter_map(
		|nix,_| nodes.contains(&nix).then_some(()), // default color
		|eix,_| edges.contains(&eix).then_some(())
	)
}

/// Returns a graph with nodes "colored" by circuit they are part of
fn circuit_group_graph<N,E:Default>(graph:UnGraph<N,E>) -> UnGraph<GroupId,E> {

	use petgraph::visit::Dfs;

	let mut color = {
		let mut iter = 1u16..;
		move || GroupId::from(iter.next().unwrap())
	};

	let mut colored_graph = graph.map_owned(|_,_|GroupId::None, |_,_| E::default());

	// Color all nodes that make a circuit

	for nix in colored_graph.node_indices() {

		if colored_graph[nix] == GroupId::None {

			let mut dfs = Dfs::new(&colored_graph, nix);

			let c = color();

			dbg!(c);

			colored_graph[nix] = c;

			while let Some(nx) = dfs.next(&colored_graph) {
				colored_graph[nx] = c;
			}
		}
	}

	colored_graph
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 8;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let box_locations = parse(input).into_iter();
		let distance_graph:UnGraph<Location3,Distance> = distance_graph(box_locations).into_graph();
		Self::solve_for::<1000>(distance_graph)
	}
}

impl Part1 {

	/// Solve considering only the top `LIMIT` shortest connections
	fn solve_for<const LIMIT:usize>(distance_graph:UnGraph<Location3,Distance>)-> impl Display {

		let proximity_graph = proximity_graph::<LIMIT>(distance_graph);

		dbg!(proximity_graph.raw_edges());

		let circuit_group_graph = circuit_group_graph(proximity_graph);

		circuit_group_graph.node_weights()
			.sorted_unstable()
			.dedup_with_count()
			.map(|(count,_)| count)
			.sorted_unstable()
			.rev()
			.take(3)
			.product::<usize>()
	}
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 8;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let box_locations = parse(input).into_iter();
		let distance_graph:UnGraph<Location3,Distance> = distance_graph(box_locations).into_graph();
		let total_nodes = distance_graph.node_count();
		let mut node_seen_graph = distance_graph.map(|_,_|false, |_,_|());

		let mut sorted_edge_refs_by_weight = distance_graph
			.edge_references()
			.sorted_unstable_by_key(EdgeReference::weight);

		let mut connected_nodes = 0;

		loop {

			let edge = sorted_edge_refs_by_weight.next()
				.expect("A full circuit should be formed before running out of potential connections");

			for nix in [edge.source(),edge.target()] {
				if !node_seen_graph[nix] {
					node_seen_graph[nix] = true;
					connected_nodes += 1;
				}
			}

			if connected_nodes >= total_nodes {

				break distance_graph[edge.source()].x * distance_graph[edge.target()].x
			}
		}

	}
}

#[cfg(test)]
mod test {

	use super::*;

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		162,817,812
		57,618,57
		906,360,560
		592,479,940
		352,342,300
		466,668,158
		542,29,236
		431,825,988
		739,650,466
		52,470,668
		216,146,977
		819,987,18
		117,168,530
		805,96,715
		346,949,466
		970,615,88
		941,993,340
		862,61,35
		984,92,344
		425,690,689
		"#
	};

	#[test]
	fn test_parse() {

		let mut locations = parse(EXAMPLE_INPUT).into_iter();

		let actual:Location3 = locations.next().unwrap();
		let expected:Location3 = [162,817,812].into();

		assert_eq!(actual,expected);

		let actual:Location3 = locations.last().unwrap();
		let expected:Location3 = [425,690,689].into();

		assert_eq!(actual,expected);
	}

	#[test]
	fn test_part1() {

		let box_locations = parse(EXAMPLE_INPUT).into_iter();
		let distance_graph:UnGraph<Location3,Distance> = distance_graph(box_locations).into_graph();

		let expected = "40";
		let actual = Part1::solve_for::<10>(distance_graph).to_string();
		assert_eq!(actual,expected);
	}

	#[test]
	fn test_part2() {

		let expected = "25272";
		let actual = Part2::solve(EXAMPLE_INPUT).to_string();
		assert_eq!(actual,expected)
	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
