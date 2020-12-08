use std::collections::HashSet;
use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct ParseError {}

#[derive(Debug)]
struct RunError {}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError {}
    }
}

impl FromStr for OpCode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(OpCode::Nop),
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    pub code: OpCode,
    pub value: i32,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split_ascii_whitespace().collect();
        match parts.len() {
            2 => Ok(Instruction {
                code: parts[0].parse()?,
                value: parts[1].parse()?,
            }),
            _ => Err(ParseError {}),
        }
    }
}

fn run_program_part1(instructions: &[Instruction]) -> i32 {
    let mut visited: HashSet<usize> = HashSet::new();

    let mut current_inst = 0;
    let mut acc = 0;
    loop {
        if visited.contains(&current_inst) {
            break;
        }
        visited.insert(current_inst);
        let inst = &instructions[current_inst];
        match inst.code {
            OpCode::Nop => {
                current_inst += 1;
            }
            OpCode::Acc => {
                acc += inst.value;
                current_inst += 1;
            }
            OpCode::Jmp => {
                current_inst = (current_inst as i32 + inst.value) as usize;
            }
        }
    }
    acc
}

fn run_program_part2(instructions: &[Instruction]) -> Result<i32, RunError> {
    let mut visited: HashSet<usize> = HashSet::new();

    let mut current_inst = 0;
    let mut acc = 0;
    loop {
        if current_inst >= instructions.len() {
            break;
        }
        if visited.contains(&current_inst) {
            Err(RunError {})?;
        }
        visited.insert(current_inst);
        let inst = &instructions[current_inst];
        match inst.code {
            OpCode::Nop => {
                current_inst += 1;
            }
            OpCode::Acc => {
                acc += inst.value;
                current_inst += 1;
            }
            OpCode::Jmp => {
                current_inst = (current_inst as i32 + inst.value) as usize;
            }
        }
    }
    Ok(acc)
}

fn main() {
    let instructions: Vec<Instruction> = INPUT
        .trim()
        .lines()
        .map(|i| i.parse())
        .collect::<Result<_, _>>()
        .expect("invalid instructions");
    println!("Acc: {}", run_program_part1(&instructions));

    let mut first_pos: i32 = -1;
    loop {
        if let Some((idx, inst)) = instructions
            .iter()
            .enumerate()
            .skip(first_pos as usize + 1)
            .find(|(_, i)| i.code == OpCode::Jmp)
        {
            first_pos = idx as i32;
            let mut new_instructions = instructions.clone();
            new_instructions[idx] = Instruction {
                code: OpCode::Nop,
                value: inst.value,
            };
            if let Ok(res) = run_program_part2(&new_instructions) {
                println!("Result: {}", res);
                break;
            }
        } else {
            break;
        }
    }
}
