fn main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let lines: Vec<_> = input.trim().split('\n').collect();

	#[derive(Debug)]
	enum Op {
		Add,
		Mul,
		Concat,
	}

	let mut result = 0u64;

	for equation in lines {
		let (test, rest) = equation.split_once(':').unwrap();
		let test: u64 = test.parse()?;
		let numbers: Vec<u64> = rest.split_whitespace().map(|s| s.parse().unwrap()).collect();

		let mut stack = Vec::new();
		stack.push((0u64, Op::Add, 0usize));
		while let Some((total, op, i)) = stack.pop() {
			let num = numbers[i];
			if let Some(new_total) = match op {
				Op::Add => total.checked_add(num),
				Op::Mul => total.checked_mul(num),
				Op::Concat => format!("{total}{num}").parse().ok(),
			} {
				// println!("{}: {} {:?} {} => {}", test, total, op, num, new_total);
				if i == numbers.len() - 1 {
					if new_total == test {
						result += test;
						break;
					}
				} else if new_total <= test {
					stack.push((new_total, Op::Add, i + 1));
					stack.push((new_total, Op::Mul, i + 1));
					if !cfg!(feature = "part1") {
						stack.push((new_total, Op::Concat, i + 1));
					}
				}
			}
		}
	}

	println!("{}", result);

	Ok(())
}
