use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|it| it.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    dbg!(&lines);
}
