#![warn(rust_2024_incompatible_pat)]

pub const YEAR:i32 = 2025;

#[cfg(test)]
pub mod days;

#[allow(unused)]
#[derive(Debug)]
pub enum AppError {
	BadConfiguration(String),
	IncorrectSolution(String)
}

pub fn cookie() -> Result<String,AppError> {
	std::env::var("COOKIE")
		.map_err(|e| {
			let msg = format!("Cookie error! {e:?}");
			AppError::BadConfiguration(msg)
		})
}
