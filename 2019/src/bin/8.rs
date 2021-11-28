#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    dbg!(solve2());
}

fn solve1() -> u32 {
    let width = 25;
    let height = 6;
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).unwrap();
    let data: Vec<u32> = data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut layer = 1u32;
    let mut i = 0;
    let mut best = (layer, u32::MAX, HashMap::new());
    while i < data.len() {
        let mut counter = HashMap::new();
        for _row in 0..height {
            for _col in 0..width {
                let pixel = data[i];
                *counter.entry(pixel).or_insert(0u32) += 1;
                i += 1;
            }
        }
        if let Some(&z) = counter.get(&0) {
            if z < best.1 {
                best = dbg!((layer, z, counter));
            }
        }
        layer += 1;
    }

    return best.2[&1] * best.2[&2];
}

fn solve2() -> () {
    let width = 25;
    let height = 6;
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).unwrap();
    let data: Vec<u32> = data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut image = vec![2; width * height];
    let mut i = 0;
    for _row in 0..height {
        for _col in 0..width {
            for chunk in data.chunks_exact(width * height) {
                if chunk[i] != 2 {
                    image[i] = chunk[i];
                    break;
                }
            }
            i += 1;
        }
    }
    for row in 0..height {
        println!(
            "{}",
            image[row * width..(row + 1) * width]
                .iter()
                .map(|&p| if p == 0 { " " } else { "@" })
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
