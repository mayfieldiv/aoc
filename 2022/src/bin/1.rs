use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut sums: Vec<u32> = input
        .trim()
        .split("\n\n")
        .map(|it| {
            dbg!(it);
            it.trim()
                .split("\n")
                .map(|it| it.parse::<u32>().unwrap())
                .reduce(|acc, it| acc + it)
                .unwrap()
        })
        .collect();

    sums.sort();
    sums.reverse();

    println!("{sums:?}, {}", sums[0] + sums[1] + sums[2]);
    Ok(())
}
