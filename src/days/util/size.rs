pub trait HasSize {
	fn size(&self)->Size;
}

#[derive(Debug,Clone,Copy)]
pub struct Size {
	pub width:usize,
	pub height:usize,
}

impl From<(usize,usize)> for Size {
	fn from(value: (usize,usize)) -> Self {
		Self { width: value.0, height: value.1 }
	}
}
