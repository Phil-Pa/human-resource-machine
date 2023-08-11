use std::{num::ParseIntError, io::{BufReader, BufRead}, fs::File};

use crate::machine::{Machine, Instruction};

#[derive(Debug)]
pub enum InstructionParseError {
    InstructionNotFound,
    ParseIntError(ParseIntError),
    InvalidSyntax
}

impl From<ParseIntError> for InstructionParseError {
    fn from(err: ParseIntError) -> Self {
        InstructionParseError::ParseIntError(err)
    }
}

pub struct InstructionParser {
    lines: Vec<String>,
}

impl InstructionParser {

    pub fn new_from_file(filename: &str) -> std::io::Result<Self> {
        let lines = BufReader::new(File::open(filename)?).lines().collect::<std::io::Result<Vec<String>>>()?;

        Ok(InstructionParser {
            lines
        })
    }

    pub fn parse(&self) -> std::result::Result<Machine, InstructionParseError> {
        let mut instructions = Vec::new();

        for line in &self.lines {
            let line = line.as_str().trim();
    
            let should_ignore_line = line.starts_with("//") || line.is_empty();
            if should_ignore_line {
                continue;
            }

            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            if parts.len() < 2 {
                return Err(InstructionParseError::InvalidSyntax);
            }

            let number = parts[1].parse()?;
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
                _ => return Err(InstructionParseError::InstructionNotFound),
            }
        }

        Ok(Machine::new(instructions, 10, false))
    }
}