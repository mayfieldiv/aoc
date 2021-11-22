use std::io;

fn main() {
    let _solve1 = || -> Option<usize> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let mut state: Vec<usize> = input
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        state[1] = 12;
        state[2] = 2;
        let mut i = 0usize;
        loop {
            match dbg!(&state).get(i) {
                Some(1) => {
                    let (a, b, c) = (state[i + 1], state[i + 2], state[i + 3]);
                    state[c] = state[a] + state[b];
                    i += 3;
                }
                Some(2) => {
                    let (a, b, c) = (state[i + 1], state[i + 2], state[i + 3]);
                    state[c] = state[a] * state[b];
                    i += 3;
                }
                _ => return Some(state[0]),
            }
            i += 1;
        }
    };

    let solve2 = || -> Option<usize> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let initial_state: Vec<usize> = input
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        for noun in 0usize..=99 {
            for verb in 0usize..=99 {
                let mut state = initial_state.clone();
                state[1] = noun;
                state[2] = verb;
                let mut i = 0usize;
                loop {
                    match state.get(i) {
                        Some(1) => {
                            let (a, b, c) = (state[i + 1], state[i + 2], state[i + 3]);
                            state[c] = state[a] + state[b];
                            i += 3;
                        }
                        Some(2) => {
                            let (a, b, c) = (state[i + 1], state[i + 2], state[i + 3]);
                            state[c] = state[a] * state[b];
                            i += 3;
                        }
                        _ => {
                            if state[0] == 19690720 {
                                return Some(100 * noun + verb);
                            } else {
                                break;
                            }
                        }
                    }
                    i += 1;
                }
            }
        }

        None
    };

    // let answer = solve1().unwrap();
    let answer = solve2().unwrap();
    println!("{}", answer);
}
