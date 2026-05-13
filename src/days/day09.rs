// https://adventofcode.com/2025/day/9

use super::*;

peg::parser! {
	grammar parser() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		pub rule number() -> usize = ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		pub rule coordinate() -> Location
			= x:number() "," y:number() { (x,y).into() }
	}
}

struct Corners<T> {
	nw:T,
	ne:T,
	se:T,
	sw:T,
}

struct Sides<T> {
	n:T,
	e:T,
	s:T,
	w:T
}

impl From<Rect2> for Corners<Rect2> {
	fn from(rect: Rect2) -> Self {

		let corners = rect.corners();
		let center = rect.center();

		// prevent overlap
		Corners {
			nw: (corners.nw, (center.x , center.y  ).into()).into(),
			ne: (corners.ne, (center.x+1,center.y  ).into()).into(),
			se: (corners.se, (center.x+1,center.y+1).into()).into(),
			sw: (corners.sw, (center.x  ,center.y+1).into()).into(),
		}
	}
}

#[derive(Debug,PartialEq,Eq)]
struct Rect2 {
	loca: Location,
	size: Size
}

impl HasSize for Rect2 {
	fn size(&self)->Size {
		self.size
	}
}

impl From<(Location,Location)> for Rect2 {
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

impl Rect2 {

	fn corners(&self) -> Corners<Location> {
		let Location {x,y} = self.loca;
		let Size {width:w,height:h} = self.size;
		Corners {
			nw: (x,y).into(),
			ne: (x+w,y).into(),
			se: (x+w,y+h).into(),
			sw: (x,y+h).into(),
		}
	}

	fn sides(&self) -> Sides<usize> {

		let Location { x, y } = self.loca;
		let Size { width:w, height:h } = self.size;

		Sides {
			n: y,
			e: x + w,
			s: y + h,
			w: x,
		}
	}

	fn center(&self) -> Location {

		let Sides { n, e, s, w } = self.sides();

		( n.midpoint(s) , w.midpoint(e)).into()
	}
}

// Returns a rectangle covering all 'locations'
fn bounding_box(locas:&[Location]) -> Rect2 {

	use itertools::MinMaxResult::MinMax;

	assert!(locas.len() >= 2);

	let xs = locas.iter().map(|v| v.x);
	let ys = locas.iter().map(|v| v.y);
	let MinMax(min_x,max_x) = xs.minmax() else { unreachable!()};
	let MinMax(min_y,max_y) = ys.minmax() else { unreachable!()};

	let loca:Location = (min_x,min_y).into();
	let size:Size = (max_x-min_x,max_y-min_y).into();

	Rect2 { loca, size }
}

#[cfg(test)]
mod test {
	use super::*;

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

		// ..............
		// ..⌜....#...#..
		// ..............
		// ..#....#......
		// ..............
		// ..#......#....
		// ..............
		// .........#.⌟..
		// ..............

		let expected:Rect2 = ((2,1).into(),(11,7).into()).into();
		let actual = bounding_box(&red_tile_locations);

		assert_eq!(actual,expected);
	}
}

// Sort first in quadrants by closer to corner
// NOTICE: most distant points are those around the diagonal
