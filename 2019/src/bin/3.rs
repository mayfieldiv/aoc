#![allow(dead_code)]
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::io::stdin;

fn main() {
    // let answer = solve1().unwrap();
    let answer = solve2().unwrap();
    println!("{}", answer);
}

fn solve1() -> Option<i32> {
    let get_points = || -> Option<HashSet<(i32, i32)>> {
        let mut input = String::new();
        stdin().read_line(&mut input).ok()?;
        let input = input.trim();
        let mut points = HashSet::new();
        let (mut x, mut y) = (0, 0);
        for inst in input.split(',') {
            let (dir, n) = inst.split_at(1);
            let n: i32 = n.parse().ok()?;
            for _ in 0..n {
                match dir {
                    "R" => x += 1,
                    "L" => x -= 1,
                    "U" => y += 1,
                    "D" => y -= 1,
                    _ => unreachable!(),
                }
                points.insert((x, y));
            }
        }
        Some(points)
    };

    let distance = |p: &(i32, i32)| p.0.abs() + p.1.abs();
    let line1 = get_points()?;
    let line2 = get_points()?;
    let intersections = dbg!(line1.intersection(&line2));
    let best = dbg!(intersections.min_by_key(|&p| distance(p)))?;
    Some(distance(best))
}

fn solve2() -> Option<i32> {
    let get_points = || -> Option<_> {
        let mut input = String::new();
        stdin().read_line(&mut input).ok()?;
        let input = input.trim();
        let mut points = HashMap::new();
        let (mut x, mut y) = (0, 0);
        let mut steps = 0;
        for inst in input.split(',') {
            let (dir, n) = inst.split_at(1);
            let n: i32 = n.parse().ok()?;
            for _ in 0..n {
                match dir {
                    "R" => x += 1,
                    "L" => x -= 1,
                    "U" => y += 1,
                    "D" => y -= 1,
                    _ => unreachable!(),
                }
                steps += 1;
                if let Entry::Vacant(entry) = points.entry((x, y)) {
                    entry.insert(steps);
                }
            }
        }
        Some(points)
    };

    let line1 = get_points()?;
    let line2 = get_points()?;
    let points1 = line1.keys().collect::<HashSet<_>>();
    let points2 = line2.keys().collect::<HashSet<_>>();
    let intersections = dbg!(points1.intersection(&points2));
    let distance = |p: &(i32, i32)| line1[p] + line2[p];
    let best = dbg!(intersections.min_by_key(|&p| distance(p)))?;
    Some(distance(best))
}
