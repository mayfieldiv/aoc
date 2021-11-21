use std::io;

fn main() {
    let mut total = 0i32;

    let mut _solve1 = || -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let mass: i32 = input.trim().parse()?;
            total += mass / 3 - 2;
        }
    };

    let mut solve2 = || -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let mut mass: i32 = dbg!(input.trim()).parse()?;
            loop {
                mass = mass / 3 - 2;
                if dbg!(mass) > 0 {
                    total += mass;
                } else {
                    break;
                }
            }
        }
    };

    // let _ = solve1();
    let _ = solve2();
    println!("{}", total);
}
