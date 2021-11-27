#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    dbg!(solve2());
}

fn solve1() -> u32 {
    let mut orbits = vec![];
    let mut name_map = HashMap::new();
    loop {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() || line.trim().is_empty() {
            break;
        }
        let line = line.trim();
        let (a, b) = line.split_once(')').unwrap();

        let mut get_or_add_node = |name: &str| -> usize {
            *name_map.entry(name.to_string()).or_insert_with(|| {
                orbits.push(None);
                orbits.len() - 1
            })
        };
        let a = get_or_add_node(a);
        let b = get_or_add_node(b);
        orbits[b] = Some(a);
    }

    fn dfs(index: usize, memo: &mut HashMap<usize, u32>, orbits: &[Option<usize>]) -> u32 {
        if let Some(v) = memo.get(&index) {
            *v
        } else {
            orbits[index].map_or(0, |i| {
                let v = 1 + dfs(i, memo, orbits);
                memo.insert(index, v);
                v
            })
        }
    }

    let mut memo = HashMap::<usize, u32>::new();
    (0..orbits.len()).fold(0, |count, i| count + dfs(i, &mut memo, &orbits))
}

fn solve2() -> u32 {
    let mut orbits = vec![];
    let mut name_map = HashMap::new();
    loop {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() || line.trim().is_empty() {
            break;
        }
        let line = line.trim();
        let (a, b) = line.split_once(')').unwrap();

        let mut get_or_add_node = |name: &str| -> usize {
            *name_map.entry(name.to_string()).or_insert_with(|| {
                orbits.push(None);
                orbits.len() - 1
            })
        };
        let a = get_or_add_node(a);
        let b = get_or_add_node(b);
        orbits[b] = Some(a);
    }

    let mut depths = HashMap::<usize, u32>::new();

    let mut depth = 0;
    let mut index = name_map["YOU"];
    while let Some(next) = orbits[index] {
        depths.insert(index, depth);
        depth += 1;
        index = next;
    }

    let mut depth = 0;
    let mut index = name_map["SAN"];
    loop {
        if let Some(other_depth) = depths.get(&index) {
            dbg!((index, depth, other_depth));
            return depth + other_depth - 2;
        }
        depth += 1;
        index = orbits[index].unwrap();
    }
}
