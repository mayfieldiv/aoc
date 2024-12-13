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

	println!("{nodes}");

	let mut left = nodes.first().unwrap().idx;
	let mut right = nodes.last().unwrap().idx;

	while left != right {
		// println!("{nodes}");
		let left_node = nodes.get(left).unwrap();
		let right_node = nodes.get(right).unwrap();
		if !left_node.is_free() {
			left = left_node.next(&nodes).unwrap().idx;
		} else if !right_node.is_file() {
			right = right_node.prev(&nodes).unwrap().idx;
		} else {
			// println!("left: {:?}", left);
			// println!("right: {:?}", right);
			let free_size = left_node.size;
			let file_size = right_node.size;
			if free_size >= file_size {
				nodes.insert_after(left_node.prev_idx, right_node.meta, file_size);
				nodes.get_mut(left).size -= file_size;
				nodes.get_mut(right).size = 0;
			} else {
				nodes.insert_after(left_node.prev_idx, right_node.meta, free_size);
				nodes.get_mut(left).size = 0;
				nodes.get_mut(right).size -= free_size;
			}
		}
	}

	println!("{nodes}");

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
	use std::fmt::{Debug, Display};

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub struct Node {
		pub idx: usize,
		pub prev_idx: Option<usize>,
		pub next_idx: Option<usize>,
		pub size: usize,
		pub meta: NodeMeta,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum NodeMeta {
		File { id: usize },
		Free,
	}

	impl Display for Node {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self.meta {
				NodeMeta::File { id } => write!(f, "{}", id.to_string().repeat(self.size)),
				NodeMeta::Free => write!(f, "{}", ".".repeat(self.size)),
			}
		}
	}

	impl Node {
		pub fn is_free(&self) -> bool {
			matches!(self.meta, NodeMeta::Free) && self.size > 0
		}

		pub fn is_file(&self) -> bool {
			matches!(self.meta, NodeMeta::File { .. }) && self.size > 0
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

	impl Display for NodeCollection {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			for node in self.iter() {
				write!(f, "{}", node)?;
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

		pub fn insert_after(&mut self, idx: Option<usize>, meta: NodeMeta, size: usize) {
			let new_idx = self.nodes.len();
			let next_idx = idx.and_then(|i| self.nodes[i].next_idx);

			self.nodes.push(Node {
				idx: new_idx,
				size,
				meta,
				prev_idx: idx,
				next_idx,
			});

			if let Some(idx) = idx {
				self.nodes[idx].next_idx = Some(new_idx);
			}
			if let Some(next_idx) = next_idx {
				self.nodes[next_idx].prev_idx = Some(new_idx);
			}
		}

		pub fn get_mut(&mut self, idx: usize) -> &mut Node {
			&mut self.nodes[idx]
		}

		pub fn push(&mut self, meta: NodeMeta, size: usize) {
			let new_idx = self.nodes.len();
			let mut node = Node {
				idx: new_idx,
				size,
				meta,
				prev_idx: None,
				next_idx: None,
			};

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
