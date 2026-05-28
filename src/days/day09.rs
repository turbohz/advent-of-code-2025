// https://adventofcode.com/2025/day/9

use std::{any, iter::{Peekable, repeat_n}, ops::{Add, AddAssign, Deref}};

use anyhow::{Context, anyhow, bail};
use assert2::check;
use derive_more::{Deref, DerefMut, Display, Index, IndexMut, derive};
use funty::{Fundamental as _, Integral};
use itertools::Either;

use super::*;

peg::parser! {
	grammar parser() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		pub rule number() -> usize = ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		pub rule coordinate() -> Location
			= x:number() "," y:number() { (x,y).into() }
	}
}

#[derive(Debug,Clone)]
struct Sides<T> {
	n:T,
	e:T,
	s:T,
	w:T
}

impl<T> HasSize for Sides<T>
where T:Integral {
	fn size(&self)->Size {
		let Sides { n, e, s, w } = *self;
		Size {
			width: n.abs_diff(s).as_usize(),
			height: e.abs_diff(w).as_usize(),
		}
	}
}

#[derive(Debug,PartialEq,Eq)]
struct Rect {
	loca: Location,
	size: Size
}

impl HasSize for Rect {
	fn size(&self)->Size {
		self.size
	}
}

impl From<(Location,Location)> for Rect {
	fn from((a,b): (Location,Location)) -> Self {

		use std::cmp::{min,max};

		let x1 = min(a.x,b.x);
		let x2 = max(a.x,b.x);
		let y1 = min(a.y,b.y);
		let y2 = max(a.y,b.y);

		Self {
			loca: (x1,y1).into(),
			size: (x2-x1,y2-y1).into()
		}
	}
}

impl Rect {

	fn sides(&self) -> Sides<usize> {

		let Location { x, y } = self.loca;
		let Size { width, height } = self.size;

		Sides {
			n: y,
			e: x + width,
			s: y + height,
			w: x,
		}
	}
}

/// Returns a rectangle covering all provided locations
fn bounding_rect(locas:&[Location]) -> Rect {

	use itertools::MinMaxResult::MinMax;

	assert!(locas.len() >= 2);

	let xs = locas.iter().map(|v| v.x);
	let ys = locas.iter().map(|v| v.y);
	let MinMax(min_x,max_x) = xs.minmax() else { unreachable!()};
	let MinMax(min_y,max_y) = ys.minmax() else { unreachable!()};

	let loca:Location = (min_x,min_y).into();
	let size:Size = (max_x-min_x,max_y-min_y).into();

	Rect { loca, size }
}

// NOTICE: This is not the standard geometric area, because:
// - The area between a tile and itself is 1
// - The area between two axis aligned tiles is their distance in tiles
fn tile_area(l1:Location,l2:Location) -> usize {
	let width  = 1 + l1.x.abs_diff(l2.x);
	let height = 1 + l1.y.abs_diff(l2.y);
	width * height
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 9;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> anyhow::Result<impl Display> {

		let locas:Vec<Location> = parse(input,parser::coordinate).collect();

		let max_area = locas.into_iter().combinations(2).map(|v| tile_area(v[0],v[1])).max()
			.context("There should be a max area among all")?;

		Ok(max_area)
	}
}

mod check {

	use super::*;

