use aoc_2024::{print_map, read_input};
use itertools::Itertools as _;
use std::{
	collections::{HashMap, HashSet},
	fmt::Debug,
};

fn main() -> anyhow::Result<()> {
	let map = read_input()
		.lines()
		.map(|line| line.bytes().collect_vec())
		.collect_vec();

	#[derive(PartialEq, Eq, Hash, Clone)]
	struct Node {
		y: isize,
		x: isize,
		value: u8,
	}

	impl Debug for Node {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "('{}': {},{})", self.value as char, self.y, self.x)
		}
	}

	let mut nodes_map = HashMap::new();

	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x] != b'.' {
				let value = map[y][x];
				nodes_map.entry(value).or_insert_with(Vec::new).push(Node {
					y: y as isize,
					x: x as isize,
					value,
				});
			}
		}
	}

	println!("{:?}", nodes_map);
	let mut antis = HashSet::new();
	for nodes in nodes_map.values() {
		for (i, node) in nodes.iter().enumerate() {
			for (j, other) in nodes.iter().enumerate() {
				if i != j {
					let (top, bottom) = if node.y < other.y { (node, other) } else { (other, node) };

					let up_diff = bottom.y - top.y;
					let left_diff = bottom.x - top.x;
					let down_diff = top.y - bottom.y;
					let right_diff = top.x - bottom.x;

					for i in if cfg!(feature = "part1") { 1..2 } else { 0..999 } {
						let anti_top = Node {
							y: top.y - up_diff * i,
							x: top.x - left_diff * i,
							value: b'#',
						};
						if anti_top.y >= 0
							&& anti_top.x >= 0
							&& anti_top.y < map.len() as isize
							&& anti_top.x < map[0].len() as isize
						{
							antis.insert(anti_top);
						} else {
							break;
						}
					}

					for i in if cfg!(feature = "part1") { 1..2 } else { 0..999 } {
						let anti_bottom = Node {
							y: bottom.y - down_diff * i,
							x: bottom.x - right_diff * i,
							value: b'#',
						};

						if anti_bottom.y >= 0
							&& anti_bottom.x >= 0
							&& anti_bottom.y < map.len() as isize
							&& anti_bottom.x < map[0].len() as isize
						{
							antis.insert(anti_bottom);
						} else {
							break;
						}
					}
				}
			}
		}
	}

	println!("{:?}", antis.len());

	let mut map = map.clone();
	for node in antis {
		map[node.y as usize][node.x as usize] = b'#';
	}
	print_map(&map);

	Ok(())
}
