#![allow(dead_code)]

use std::cell::Cell;

fn main() {
    // println!("{}", solve1().unwrap());
    println!("{}", solve2());
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

fn solve1() -> Option<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok()?;
    let mut state: Vec<i32> = line.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut ip = 0usize;
    loop {
        match dbg!(&state).get(ip) {
            Some(inst) => {
                let op = inst % 100;
                let mut modes = inst / 100;
                ip += 1;
                let mut read_param = || -> Option<i32> {
                    let value = dbg!(state[ip]);
                    ip += 1;
                    let result = match dbg!(modes % 10) {
                        m if m == ParamMode::Position as i32 => {
                            state[dbg!(usize::try_from(value).ok()?)]
                        }
                        m if m == ParamMode::Immediate as i32 => value as i32,
                        _ => unreachable!(),
                    };
                    modes /= 10;
                    Some(dbg!(result))
                };
                let read_loc_param = |ip: &mut usize, modes: &mut i32| -> Option<usize> {
                    let value = usize::try_from(state[*ip]).ok()?;
                    *ip += 1;
                    *modes /= 10;
                    Some(dbg!(value))
                };
                match dbg!(op) {
                    1 => {
                        let a = read_param()?;
                        let b = read_param()?;
                        let c = read_loc_param(&mut ip, &mut modes)?;
                        state[c] = a + b;
                    }
                    2 => {
                        let a = read_param()?;
                        let b = read_param()?;
                        let c = read_loc_param(&mut ip, &mut modes)?;
                        state[c] = a * b;
                    }
                    3 => {
                        let p = read_loc_param(&mut ip, &mut modes)?;
                        state[p] = 1;
                    }
                    4 => {
                        let p = read_param()?;
                        if p != 0 {
                            return Some(p);
                        }
                    }
                    99 => return Some(state[0]),
                    _ => unreachable!(),
                }
            }
            None => return Some(state[0]),
        }
    }
}

fn solve2() -> i32 {
    const INPUT: i32 = 5;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut state: Vec<i32> = line.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let ip = Cell::new(0usize);
    loop {
        match dbg!(&state).get(ip.get()) {
            Some(inst) => {
                let op: OpCode = (inst % 100).into();
                let modes = Cell::new(inst / 100);
                ip.set(ip.get() + 1);
                let read_param = || -> i32 {
                    let value = dbg!(state[ip.get()]);
                    ip.set(ip.get() + 1);
                    let mode: ParamMode = (modes.get() % 10).into();
                    let result = match dbg!(mode) {
                        ParamMode::Position => state[usize::try_from(value).unwrap()],
                        ParamMode::Immediate => value as i32,
                    };
                    modes.set(modes.get() / 10);
                    dbg!(result)
                };
                let read_loc_param = || -> usize {
                    let value = usize::try_from(state[ip.get()]).unwrap();
                    ip.set(ip.get() + 1);
                    modes.set(modes.get() / 10);
                    dbg!(value)
                };
                match dbg!(op) {
                    OpCode::Add => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        state[c] = a + b;
                    }
                    OpCode::Mul => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        state[c] = a * b;
                    }
                    OpCode::Input => {
                        let p = read_loc_param();
                        state[p] = dbg!(INPUT);
                    }
                    OpCode::Output => {
                        let output = read_param();
                        return dbg!(output);
                    }
                    OpCode::JumpIfTrue => {
                        let a = read_param();
                        let b = read_param();
                        if a != 0 {
                            ip.set(usize::try_from(b).unwrap());
                        }
                    }
                    OpCode::JumpIfFalse => {
                        let a = read_param();
                        let b = read_param();
                        if a == 0 {
                            ip.set(usize::try_from(b).unwrap());
                        }
                    }
                    OpCode::LessThan => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        state[c] = if a < b { 1 } else { 0 };
                    }
                    OpCode::Equals => {
                        let a = read_param();
                        let b = read_param();
                        let c = read_loc_param();
                        state[c] = if a == b { 1 } else { 0 };
                    }
                    OpCode::Halt => return state[0],
                }
            }
            None => return state[0],
        }
    }
}
