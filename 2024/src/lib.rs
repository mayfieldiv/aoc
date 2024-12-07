use std::io::{stdin, Read as _};

pub fn read_input() -> String {
	let mut input = String::new();
	stdin().read_to_string(&mut input).unwrap();
	input.trim().to_string()
}
