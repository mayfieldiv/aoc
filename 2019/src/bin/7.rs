#![allow(dead_code)]
use std::cell::Cell;
use std::collections::VecDeque;

fn main() {
    dbg!(solve2());
}

fn solve1() -> i32 {
    solve((0..5 as i32).collect())
}

fn solve2() -> i32 {
    solve((5..=9 as i32).collect())
}

fn solve(phases: Vec<i32>) -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let program: Vec<i32> = line.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut best = (0, phases.clone());
    for phases in permute(phases) {
        let mut states = vec![
            State {
                memory: program.clone(),
                ip: Cell::new(0),
            };
            phases.len()
        ];
        let mut inputs = vec![VecDeque::default(); phases.len()];
        for i in 0..phases.len() {
            inputs[i].push_back(phases[i]);
        }
        inputs[0].push_back(0);

        let mut simulate = || -> i32 {
            loop {
                for i in 0..states.len() {
                    let state = states.get_mut(i).unwrap();
                    let (input, output) = if i + 1 < inputs.len() {
                        let (left, right) = inputs.split_at_mut(i + 1);
                        (&mut left[i], &mut right[0])
                    } else {
                        let (left, right) = inputs.split_at_mut(i);
                        (&mut right[0], &mut left[0])
                    };
                    dbg!(i, &input, &output);
                    match run(state, input, output) {
                        ExitCode::Halted | ExitCode::Completed if i == states.len() - 1 => {
                            return *output.back().unwrap()
                        }
                        _ => {}
                    }
                }
            }
        };
        let output = simulate();
        if output > best.0 {
            best = (output, phases);
        }
    }

    return dbg!(best).0;
}

fn permute(list: Vec<i32>) -> Vec<Vec<i32>> {
    if list.len() == 1 {
        return vec![vec![list[0]]];
    }
    let mut permutations = vec![];
    for i in 0..list.len() {
        let mut others = list[..i].to_vec();
        others.extend(list[i + 1..].iter());
        for mut p in permute(others) {
            p.push(list[i]);
            permutations.push(p);
        }
    }
    permutations
}

fn run(state: &mut State, input: &mut VecDeque<i32>, output: &mut VecDeque<i32>) -> ExitCode {
    loop {
        match state.memory.get(state.ip.get()) {
            Some(inst) => {
                let op: OpCode = (inst % 100).into();
                let modes = Cell::new(inst / 100);
                state.ip.set(state.ip.get() + 1);
                let read_param = || -> i32 {
                    let value = dbg!(state.memory[state.ip.get()]);
                    state.ip.set(state.ip.get() + 1);
                    let mode: ParamMode = (modes.get() % 10).into();
                    let param = match dbg!(mode) {
                        ParamMode::Position => state.memory[usize::try_from(value).unwrap()],
                        ParamMode::Immediate => value as i32,
                    };
                    modes.set(modes.get() / 10);
                    dbg!(param)
                };
                let read_loc_param = || -> usize {
                    let loc_param = usize::try_from(state.memory[state.ip.get()]).unwrap();
                    state.ip.set(state.ip.get() + 1);
                    modes.set(modes.get() / 10);
                    dbg!(loc_param)
                };
                match dbg!(op) {
                    OpCode::Add => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        dbg!(a, b, c);
                        state.memory[c] = a + b;
                    }
                    OpCode::Mul => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        dbg!(a, b, c);
                        state.memory[c] = a * b;
                    }
                    OpCode::Input => {
                        let p = read_loc_param();
                        if let Some(i) = input.pop_front() {
                            state.memory[p] = i;
                        } else {
                            state.ip.set(state.ip.get() - 2);
                            return ExitCode::Blocked;
                        }
                    }
                    OpCode::Output => {
                        let o = read_param();
                        output.push_back(dbg!(o));
                    }
                    OpCode::JumpIfTrue => {
                        let a = read_param();
                        let b = read_param();
                        if a != 0 {
                            state.ip.set(usize::try_from(b).unwrap());
                        }
                    }
                    OpCode::JumpIfFalse => {
                        let a = read_param();
                        let b = read_param();
                        if a == 0 {
                            state.ip.set(usize::try_from(b).unwrap());
                        }
                    }
                    OpCode::LessThan => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        state.memory[c] = if a < b { 1 } else { 0 };
                    }
                    OpCode::Equals => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        state.memory[c] = if a == b { 1 } else { 0 };
                    }
                    OpCode::Halt => return ExitCode::Halted,
                }
            }
            None => return ExitCode::Completed,
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    memory: Vec<i32>,
    ip: Cell<usize>,
}

#[derive(Debug)]
enum ExitCode {
    Blocked,
    Halted,
    Completed,
}

#[derive(Debug)]
enum ParamMode {
    Position = 0,
    Immediate = 1,
}

impl From<i32> for ParamMode {
    fn from(v: i32) -> Self {
        match v {
            x if x == ParamMode::Position as i32 => ParamMode::Position,
            x if x == ParamMode::Immediate as i32 => ParamMode::Immediate,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl From<i32> for OpCode {
    fn from(v: i32) -> Self {
        match v {
            x if x == OpCode::Add as i32 => OpCode::Add,
            x if x == OpCode::Mul as i32 => OpCode::Mul,
            x if x == OpCode::Input as i32 => OpCode::Input,
            x if x == OpCode::Output as i32 => OpCode::Output,
            x if x == OpCode::JumpIfTrue as i32 => OpCode::JumpIfTrue,
            x if x == OpCode::JumpIfFalse as i32 => OpCode::JumpIfFalse,
            x if x == OpCode::LessThan as i32 => OpCode::LessThan,
            x if x == OpCode::Equals as i32 => OpCode::Equals,
            x if x == OpCode::Halt as i32 => OpCode::Halt,
            _ => unreachable!(),
        }
    }
}