	pub fn all_red_tiles_corners(locas: &[Location])->bool {

		locas.iter()
			.circular_tuple_windows::<(_,_,_)>()
			.all(|(prv,cur,nxt)| {
				// All three **must not** be aligned
				!(
					(prv.x == cur.x && cur.x == nxt.x) ||
					(prv.y == cur.y && cur.y == nxt.y)
				)
			})
	}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl From<(Location,Location)> for Direction {
	fn from(value: (Location,Location)) -> Self {

		use std::cmp::max;

		let (Location { x, y }, Location { x:tip_x , y:tip_y}) = value;

		if x == tip_x {

			assert!(y!=tip_y, "No distinct Locations provided");

			// Vertical, but which way?

			let topmost = max(y,tip_y);

			if topmost == tip_y {
				Direction::Down
			} else {
				Direction::Up
			}

		} else if y == tip_y {

			assert!(x != tip_x, "No distinct Locations provided");

			// Horizontal, but which way?

			let rightmost = max(x,tip_x);

			if rightmost == tip_x {
				Direction::Right
			} else {
				Direction::Left
			}

		} else {
			panic!("We expected only \"straight\" location pairs, creating right angles");
		}
	}
}

impl Direction {
	fn right_turn(&self) -> Self {
		use Direction::*;
		match self {
			Up => Right,
			Right => Down,
			Down => Left,
			Left => Up,
		}
	}
}

impl From<Direction> for V2 {
	fn from(dir: Direction) -> Self {
		use Direction::*;
		match dir {
			Up    => V2([ 0,-1]),
			Right => V2([ 1, 0]),
			Down  => V2([ 0, 1]),
			Left  => V2([-1, 0])
		}
	}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[derive(Index,IndexMut)]
struct V2([i8;2]);
impl Add for V2 {
	type Output = V2;
	fn add(self, rhs: Self) -> Self::Output {
		V2([self[0]+rhs[0],self[1]+rhs[1]])
	}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Curvature {
	Concave,
	Convex,
}

#[derive(Debug,Clone,Copy)]
struct CornerDesc {
	location: Location,
	inside: V2,
	curvature: Curvature,
}

fn into_corners(locas:&[Location]) -> Vec<CornerDesc> {
	locas.iter().copied()
		.circular_tuple_windows::<(Location,Location,Location)>()
		.map(|(prv,cur,nxt)| {

			let before_dir: Direction = (prv,cur).into();
			let after_dir:  Direction = (cur,nxt).into();

			let before_ins: Direction = before_dir.right_turn();
			let after_ins:  Direction = after_dir.right_turn();

			let curvature = if before_ins == after_dir {
				Curvature::Convex
			} else {
				Curvature::Concave
			};

			let inside:V2 = V2::from(before_ins) + V2::from(after_ins);

			CornerDesc { location: cur, inside, curvature }
		})
		.collect()
}

#[derive(Debug,Clone,Copy)]
enum RectCornerKind {
	TopLeft,
	BottomRight,
	Both
}

#[derive(Debug,Clone,Copy)]
struct RectCorner {
	kind: RectCornerKind,
	location: Location,
}

impl TryFrom<&CornerDesc> for RectCorner {
	type Error = anyhow::Error;

	fn try_from(value: &CornerDesc) -> Result<Self, Self::Error> {

		let CornerDesc { location: loca, inside, curvature } = *value;

		let kind = match (inside,curvature) {
			// TopLeft
			( V2([ 1, 1]), _ ) => Ok(RectCornerKind::TopLeft),
			// BottomRight
			( V2([-1,-1]), _ ) => Ok(RectCornerKind::BottomRight),
			// Both
			( V2([-1, 1]), Curvature::Concave ) |
			( V2([ 1,-1]), Curvature::Concave ) => Ok(RectCornerKind::Both),
			// Otherwise ..
			_ => Err(anyhow!("Not a Rect corner"))
		}?;

		Ok(RectCorner { location: loca, kind })
	}
}

fn get_rect_corners(corners:&[CornerDesc]) -> (Vec<RectCorner>,Vec<RectCorner>) {
	use itertools::Either;

	let mut tl:Vec<RectCorner> = vec![];
	let mut br:Vec<RectCorner> = vec![];

	use RectCornerKind::*;

	for rc in corners.iter().flat_map(RectCorner::try_from) {

		match rc.kind {
			TopLeft => tl.push(rc),
			BottomRight => br.push(rc),
			Both => {
				tl.push(RectCorner { kind: TopLeft     , ..rc });
				br.push(RectCorner { kind: BottomRight , ..rc });
			}
		}
	}

	(tl,br)
}

#[derive(Deref)]
struct Edges(Vec<(Location,Location)>);

impl From<&[Location]> for Edges {
	fn from(locas: &[Location]) -> Self {
		let vec:Vec<_> = locas.iter().copied()
			.circular_tuple_windows()
			.collect();
		Self(vec)
	}
}

#[derive(Debug,Clone,Copy)]
struct AreaRect {
	tl: Location,
	br: Location,
}

impl AreaRect {
	fn area(&self) -> usize {
		tile_area(self.tl,self.br)
	}

