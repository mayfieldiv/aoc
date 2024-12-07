use anyhow::Result;
use aoc_2024::read_input;
use itertools::Itertools as _;

fn main() -> anyhow::Result<()> {
	let mut map = read_input()
		.lines()
		.map(|line| line.bytes().collect_vec())
		.collect_vec();

	#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
	enum Direction {
		#[default]
		Up,
		Down,
		Left,
		Right,
	}

	#[derive(Debug, PartialEq, Eq, Hash, Default)]
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

	let mut current = Step::default();

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
				current = Step { y, x, dir };
				break;
			}
		}
	}

	#[allow(dead_code)]
	fn print(map: &[Vec<u8>]) {
		for line in map {
			println!("{}", std::str::from_utf8(&line).unwrap());
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

	map[current.y][current.x] = b'X';
	while let Ok(next) = next(&current, &map) {
		current = next;
		map[current.y][current.x] = b'X';
	}

	print(&map);
	let mut result = 0;
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x] == b'X' {
				result += 1;
			}
		}
	}
	println!("{}", result);
	Ok(())
}
