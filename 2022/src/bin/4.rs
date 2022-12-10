use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut result = 0;
    for line in input.trim().split('\n') {
        let (a, b) = line.split_once(',').unwrap();
        let (a_start, a_end) = a.split_once('-').unwrap();
        let (b_start, b_end) = b.split_once('-').unwrap();
        let (a_start, a_end, b_start, b_end): (u32, u32, u32, u32) = (
            a_start.parse().unwrap(),
            a_end.parse().unwrap(),
            b_start.parse().unwrap(),
            b_end.parse().unwrap(),
        );
        let a = a_start..=a_end;
        let b = b_start..=b_end;
        if cfg!(part_1) {
            if (a.contains(&b_start) && a.contains(&b_end))
                || (b.contains(&a_start) && b.contains(&a_end))
            {
                dbg!((a_start, a_end, b_start, b_end));
                result += 1;
            }
        } else {
            if a.contains(&b_start)
                || a.contains(&b_end)
                || b.contains(&a_start)
                || b.contains(&a_end)
            {
                dbg!((a_start, a_end, b_start, b_end));
                result += 1;
            }
        }
    }

    dbg!(result);
    Ok(())
}
