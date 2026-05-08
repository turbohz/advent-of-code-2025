// https://adventofcode.com/2025/day/11

use std::{any, collections::{BTreeSet, HashSet}, thread};

use super::*;
use anyhow::{Context, bail};
use derive_more::{Deref, DerefMut, Display, From, Index, Into};

mod parse {

	use tap::Tap as _;

	#[derive(Debug,Default)]
	#[cfg_attr(test, derive(PartialEq,Eq,Clone))]
	pub struct DeviceDescription {
		pub name: [char;3],
		pub outputs: Vec<[char;3]>,
	}

	peg::parser! {

		pub grammar parser() for str {

			rule name() -> [char;3] = vec:(['a'..='z']*<3>) {
				[' ';3].tap_mut(|a| a.copy_from_slice(&vec))
			}

			pub rule device() -> DeviceDescription =
				name:name() ": " outputs:(name() ** " ")  {
					DeviceDescription { name, outputs }
				}
		}
	}

	#[cfg(test)]
	mod test {
		use super::*;
		use assert2::assert;
		#[test]
		fn test_parser() {

			let input = "aaa: you hhh";
			let expected = DeviceDescription {
				name: ['a','a','a'],
				outputs: vec![ ['y','o','u'] , ['h','h','h'] ],
			};

			assert!(let Ok(actual) = parser::device(input));
			assert!( actual == expected );
		}
	}

}

use funty::{AtMost32, Integral, Unsigned};
use heapless::{deque::IntoIter, vec};
use num::iter::RangeStepInclusive;
use parse::{DeviceDescription,parser};
use petgraph::{Direction::Outgoing, acyclic::Acyclic, csr::IndexType, graph::{self, DiGraph, NodeIndex}, visit::{GraphBase, IntoNeighborsDirected, NodeCount, Visitable}};

#[derive(Clone,Copy,PartialEq,PartialOrd,Eq,Ord,Default)]
#[derive(From,Display)]
#[display("{}",unsafe { str::from_utf8_unchecked(&self.0) })]
struct Name([u8;3]);

impl std::fmt::Debug for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"Name[{}]",self)
	}
}

fn name(n:impl Into<Name>) -> Name {
	n.into()
}

impl From<[char;3]> for Name {
	fn from(value: [char;3]) -> Self {
		value.map(|c| c as u8).into()
	}
}

impl From<&str> for Name {
	fn from(value: &str) -> Self {
		value.chars().collect_array().unwrap().into()
	}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
#[derive(Deref,DerefMut,Default,Into)]
struct Id(usize);

unsafe impl IndexType for Id {
	fn new(x: usize) -> Self {
		Id(x)
	}

	fn index(&self) -> usize {
		self.0
	}

	fn max() -> Self {
		Id(4096)
	}
}

#[derive(Debug)]
#[derive(Index,Deref)]
struct DeviceDict(Vec<Name>);

impl<I> From<&I> for DeviceDict
where I:Clone+Iterator<Item=DeviceDescription>
{
	fn from(devs: &I) -> Self {

		use itertools::chain;
		use std::iter::once;

		// list of inputs
		let inputs = devs.clone().map(|desc| name(desc.name));

		// "out" is only found in the outputs,
		// we must add it manually
		let sorted = chain(once(Name::from("out")),inputs).sorted();

		Self(sorted.collect_vec())
	}
}

impl DeviceDict {

