use std::collections::HashSet;

use anyhow::Result;
use aoc_2024::read_input;
use itertools::Itertools as _;

fn main() -> anyhow::Result<()> {
	let map = read_input()
		.lines()
		.map(|line| line.bytes().collect_vec())
		.collect_vec();

	if cfg!(feature = "part1") {
		let mut map = map;
		let mut current = find_start(&map);
		map[current.y][current.x] = b'X';
		while let Ok(next) = next(&current, &map) {
			current = next;
			map[current.y][current.x] = b'X';
		}
		println!("{}", count_x(&map));
		print(&map);
	} else {
		let start = find_start(&map);
		let mut loops = 0;

		for y in 0..map.len() {
			println!("{}/{}", y, map.len());
			for x in 0..map[y].len() {
				if map[y][x] == b'.' {
					let mut map = map.clone();
					map[y][x] = b'#';

					let mut steps = HashSet::new();
					let mut current = start.clone();

					loop {
						if steps.contains(&current) {
							loops += 1;
							break;
						}
						if let Ok(next) = next(&current, &map) {
							steps.insert(current);
							current = next;
						} else {
							break;
						}
					}
				}
			}
		}
		println!("{}", loops);
	}

	Ok(())
}

fn find_start(map: &[Vec<u8>]) -> Step {
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x] != b'.' && map[y][x] != b'#' {
				let dir = match map[y][x] {
					b'^' => Direction::Up,
					b'v' => Direction::Down,
					b'<' => Direction::Left,
					b'>' => Direction::Right,
					_ => unreachable!(),
				};
				return Step { y, x, dir };
			}
		}
	}
	unreachable!()
}

fn count_x(map: &[Vec<u8>]) -> usize {
	let mut result = 0;
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x] == b'X' {
				result += 1;
			}
		}
	}
	result
}

#[allow(dead_code)]
fn print(map: &[Vec<u8>]) {
	for line in map {
		println!("{}", std::str::from_utf8(&line).unwrap());
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Step {
	y: usize,
	x: usize,
	dir: Direction,
}

impl Step {
	fn with_dir(&self, dir: Direction) -> Self {
		Self { dir, ..*self }
	}
}

fn next(step: &Step, map: &[Vec<u8>]) -> Result<Step> {
	fn done() -> anyhow::Error {
		anyhow::anyhow!("out of bounds")
	}

	Ok(match step.dir {
		Direction::Up => {
			let (y, x) = (step.y.checked_sub(1).ok_or_else(done)?, step.x);
			if map[y][x] == b'#' {
				step.with_dir(Direction::Right)
			} else {
				Step { y, x, dir: step.dir }
			}
		}
		Direction::Right => {
			let (y, x) = (step.y, step.x + 1);
			if x >= map[y].len() {
				Err(done())?
			} else if map[y][x] == b'#' {
				step.with_dir(Direction::Down)
			} else {
				Step { y, x, dir: step.dir }
			}
		}
		Direction::Down => {
			let (y, x) = (step.y + 1, step.x);
			if y >= map.len() {
				Err(done())?
			} else if map[y][x] == b'#' {
				step.with_dir(Direction::Left)
			} else {
				Step { y, x, dir: step.dir }
			}
		}
		Direction::Left => {
			let (y, x) = (step.y, step.x.checked_sub(1).ok_or_else(done)?);
			if map[y][x] == b'#' {
				step.with_dir(Direction::Up)
			} else {
				Step { y, x, dir: step.dir }
			}
		}
	})
}
