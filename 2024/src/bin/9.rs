#![feature(let_chains)]

use aoc_2024::{print_map, read_input};
use itertools::Itertools as _;
use std::{
	collections::{HashMap, HashSet},
	fmt::Debug,
};

fn main() -> anyhow::Result<()> {
	let input = read_input()
		.trim()
		.chars()
		.map(|c| c.to_digit(10).unwrap() as u8)
		.collect_vec();

	#[derive(Clone)]
	enum Node {
		File { id: usize, file_size: usize },
		Free, // { free_size: usize },
		Nothing,
	}

	impl Debug for Node {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Node::File { id, file_size } => write!(f, "{}", id.to_string().repeat(*file_size)),
				Node::Free => write!(f, "."),
				Node::Nothing => Ok(()),
				// Node::Free { free_size } => write!(f, "{}", ".".repeat(*free_size)),
				// Node::FileAndFree {
				// 	id,
				// 	file_size,
				// 	free_size,
				// } => write!(f, "{}{}", id.to_string().repeat(*file_size), ".".repeat(*free_size)),
			}
		}
	}

	let mut nodes = Vec::new();
	for (i, &num) in input.iter().enumerate() {
		if i % 2 == 0 {
			nodes.push(Node::File {
				id: i / 2,
				file_size: num as usize,
			});
		} else {
			for _ in 0..num as usize {
				nodes.push(Node::Free);
			}
			// nodes.push(Node::Free {
			// 	free_size: num as usize,
			// });
		}
	}
	// nodes.push(Node::Free { free_size: 0 });

	println!("{:#?}", nodes);

	fn print_nodes(nodes: &[Node]) {
		for node in nodes {
			print!("{:?}", node);
		}
		println!();
	}

	print_nodes(&nodes);

	let mut from_start = 0;
	let mut from_end = nodes.len() - 1;
	let mut previous_file_index = 0;

	while from_start < from_end {
		// nodes[from_end] = normalize(nodes[from_end].clone());
		if matches!(nodes[from_end], Node::File { file_size: 0, .. }) {
			nodes[from_end] = Node::Nothing;
		}

		// println!("{:?} {:?}", nodes[from_start], nodes[from_end]);
		// print_nodes(&nodes);

		if !matches!(nodes[from_start], Node::Free) {
			from_start += 1;
			previous_file_index = from_start;
		} else if !matches!(nodes[from_end], Node::File { file_size: 1.., .. }) {
			from_end -= 1;
		} else {
			if let Node::File { file_size, id } = nodes[from_end] {
				nodes[from_end] = Node::File {
					id,
					file_size: file_size - 1,
				};
				if let Node::File {
					id: previous_id,
					file_size: previous_file_size,
				} = nodes[previous_file_index]
					&& previous_id == id
				{
					nodes[previous_file_index] = Node::File {
						id: previous_id,
						file_size: previous_file_size + 1,
					};
					nodes[from_start] = Node::Nothing;
					from_start += 1;
				} else {
					nodes[from_start] = Node::File { id, file_size: 1 };
					previous_file_index = from_start;
				}

				nodes[from_end] = Node::File {
					id,
					file_size: file_size - 1,
				};
				nodes.push(Node::Free);
			} else {
				panic!("Expected file");
			}
		}
	}

	print_nodes(&nodes);

	let mut result = 0;
	let mut position = 0;
	for node in nodes {
		if let Node::File { id, file_size } = node {
			for _ in 0..file_size {
				result += id * position;
				position += 1;
			}
		}
	}

	println!("{}", result);

	Ok(())
}
