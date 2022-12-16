use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let num_unique = if cfg!(part_1) { 4 } else { 14 };

    for i in num_unique..=input.trim_end().len() {
        let substring = &input[i - num_unique..i];
        let set = substring.chars().fold(HashSet::new(), |mut set, it| {
            set.insert(it);
            set
        });
        if set.len() == num_unique {
            dbg!(i, substring);
            break;
        }
    }

    Ok(())
}
