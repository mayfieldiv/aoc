use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

use regex::{Regex, RegexBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let (start, instructions) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r".(.).( |$)").unwrap();
    let mut stacks = Vec::new();
    for line in start.split('\n') {
        for (i, letter) in re.captures_iter(line).map(|x| x[1].to_string()).enumerate() {
            if stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            let c = letter.chars().next().unwrap();
            if c.is_alphabetic() {
                stacks[i].push_back(c);
            }
        }
    }
    dbg!(&stacks);

    let re = RegexBuilder::new(r"move ([[:digit:]]+) from ([[:digit:]]+) to ([[:digit:]]+)$")
        .multi_line(true)
        .build()
        .unwrap();
    for instruction in re.captures_iter(instructions.trim()) {
        let parts: Vec<usize> = instruction
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse().unwrap())
            .collect();
        let (count, from, to) = (parts[0], parts[1] - 1, parts[2] - 1);
        dbg!(&stacks);
        dbg!(count, from, to);

        let items: Vec<_> = stacks[from].drain(0..count).collect();
        dbg!(&items);
        if cfg!(part_1) {
            for item in items {
                stacks[to].push_front(item);
            }
        } else {
            for item in items.into_iter().rev() {
                stacks[to].push_front(item);
            }
        }
    }
    dbg!(&stacks);

    let firsts: Vec<u8> = stacks
        .iter_mut()
        .map(|x| x.pop_front().unwrap() as u8)
        .collect();
    let result = std::str::from_utf8(&firsts).unwrap();
    dbg!(result);

    Ok(())
}
