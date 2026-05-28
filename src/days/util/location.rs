use derive_more::{Display, Sub};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[derive(Display,Sub)]
#[display("[{x},{y}]")]
pub struct Location {
	pub x:usize,
	pub y:usize,
}

impl From<(usize,usize)> for Location {
	fn from((x,y): (usize,usize)) -> Self {
		Self { x, y }
	}
}

impl From<Location> for (usize,usize) {
		fn from(loc: Location) -> Self {
			(loc.x,loc.y)
		}
}

impl Location {

		pub fn down_unchecked(self) -> Self {
			Self { y: self.y+1, ..self }
		}

		pub fn up_unchecked(self) -> Self {
			Self { y: self.y-1, ..self }
		}

		pub fn right_unchecked(self) -> Self {
			Self { x: self.x+1, ..self }
		}

		pub fn left_unchecked(self) -> Self {
			Self { x: self.x-1, ..self }
		}
}