	/// Given a `Name`, returns an `Id` (index in the vec)
	/// if the name is in the dict
	pub fn id(&self, n:impl Into<Name>) -> Option<Id> {
		self.0.binary_search(&n.into()).ok().map(Id)
	}
}

fn graph_edges<T>(descriptions: T, dict:&DeviceDict) -> anyhow::Result<Vec<(Id,Id)>>
	where T:Clone+Iterator<Item=DeviceDescription>
{

	use anyhow::{anyhow,Result,Ok,Error};

	let edges = Vec::<(Id,Id)>::new();

	let id = |input:[char;3]| -> Option<Id> {
		dict.id(input)
	};

	let mut max_branches:usize = 0;

	descriptions.into_iter()
		.map(|desc| {

			use std::iter::repeat_n;
			use std::cmp::max;

			let maybe_input = id(desc.name);
			let outputs_maybe = desc.outputs.into_iter().map(id);
			let count = outputs_maybe.len();

			max_branches = max(max_branches,count);

			repeat_n(maybe_input,count)
				.zip(outputs_maybe)
				.map(|(mi,mo)| {
					let i = mi.context("Invalid input")?;
					let o = mo.context("Invalid output")?;
					Ok((i,o))
				})
		})
		.flatten()
		.fold_ok(edges,|mut edges, (i,o)| {
			edges.push((i,o));
			edges
		})
}

fn all_simple_paths<G,N>(graph:G,start:N,end:N) -> impl Iterator<Item = Vec<<G as GraphBase>::NodeId>>
	where
		G: NodeCount + IntoNeighborsDirected,
		G::NodeId: std::cmp::Eq + std::hash::Hash,
		N:Into<G::NodeId>,
{

	petgraph::algo::all_simple_paths::<Vec<_>, _, std::hash::RandomState>(graph,start.into(),end.into(),0,None)
}

type DGraph = DiGraph<(),(),Id>;

struct DeviceMap { graph: Acyclic<DGraph>, dict:DeviceDict }

impl TryFrom<&str> for DeviceMap {
	type Error = anyhow::Error;

	fn try_from(input: &str) -> Result<Self, Self::Error> {

		let descriptions = parse(input,parser::device);
		let dict = DeviceDict::from(&descriptions);
		let edges = graph_edges(descriptions, &dict).expect("A complete list of graph edges");
		let graph = Acyclic::try_from_graph(DGraph::from_edges(edges)).expect("Graph should be acyclic");

		Ok(DeviceMap { dict, graph })
	}
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 11;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> anyhow::Result<impl Display> {

		let DeviceMap { graph, dict } = DeviceMap::try_from(input)
			.context("Failed to build DeviceMap")?;

		let start = dict.id("you").context("Invalid node name")?;
		let end   = dict.id("out").context("Invalid node name")?;
		let paths = all_simple_paths(&graph, start,end);

		paths.count().to_string().ok()
	}
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 11;
	const PART: Part = Part::Part2;

	// This proposed solution splits the problem in three parts,
	// finding paths from svr->fft, fft->dac and finally dac->out
	// Then, by multiplying those 3 counts, we'll get the total
	// number of paths from svr->out
	fn solve(input:&str) -> anyhow::Result<impl Display> {

		use petgraph::{algo, prelude::*};
		use std::hash::RandomState;

		fn node(dict:&DeviceDict,name:&str) -> NodeIndex<Id> {
			dict.id(name).map(NodeIndex::from).expect("Invalid node name")
		}

		let device_graph = DeviceMap::try_from(input)
			.context("Failed to build DeviceMap")?;

		let DeviceMap { graph, ref dict, .. } = device_graph;

		// To speed things up, we'll remove the nodes we encounter when
		// searching paths from dac->out.
		// We can do that, because the graph is acyclic:
		// Any reachable node from dac that reached back to ttf would create a cycle.
		let mut sgraph:StableDiGraph<(),(),Id> = graph.into_inner().into();

		let dac_out = {
			let from = node(dict,"dac");
			let to = node(dict,"out");
			let paths = all_simple_paths(&sgraph, from, to);
			let mut reject = BTreeSet::<NodeIndex<Id>>::new();

			let total = paths.inspect(|p| {
				p.iter().for_each(|n| { reject.insert(*n); });
			}).count();

			// We avoid removing dac
			reject.remove(&from);
			reject.into_iter().for_each(|n| { sgraph.remove_node(n); } );

			total
		};

		// We'll use another trick, so this time we won't remove any node.

		let fft_dac = {
			let from = node(dict,"fft");
			let to = node(dict,"dac");
			let paths = all_simple_paths(&sgraph, from, to);

			paths.count()
		};

		// Reversing the graph, we can instead count the paths from ttf->svr,
		// reducing our seach space.
		sgraph.reverse();

		let svr_fft = {

			let from = node(dict,"fft");
			let to = node(dict,"svr");
			let paths = all_simple_paths(&sgraph, from, to);
			paths.count()
		};

		let total = svr_fft * fft_dac * dac_out;

		total.to_string().ok()
	}
}

submit! { Part1, Part2 }

#[cfg(test)]
mod test {

