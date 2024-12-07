use std::collections::HashMap;

fn main1() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let lines: Vec<_> = input.trim().split('\n').collect();
	let mut left: Vec<i32> = Vec::new();
	let mut right: Vec<i32> = Vec::new();
	for line in lines {
		let (a, b) = line.split_once("   ").unwrap();
		left.push(a.parse::<i32>()?);
		right.push(b.parse::<i32>()?);
	}
	left.sort();
	right.sort();

	let mut result = 0;

	for (l, r) in left.iter().zip(right.iter()) {
		result += (l - r).abs();
	}

	println!("{}", result);
	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let lines: Vec<_> = input.trim().split('\n').collect();

	let mut result = 0;
	let mut left: Vec<i32> = Vec::new();
	let mut right = HashMap::new();

	for line in lines {
		let (l, r) = line.split_once("   ").unwrap();
		let l = l.parse::<i32>()?;
		let r = r.parse::<i32>()?;
		left.push(l);
		*right.entry(r).or_insert(0) += 1;
	}

	for l in left {
		result += l * right.get(&l).unwrap_or(&0);
	}

	println!("{}", result);
	Ok(())
}
