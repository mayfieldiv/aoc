#![feature(let_chains)]

use aoc_2024::read_input;
use itertools::Itertools as _;
use node::{NodeCollection, NodeMeta};

fn main() -> anyhow::Result<()> {
	let input = read_input()
		.trim()
		.chars()
		.map(|c| c.to_digit(10).unwrap() as u8)
		.collect_vec();

	let mut nodes = NodeCollection::new();

	for (i, &num) in input.iter().enumerate() {
		let meta = if i % 2 == 0 {
			NodeMeta::File { id: i / 2 }
		} else {
			NodeMeta::Free
		};

		nodes.push(meta, num as usize);
	}

	println!("{:?}", nodes);

	let mut left = nodes.first().unwrap();
	let mut right = nodes.last().unwrap();
	// let mut previous_file_index = 0;

	while left != right {
		// nodes[from_end] = normalize(nodes[from_end].clone());
		// if matches!(nodes[from_end], Node::File { file_size: 0, .. }) {
		// 	nodes[from_end] = Node::Nothing;
		// }

		// println!("{:?} {:?}", nodes[from_start], nodes[from_end]);
		println!("{:?}", nodes);

		if !left.is_free() {
			left = left.next(&nodes).unwrap();
			// previous_file_index = left_index;
		} else if right.size == 0 {
			right = right.prev(&nodes).unwrap();
		} else {
			println!("left: {:?} {:?}", left.size, left.meta);
			println!("right: {:?} {:?}", right.size, right.meta);
			if left.size >= right.size {
				left.size -= right.size;
				right.size = 0;
			}
			todo!()
			// if let Node::File { file_size, id } = nodes[right_index] {
			// 	nodes[right_index] = Node::File {
			// 		id,
			// 		file_size: file_size - 1,
			// 	};
			// 	if let Node::File {
			// 		id: previous_id,
			// 		file_size: previous_file_size,
			// 	} = nodes[previous_file_index]
			// 		&& previous_id == id
			// 	{
			// 		nodes[previous_file_index] = Node::File {
			// 			id: previous_id,
			// 			file_size: previous_file_size + 1,
			// 		};
			// 		left = Node::Nothing;
			// 		left_index += 1;
			// 	} else {
			// 		left = Node::File { id, file_size: 1 };
			// 		previous_file_index = left_index;
			// 	}

			// 	nodes[right_index] = Node::File {
			// 		id,
			// 		file_size: file_size - 1,
			// 	};
			// 	nodes.push(Node::Free);
			// } else {
			// 	panic!("Expected file");
			// }
		}
	}

	println!("{:?}", nodes);

	let mut result = 0;
	let mut position = 0;
	for node in nodes.iter() {
		if let NodeMeta::File { id } = node.meta {
			for _ in 0..node.size {
				result += id * position;
				position += 1;
			}
		}
	}

	println!("{}", result);

	Ok(())
}

mod node {
	use std::fmt::Debug;

	#[derive(Clone, PartialEq, Eq)]
	pub struct Node {
		prev_idx: Option<usize>,
		next_idx: Option<usize>,
		pub size: usize,
		pub meta: NodeMeta,
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum NodeMeta {
		File { id: usize },
		Free,
	}

	impl Debug for Node {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self.meta {
				NodeMeta::File { id } => write!(f, "{}", id.to_string().repeat(self.size)),
				NodeMeta::Free => write!(f, "{}", ".".repeat(self.size)),
			}
		}
	}

	impl Node {
		pub fn is_free(&self) -> bool {
			matches!(self.meta, NodeMeta::Free)
		}

		pub fn prev<'a>(&self, nodes: &'a NodeCollection) -> Option<&'a Node> {
			self.prev_idx.and_then(|idx| nodes.get(idx))
		}

		pub fn next<'a>(&self, nodes: &'a NodeCollection) -> Option<&'a Node> {
			self.next_idx.and_then(|idx| nodes.get(idx))
		}
	}

	pub struct NodeCollection {
		nodes: Vec<Node>,
		first: Option<usize>,
		last: Option<usize>,
	}

	impl Debug for NodeCollection {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			for node in &self.nodes {
				write!(f, "{:?}", node)?;
			}
			Ok(())
		}
	}

	impl NodeCollection {
		pub fn new() -> Self {
			Self {
				nodes: Vec::new(),
				first: None,
				last: None,
			}
		}

		pub fn get(&self, idx: usize) -> Option<&Node> {
			self.nodes.get(idx)
		}

		pub fn first(&self) -> Option<&Node> {
			self.first.map(|idx| &self.nodes[idx])
		}

		pub fn last(&self) -> Option<&Node> {
			self.last.map(|idx| &self.nodes[idx])
		}

		pub fn push(&mut self, meta: NodeMeta, size: usize) {
			let mut node = Node {
				size,
				meta,
				prev_idx: None,
				next_idx: None,
			};

			let new_idx = self.nodes.len();
			if let Some(last_idx) = self.last {
				self.nodes[last_idx].next_idx = Some(new_idx);
				node.prev_idx = Some(last_idx);
			} else {
				self.first = Some(new_idx);
			}

			self.last = Some(new_idx);
			self.nodes.push(node);
		}

		pub fn iter(&self) -> NodeCollectionIterator {
			NodeCollectionIterator::new(&self.nodes)
		}
	}

	pub struct NodeCollectionIterator<'a> {
		nodes: &'a [Node],
		next: Option<&'a Node>,
		started: bool,
	}

	impl<'a> NodeCollectionIterator<'a> {
		fn new(nodes: &'a [Node]) -> Self {
			Self {
				nodes,
				next: None,
				started: false,
			}
		}
	}

	impl<'a> Iterator for NodeCollectionIterator<'a> {
		type Item = &'a Node;

		fn next(&mut self) -> Option<Self::Item> {
			if !self.started {
				self.started = true;
				let current = self.nodes.first();
				self.next = current.and_then(|it| it.next_idx).and_then(|it| self.nodes.get(it));
				current
			} else {
				if let Some(current) = self.next {
					self.next = current.next_idx.and_then(|it| self.nodes.get(it));
					Some(current)
				} else {
					None
				}
			}
		}
	}
}
