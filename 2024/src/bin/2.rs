#![feature(array_windows)]

fn _main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let lines: Vec<_> = input.trim().split('\n').collect();

	let mut result = 0;
	for report in lines {
		let report = report
			.split_whitespace()
			.map(|s| s.parse::<i32>().unwrap())
			.collect::<Vec<_>>();

		let up = report[0] < report[1];
		let mut safe = true;

		for [a, b] in report.array_windows::<2>() {
			if up && a > b || !up && a < b || a == b || (a - b).abs() > 3 {
				safe = false;
				break;
			}
		}

		if safe {
			result += 1;
		}
	}

	println!("{}", result);
	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let lines: Vec<_> = input.trim().split('\n').collect();

	fn is_safe(report: &Vec<i32>) -> bool {
		let up = report[0] < report[1];
		for [a, b] in report.array_windows::<2>() {
			if up && a > b || !up && a < b || a == b || (a - b).abs() > 3 {
				return false;
			}
		}
		true
	}

	let mut result = 0;
	for report in lines {
		let report = report
			.split_whitespace()
			.map(|s| s.parse::<i32>().unwrap())
			.collect::<Vec<_>>();

		if is_safe(&report) {
			result += 1;
		} else {
			for i in 0..report.len() {
				let mut new_report = report.clone();
				new_report.remove(i);
				if is_safe(&new_report) {
					result += 1;
					break;
				}
			}
		}
	}

	println!("{}", result);
	Ok(())
}
