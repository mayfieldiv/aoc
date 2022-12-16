use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let total = input
        .trim()
        .split('\n')
        .map(|line| {
            let score = if cfg!(part_1) {
                let (opp, me) = line.split_once(' ').unwrap();
                let shape_score = match me {
                    "X" => 1,
                    "Y" => 2,
                    _ => 3,
                };
                let outcome_score = match (opp, me) {
                    ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                    ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                    _ => 0,
                };
                shape_score + outcome_score
            } else {
                let (opp, outcome) = line.split_once(' ').unwrap();
                let shape_score = match (opp, outcome) {
                    ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
                    ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                    _ => 1,
                };
                let outcome_score = match outcome {
                    "X" => 0,
                    "Y" => 3,
                    _ => 6,
                };
                shape_score + outcome_score
            };

            dbg!(score)
        })
        .reduce(|acc, it| acc + it);
    dbg!(total);

    Ok(())
}