	use super::*;
	use assert2::{assert,check};

	const EXAMPLE_INPUT_PART1:&str = indoc! {
		r#"
		aaa: you hhh
		you: bbb ccc
		bbb: ddd eee
		ccc: ddd eee fff
		ddd: ggg
		eee: out
		fff: out
		ggg: out
		hhh: ccc fff iii
		iii: out
		"#
	};

	const EXAMPLE_INPUT_PART2:&str = indoc! {
		r#"
		svr: aaa bbb
		aaa: fft
		fft: ccc
		bbb: tty
		tty: ccc
		ccc: ddd eee
		ddd: hub
		hub: fff
		eee: dac
		dac: fff
		fff: ggg hhh
		ggg: out
		hhh: out
		"#
	};

	#[test]
	fn test_name() {

		let expected = Name([b'a',b'b',b'c']);

		assert!(Name::from("abc") == expected);
		assert!(Name::from(['a','b','c']) == expected);
		assert!(Name::from([b'a',b'b',b'c']) == expected);

		assert!(name("abc") == expected);
		assert!(name(['a','b','c']) == expected);
		assert!(name([b'a',b'b',b'c']) == expected);
	}

	#[test]
	fn test_device_dict() {

		let devices = parse(EXAMPLE_INPUT_PART1,parser::device).collect_vec();
		let len = devices.len();

		// shuffle them a bit first, since its almost totally sorted.
		let devices = devices.into_iter().rev().skip(2).cycle().take(len);

		let dict = DeviceDict::from(&devices);

		assert!(dict[0] == name("aaa"));
		assert!(let Some(id) = dict.id("aaa"));
		check!(id == Id(0));

		assert!(dict[9] == name("out"));
		assert!(let Some(id) = dict.id("out"));
		check!(id == Id(9));

		assert!(dict[10] == name("you"));
		assert!(let Some(id) = dict.id("you"));
		check!(id == Id(10));
	}

	#[test]
	fn test_graph_edges() {
		let descriptions = parse(EXAMPLE_INPUT_PART1,parser::device);
		let dict = DeviceDict::from(&descriptions);

		let edges = graph_edges(descriptions, &dict).expect("A complete list of graph edges");

		// The Dict sorts the devices by name
		const AAA:Id = Id(0);
		const BBB:Id = Id(1);
		const CCC:Id = Id(2);
		const DDD:Id = Id(3);
		const EEE:Id = Id(4);
		const FFF:Id = Id(5);
		const GGG:Id = Id(6);
		const HHH:Id = Id(7);
		const III:Id = Id(8);
		const OUT:Id = Id(9);
		const YOU:Id = Id(10);

		let expected = [
			(AAA,YOU), (AAA,HHH),
			(YOU,BBB), (YOU,CCC),
			(BBB,DDD), (BBB,EEE),
			(CCC,DDD), (CCC,EEE), (CCC,FFF),
			(DDD,GGG),
			(EEE,OUT),
			(FFF,OUT),
			(GGG,OUT),
			(HHH,CCC), (HHH,FFF), (HHH,III),
			(III,OUT)
		];

		assert_equal(edges, expected);
	}

	#[test]
	fn test_part1_example() {
		assert!(let Ok(actual) = Part1::solve(EXAMPLE_INPUT_PART1));
		check!(actual.to_string() == "5");
	}

	#[test]
	fn test_part2_example() {
		assert!(let Ok(actual) = Part2::solve(EXAMPLE_INPUT_PART2));
		check!(actual.to_string() == "2");
	}
}
