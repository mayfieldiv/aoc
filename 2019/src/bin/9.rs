#![allow(dead_code)]
use std::collections::VecDeque;

fn main() {
    println!("{}", solve(2));
}

fn solve(mode: i64) -> i64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let program: Vec<i64> = line.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut input = VecDeque::new();
    input.push_back(mode);
    let mut output = VecDeque::new();
    let mut memory = program.clone();
    memory.resize(10_000, 0);
    match run(
        &mut State {
            memory,
            ip: 0,
            relative_base: 0,
        },
        &mut input,
        &mut output,
    ) {
        ExitCode::Halted | ExitCode::Completed => return *dbg!(output).back().unwrap(),
        _ => unreachable!(),
    }
}

fn run(state: &mut State, input: &mut VecDeque<i64>, output: &mut VecDeque<i64>) -> ExitCode {
    loop {
        eprintln!("=====================================================");
        match state.memory.get(dbg!(state.ip)) {
            Some(inst) => {
                let op: OpCode = (dbg!(inst) % 100).into();
                let mut modes = inst / 100;
                state.ip += 1;

                let mut read_param = || -> Param {
                    let value = state.memory[state.ip];
                    state.ip += 1;
                    let mode: ParamMode = (modes % 10).into();
                    dbg!(&mode, value);
                    let param = match mode {
                        ParamMode::Position => {
                            let loc: usize = value.try_into().unwrap();
                            eprintln!("state.memory[{}] -> {}", value, state.memory[loc]);
                            Param::Position(loc)
                        }
                        ParamMode::Immediate => Param::Value(value as i64),
                        ParamMode::Relative => {
                            let loc: usize = (state.relative_base + value).try_into().unwrap();
                            eprintln!(
                                "state.memory[{} + {} = {}] -> {}",
                                state.relative_base, value, loc, state.memory[loc]
                            );
                            Param::Position(loc)
                        }
                    };
                    modes /= 10;
                    dbg!(param)
                };
                let get_value = |param: Param| -> i64 {
                    match param {
                        Param::Position(p) => state.memory[p],
                        Param::Value(v) => v,
                    }
                };
                let get_loc = |param: Param| -> usize {
                    match param {
                        Param::Position(p) => p.try_into().unwrap(),
                        Param::Value(_) => unreachable!(),
                    }
                };

                match dbg!(op) {
                    OpCode::Add => {
                        let a = get_value(read_param());
                        let b = get_value(read_param());
                        let c = get_loc(read_param());
                        eprintln!("state.memory[{}] = {} + {} = {}", c, a, b, a + b);
                        state.memory[c] = a + b;
                    }
                    OpCode::Mul => {
                        let a = get_value(read_param());
                        let b = get_value(read_param());
                        let c = get_loc(read_param());
                        eprintln!("state.memory[{}] = {} * {} = {}", c, a, b, a * b);
                        state.memory[c] = a * b;
                    }
                    OpCode::Input => {
                        let p = get_loc(read_param());
                        if let Some(i) = input.pop_front() {
                            eprintln!("state.memory[{}] = {}", p, i);
                            state.memory[p] = i;
                        } else {
                            state.ip -= 2;
                            return ExitCode::Blocked;
                        }
                    }
                    OpCode::Output => {
                        let o = get_value(read_param());
                        eprintln!("output <- {}", o);
                        output.push_back(o);
                    }
                    OpCode::JumpIfTrue => {
                        let a = get_value(read_param());
                        let b = get_value(read_param());
                        if a != 0 {
                            eprintln!("{} != 0 -> jump to {}", a, b);
                            state.ip = b.try_into().unwrap();
                        } else {
                            eprintln!("{} == 0 -> don't jump to {}", a, b);
                        }
                    }
                    OpCode::JumpIfFalse => {
                        let a = get_value(read_param());
                        let b = get_value(read_param());
                        if a == 0 {
                            eprintln!("{} == 0 -> jump to {}", a, b);
                            state.ip = b.try_into().unwrap();
                        } else {
                            eprintln!("{} != 0 -> don't jump to {}", a, b);
                        }
                    }
                    OpCode::LessThan => {
                        let a = get_value(read_param());
                        let b = get_value(read_param());
                        let c = get_loc(read_param());
                        state.memory[c] = if a < b {
                            eprintln!("{} < {} -> state.memory[{}] = {}", a, b, c, 1);
                            1
                        } else {
                            eprintln!("{} >= {} -> state.memory[{}] = {}", a, b, c, 0);
                            0
                        };
                    }
                    OpCode::Equals => {
                        let a = get_value(read_param());
                        let b = get_value(read_param());
                        let c = get_loc(read_param());
                        state.memory[c] = if a == b {
                            eprintln!("{} == {} -> state.memory[{}] = {}", a, b, c, 1);
                            1
                        } else {
                            eprintln!("{} != {} -> state.memory[{}] = {}", a, b, c, 0);
                            0
                        };
                    }
                    OpCode::AdjustRelativeBase => {
                        let d = get_value(read_param());
                        eprintln!(
                            "relative base: {} + {} = {}",
                            state.relative_base,
                            d,
                            state.relative_base + d
                        );
                        state.relative_base += d;
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
    memory: Vec<i64>,
    ip: usize,
    relative_base: i64,
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
    Relative = 2,
}

impl From<i64> for ParamMode {
    fn from(v: i64) -> Self {
        match v {
            x if x == ParamMode::Position as i64 => ParamMode::Position,
            x if x == ParamMode::Immediate as i64 => ParamMode::Immediate,
            x if x == ParamMode::Relative as i64 => ParamMode::Relative,
            x => unimplemented!("param mode {}", x),
        }
    }
}

#[derive(Debug)]
enum Param {
    Position(usize),
    Value(i64),
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
    AdjustRelativeBase = 9,
    Halt = 99,
}

impl From<i64> for OpCode {
    fn from(v: i64) -> Self {
        match v {
            x if x == OpCode::Add as i64 => OpCode::Add,
            x if x == OpCode::Mul as i64 => OpCode::Mul,
            x if x == OpCode::Input as i64 => OpCode::Input,
            x if x == OpCode::Output as i64 => OpCode::Output,
            x if x == OpCode::JumpIfTrue as i64 => OpCode::JumpIfTrue,
            x if x == OpCode::JumpIfFalse as i64 => OpCode::JumpIfFalse,
            x if x == OpCode::LessThan as i64 => OpCode::LessThan,
            x if x == OpCode::Equals as i64 => OpCode::Equals,
            x if x == OpCode::AdjustRelativeBase as i64 => OpCode::AdjustRelativeBase,
            x if x == OpCode::Halt as i64 => OpCode::Halt,
            x => unimplemented!("op code {}", x),
        }
    }
}
