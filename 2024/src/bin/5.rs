use std::collections::{HashMap, HashSet};

fn main() -> anyhow::Result<()> {
	let mut input = String::new();
	std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
	let mut rules = HashMap::<u8, HashSet<u8>>::new();
	let (raw_rules, updates) = input.trim().split_once("\n\n").unwrap();
	for (before, after) in raw_rules.lines().map(|line| line.split_once('|').unwrap()) {
		rules.entry(before.parse()?).or_default().insert(after.parse()?);
	}

	let mut result = 0u64;
	for update in updates.lines() {
		let pages: Vec<u8> = update.split(',').map(|s| s.parse().unwrap()).collect();
		fn check(rules: &HashMap<u8, HashSet<u8>>, pages: &[u8]) -> bool {
			for (i, &page) in pages.iter().enumerate() {
				for later_idx in i + 1..pages.len() {
					let later = pages[later_idx];
					if let Some(later_rules) = rules.get(&later) {
						if later_rules.contains(&page) {
							return false;
						}
					}
				}
			}
			true
		}
		if check(&rules, &pages) {
			let middle = pages[pages.len() / 2];
			result += middle as u64;
		}
	}

	println!("{}", result);
	Ok(())
}
