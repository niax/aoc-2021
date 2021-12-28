use aoc2021::commons::io::load_argv_lines;
use fasthash::{metro::crc::Hash64_1, RandomState};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Register {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Error)]
enum RegisterParseError {
    #[error("Unknown register")]
    UnknownRegister(String),
}

impl FromStr for Register {
    type Err = RegisterParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            "w" => Ok(Self::W),
            _ => Err(RegisterParseError::UnknownRegister(input.to_string())),
        }
    }
}

#[derive(Debug)]
enum Argument {
    Register(Register),
    Immediate(isize),
}

impl Argument {
    pub fn value(&self, registers: &mut RegisterPage) -> isize {
        match self {
            Self::Register(r) => *registers.get(r),
            Self::Immediate(v) => *v,
        }
    }
}

#[derive(Debug, Error)]
enum ArgumentParseError {
    #[error("Unknown register")]
    UnknownRegister(#[from] RegisterParseError),

    #[error("Bad number")]
    NumberParse(#[from] ParseIntError),
}

impl FromStr for Argument {
    type Err = ArgumentParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.chars().next().unwrap() {
            '-' | '0'..='9' => Ok(Self::Immediate(input.parse()?)),
            _ => Ok(Self::Register(input.parse()?)),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Input(Register),
    Add(Register, Argument),
    Mul(Register, Argument),
    Div(Register, Argument),
    Mod(Register, Argument),
    Eql(Register, Argument),
}

impl Instruction {
    pub fn is_input(&self) -> bool {
        matches!(self, Self::Input(_))
    }

    pub fn execute(&self, registers: &mut RegisterPage, input: isize) {
        match self {
            Self::Input(r) => *registers.get(r) = input,
            Self::Add(r, a) => *registers.get(r) += a.value(registers),
            Self::Mul(r, a) => *registers.get(r) *= a.value(registers),
            Self::Div(r, a) => *registers.get(r) /= a.value(registers),
            Self::Mod(r, a) => *registers.get(r) %= a.value(registers),
            Self::Eql(r, a) => {
                let c = a.value(registers);
                let b = registers.get(r);
                *b = if *b == c { 1 } else { 0 }
            }
        }
    }
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("Bad instruction")]
    UnknownInstruction,

    #[error("Bad argument")]
    BadArgument(#[from] ArgumentParseError),

    #[error("Bad register")]
    BadRegister(#[from] RegisterParseError),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_string();
        let parts = input.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "inp" => Ok(Self::Input(parts[1].parse()?)),
            "add" => Ok(Self::Add(parts[1].parse()?, parts[2].parse()?)),
            "mul" => Ok(Self::Mul(parts[1].parse()?, parts[2].parse()?)),
            "div" => Ok(Self::Div(parts[1].parse()?, parts[2].parse()?)),
            "mod" => Ok(Self::Mod(parts[1].parse()?, parts[2].parse()?)),
            "eql" => Ok(Self::Eql(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(InstructionParseError::UnknownInstruction),
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct RegisterPage {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl RegisterPage {
    fn get(&mut self, r: &Register) -> &mut isize {
        match r {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
            Register::W => &mut self.w,
        }
    }
}

fn main() {
    let ins = load_argv_lines::<Instruction>()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut sub_progs = Vec::with_capacity(14);
    let mut curr = Vec::with_capacity(20);
    for i in ins {
        if i.is_input() && !curr.is_empty() {
            sub_progs.push(curr);
            curr = Vec::with_capacity(20);
        }
        curr.push(i);
    }
    sub_progs.push(curr);

    let h = RandomState::<Hash64_1>::new();
    let mut in_regs = HashMap::with_capacity_and_hasher(1, h);
    in_regs.insert(0, (0, 0));
    for prog in &sub_progs {
        let h = RandomState::<Hash64_1>::new();
        let mut next_regs = HashMap::with_capacity_and_hasher(in_regs.len(), h);
        for v in 1..=9 {
            for (z, n) in &in_regs {
                let mut regs = RegisterPage::default();
                regs.z = *z;
                for i in prog {
                    i.execute(&mut regs, v);
                }

                if regs.z > 350_000 {
                    // Abort on Z values that are growing out of control
                    continue;
                }
                let min_max = next_regs.entry(regs.z).or_insert((isize::MAX, isize::MIN));
                min_max.0 = min_max.0.min(n.0 * 10 + v);
                min_max.1 = min_max.1.max(n.1 * 10 + v);
            }
        }
        in_regs = next_regs;
    }

    let (min, max) = in_regs.get(&0).unwrap();
    println!("{}", max);
    println!("{}", min);
}
