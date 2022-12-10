use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let lines: Vec<_> = input.trim().split('\n').collect();
    fn priority(c: &char) -> u32 {
        match c {
            'a'..='z' => *c as u32 - 'a' as u32 + 1,
            'A'..='Z' => *c as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    }

    let mut sum = 0u32;
    if cfg!(part_1) {
        for line in lines {
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<char> = first.chars().collect();
            let second: HashSet<char> = second.chars().collect();
            let shared: Vec<char> = first.intersection(&second).copied().collect();
            sum += priority(shared.first().unwrap());
        }
    } else {
        for chunk in lines.chunks_exact(3) {
            let first: HashSet<char> = chunk[0].chars().collect();
            let second: HashSet<char> = chunk[1].chars().collect();
            let third: HashSet<char> = chunk[2].chars().collect();
            let shared: HashSet<char> = first.intersection(&second).copied().collect();
            let shared: Vec<char> = shared.intersection(&third).copied().collect();
            sum += priority(shared.first().unwrap());
        }
    }
    dbg!(sum);
    Ok(())
}
