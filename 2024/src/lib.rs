use std::io::{stdin, Read as _};

pub fn read_input() -> String {
	let mut input = String::new();
	stdin().read_to_string(&mut input).unwrap();
	input.trim().to_string()
}

pub fn print_map(map: &[Vec<u8>]) {
	for line in map {
		println!("{}", std::str::from_utf8(&line).unwrap());
	}
}
