fn _main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let rows: Vec<_> = input
		.trim()
		.split('\n')
		.map(|s| s.bytes().collect::<Vec<_>>())
		.collect();

	#[derive(Debug)]
	enum Op {
		Add,
		Sub,
		Noop,
	}

	fn check(result: &mut usize, rows: &Vec<Vec<u8>>, y: usize, x: usize, y_operation: Op, x_operation: Op) {
		for (i, &target) in b"XMAS".into_iter().enumerate() {
			let new_y = match y_operation {
				Op::Add if y + i >= rows.len() => return,
				Op::Add => y + i,
				Op::Sub if y < i => return,
				Op::Sub => y - i,
				Op::Noop => y,
			};

			let new_x = match x_operation {
				Op::Add if x + i >= rows[new_y].len() => return,
				Op::Add => x + i,
				Op::Sub if x < i => return,
				Op::Sub => x - i,
				Op::Noop => x,
			};

			if rows[new_y][new_x] != target {
				return;
			}
		}

		*result += 1;
	}

	let mut result = 0;
	for y in 0..rows.len() {
		for x in 0..rows[y].len() {
			check(&mut result, &rows, y, x, Op::Add, Op::Noop);
			check(&mut result, &rows, y, x, Op::Sub, Op::Noop);
			check(&mut result, &rows, y, x, Op::Noop, Op::Add);
			check(&mut result, &rows, y, x, Op::Noop, Op::Sub);
			check(&mut result, &rows, y, x, Op::Add, Op::Add);
			check(&mut result, &rows, y, x, Op::Sub, Op::Sub);
			check(&mut result, &rows, y, x, Op::Add, Op::Sub);
			check(&mut result, &rows, y, x, Op::Sub, Op::Add);
		}
	}

	println!("{}", result);
	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let rows: Vec<_> = input
		.trim()
		.split('\n')
		.map(|s| s.bytes().collect::<Vec<_>>())
		.collect();

	#[derive(Debug)]
	enum Op {
		Add,
		Sub,
	}

	fn check(rows: &Vec<Vec<u8>>, y: usize, x: usize, y_operation: Op, x_operation: Op) -> bool {
		for (i, &target) in b"MAS".into_iter().enumerate() {
			let new_y = match y_operation {
				Op::Add if y + i >= rows.len() => return false,
				Op::Add => y + i,
				Op::Sub if y < i => return false,
				Op::Sub => y - i,
			};

			let new_x = match x_operation {
				Op::Add if x + i >= rows[new_y].len() => return false,
				Op::Add => x + i,
				Op::Sub if x < i => return false,
				Op::Sub => x - i,
			};

			if rows[new_y][new_x] != target {
				return false;
			}
		}

		true
	}

	let mut result = 0;
	for y in 1..rows.len() - 1 {
		for x in 1..rows[y].len() - 1 {
			if check(&rows, y - 1, x - 1, Op::Add, Op::Add) || check(&rows, y + 1, x + 1, Op::Sub, Op::Sub) {
				if check(&rows, y - 1, x + 1, Op::Add, Op::Sub) || check(&rows, y + 1, x - 1, Op::Sub, Op::Add) {
					result += 1;
				}
			}
		}
	}

	println!("{}", result);
	Ok(())
}