	// Given a list of _edges_, checks wether none of those cuts through the rect.
	fn filled(&self, edges:&Edges) -> bool {

		// There must be no edge that cuts through it
		!edges.iter().any(|(a,b)| {
			// vertical cut check
			a.y < self.tl.y && b.y > self.br.y && a.x > self.tl.x && a.x < self.br.x ||
			// horizontal cut check
			a.x < self.tl.x && b.x > self.br.x && a.y > self.tl.y && a.y < self.br.y
		})
	}
}

fn get_areas<Itr:Clone+Iterator<Item=RectCorner>>(tl:Itr,br:Itr,all:&[CornerDesc]) -> impl Iterator<Item=AreaRect> {
	tl.cartesian_product(br)
		.filter_map(|(tl,br)|{

			// we only consider br corners that are beneath the tl.

			if tl.location.x > br.location.x || tl.location.y > br.location.y {
				return None;
			}

			// check if any other red tile falls inside the rect
			let has_tile_inside = all.iter().any(|c| {
				let outside = c.location.x <= tl.location.x ||
					c.location.x >= br.location.x ||
					c.location.y <= tl.location.y ||
					c.location.y >= br.location.y;
				!outside
			});

			if has_tile_inside {
				return None;
			}

			Some(AreaRect {
				tl: tl.location,
				br: br.location,
			})
		})
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 9;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> anyhow::Result<impl Display> {

		// NOTICE: Apparently, the _inside_ of the closed shape they create
		// is to the _right_ of the _edge_ a tile and its next create.
		//
		// All red tiles are _corners_ of the shape, and all its edges have right angles.

		let locas:Vec<Location> = parse(input,parser::coordinate).collect();

		// Verify that all red tiles are corners

		let all_corners = check::all_red_tiles_corners(&locas);

		assert!(all_corners);

		let corners = into_corners(&locas);
		let edges:Edges = locas.as_slice().into();

		let (tl_corners,br_corners) = get_rect_corners(&corners);

		let areas:Vec<_> = get_areas(tl_corners.into_iter(), br_corners.into_iter(), &corners).collect();

		let sorted = areas.into_iter()
			.map(|a| (a,a.area()) )
			.sorted_by_key(|a|a.1);

		let max_uncut = sorted.rev().find(|(rect,_)| rect.filled(&edges)).unwrap();

		// Construct an X mirrored versions of the floor,
		// to compute areas of rects that go NE to SW.
		//
		// NOTICE: The sequence must be reversed so all corners inner vec points
		// inside the shape.

		let Sides { w:min,e:max, .. } = bounding_rect(&locas).sides();
		let offset:isize = isize::try_from(min)? + isize::try_from(max)?;

		let locas:Vec<_> = locas.iter()
			.map(|l| {
				let x_isize:isize = -1 * isize::try_from(l.x).unwrap() + offset;
				let x:usize = x_isize.try_into().unwrap();
				Location { x, y: l.y}
			})
			.rev()
			.collect();

		let corners = into_corners(&locas);

		let edges:Edges = locas.as_slice().into();

		let (tl_corners,br_corners) = get_rect_corners(&corners);

		let areas:Vec<_> = get_areas(tl_corners.into_iter(), br_corners.into_iter(), &corners).collect();

		let sorted = areas.into_iter()
			.map(|a| (a,a.area()) )
			.sorted_by_key(|a|a.1);

		let max_uncut_rev = sorted.rev().find(|(rect,_)| rect.filled(&edges)).unwrap();

		std::cmp::max(max_uncut.1,max_uncut_rev.1).ok()
	}
}

submit!(Part1,Part2);

#[cfg(test)]
mod test {

	use super::*;
	use assert2::{assert,check};

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		7,1
		11,1
		11,7
		9,7
		9,5
		2,5
		2,3
		7,3
		"#
	};

	#[test]
   fn test_rectangle() {
      // let
   }

	#[test]
	fn test_bounding_box() {

		let red_tile_locations = parse(EXAMPLE_INPUT,parser::coordinate).collect_vec();

		// .+----------+.
		// .|.....#...#|.
		// .|..........|.
		// .|#....#....|.
		// .|..........|.
		// .|#......#..|.
		// .|..........|.
		// .|.......#.#|.
		// .+----------+.

		let expected:Rect = ((2,1).into(),(11,7).into()).into();
		let actual = bounding_rect(&red_tile_locations);

		assert_eq!(actual,expected);
	}

	#[test]
	fn test_area() {

		assert!(tile_area(Location { x: 7, y: 3 }, Location { x: 7, y: 3 }) == 1);
		assert!(tile_area(Location { x: 7, y: 3 }, Location { x: 2, y: 3 }) == 6);
		assert!(tile_area(Location { x: 2, y: 5 }, Location { x: 11, y: 1 }) == 50);
	}

	#[test]
	fn test_part1_example() {

		assert!(let Ok(actual) = Part1::solve(EXAMPLE_INPUT));
		check!(actual.to_string() == "50");
	}

	#[test]
	fn test_part2_example() {

		assert!(let Ok(actual) = Part2::solve(EXAMPLE_INPUT));
		check!(actual.to_string() == "24");
	}

	#[test]
	fn test_v2() {

		check!(V2([10,-10])[0] == 10);
		check!(V2([10,-10])[1] == -10);
		check!(V2([10,-10]) + V2([-10,10]) == V2([0,0]));
	}
}
