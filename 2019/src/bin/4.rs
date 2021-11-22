#![allow(dead_code)]
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("{}", solve2().unwrap());
}

fn solve1() -> Option<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).ok()?;
    let (start, end) = input.trim().split_once('-')?;
    let start: usize = start.parse().ok()?;
    let end: usize = end.parse().ok()?;
    let mut answer = 0;
    for i in start..=end {
        let valid = |p: usize| -> bool {
            let s = p.to_string();
            let c: Vec<char> = s.chars().collect();
            let mut adj = false;
            for i in 1..c.len() {
                match c[i].cmp(&c[i - 1]) {
                    Ordering::Less => return false,
                    Ordering::Equal => adj = true,
                    Ordering::Greater => {}
                }
            }
            adj
        };
        if valid(i) {
            answer += 1
        }
    }

    Some(answer)
}

fn solve2() -> Option<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).ok()?;
    let (start, end) = input.trim().split_once('-')?;
    let start: usize = start.parse().ok()?;
    let end: usize = end.parse().ok()?;
    let mut answer = 0;
    for i in start..=end {
        let valid = |p: usize| -> bool {
            let s = p.to_string();
            let c: Vec<char> = s.chars().collect();
            let mut adj = false;
            for i in 1..c.len() {
                match c[i].cmp(&c[i - 1]) {
                    Ordering::Less => return false,
                    Ordering::Equal => {
                        if (i < 2 || c[i] != c[i - 2]) && (i + 1 >= c.len() || c[i] != c[i + 1]) {
                            adj = true;
                        }
                    }
                    Ordering::Greater => {}
                }
            }
            adj
        };
        if valid(i) {
            answer += 1
        }
    }

    Some(answer)
}
