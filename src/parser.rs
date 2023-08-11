use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

use crate::machine::{Instruction, Machine};

#[derive(Debug)]
pub enum InstructionParseError {
    InstructionNotFound {
        line_number: u32,
    },
    ParseIntError {
        err: ParseIntError,
        line_number: u32,
    },
    InvalidSyntax {
        line_number: u32,
    },
}

pub struct InstructionParser {
    lines: Vec<String>,
}

impl InstructionParser {
    pub fn new_from_file(filename: &str) -> std::io::Result<Self> {
        let lines = BufReader::new(File::open(filename)?)
            .lines()
            .collect::<std::io::Result<Vec<String>>>()?;

        Ok(Self { lines })
    }

    pub fn new_from_str(program: &str) -> Self {
        let lines = program
            .lines()
            .map(|line| String::from(line))
            .collect::<Vec<String>>();
        Self { lines }
    }

    pub fn parse(&self) -> std::result::Result<Machine, InstructionParseError> {
        let mut instructions = Vec::new();

        let mut line_number = 1;

        for line in &self.lines {
            let line = line.as_str().trim();

            let should_ignore_line = line.starts_with("//") || line.is_empty();
            if should_ignore_line {
                continue;
            }

            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let num_parts = parts.len();

            let mut number = 0;

            if num_parts == 0 {
                line_number += 1;
                continue;
            }

            if num_parts > 2 {
                return Err(InstructionParseError::InvalidSyntax { line_number });
            }

            if num_parts == 2 {
                number = parts[1]
                    .parse()
                    .map_err(|err| InstructionParseError::ParseIntError { err, line_number })?;

            }

            let instruction = parts[0];

            match instruction {
                "inbox" => instructions.push(Instruction::Inbox),
                "outbox" => instructions.push(Instruction::Outbox),
                "copyfrom" => instructions.push(Instruction::CopyFrom(number)),
                "copyto" => instructions.push(Instruction::CopyTo(number)),
                "add" => instructions.push(Instruction::Add(number)),
                "sub" => instructions.push(Instruction::Sub(number)),
                "mul" => instructions.push(Instruction::Mul(number)),
                "bump+" => instructions.push(Instruction::BumpPlus(number)),
                "bump-" => instructions.push(Instruction::BumpMinus(number)),
                "label" => instructions.push(Instruction::Label(number)),
                "jump" => instructions.push(Instruction::Jump(number)),
                "jumpzero" => instructions.push(Instruction::JumpIfZero(number)),
                "jumpnegative" => instructions.push(Instruction::JumpIfNegative(number)),
                _ => return Err(InstructionParseError::InstructionNotFound { line_number }),
            }

            line_number += 1;
        }

        Ok(Machine::new(instructions, 10, false))
    }
}
