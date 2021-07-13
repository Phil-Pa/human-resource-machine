use std::io::*;
use std::env;
use std::{fs::File, path::Path};

mod machine;

use machine::{Instruction, Machine};

fn read_file_to_lines(filename: &Path) -> Vec<String> {
    let file = File::open(filename).expect("cannot find file");
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let lines = lines.map(|x| x.expect("cant read file to lines")).collect();
    lines
}

fn get_instructions(lines: Vec<&str>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for mut line in lines {
        line = line.trim();
        // TODO
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        if line.contains('#') {
            // BumpPlus 2 # this is a comment
            line = &line[0..line.find('#').expect("could not find # in line")];
        }
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        match parts[0] {
            "inbox" => instructions.push(Instruction::Inbox),
            "outbox" => {
                instructions.push(Instruction::Outbox);
            }
            "copyfrom" => instructions.push(Instruction::CopyFrom(
                parts[1].parse().expect("cannot parse to number"),
            )),
            "copyto" => {
                instructions.push(Instruction::CopyTo(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "add" => {
                instructions.push(Instruction::Add(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "sub" => {
                 instructions.push(Instruction::Sub(
                    parts[1].parse().expect("cannot parse to number"),
                ));
           }
            "mul" => {
                instructions.push(Instruction::Mul(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "bump+" => {
                instructions.push(Instruction::BumpPlus(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "bump-" => {
                instructions.push(Instruction::BumpMinus(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "label" => {
                instructions.push(Instruction::Label(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "jump" => {
                instructions.push(Instruction::Jump(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "jumpzero" => {
                instructions.push(Instruction::JumpIfZero(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            "jumpnegative" => {
                instructions.push(Instruction::JumpIfNegative(
                    parts[1].parse().expect("cannot parse to number"),
                ));
            }
            _ => {
                panic!("{}: instruction not found", line);
            }
        }
    }

    instructions
}

pub fn string_to_lines(program: &str) -> Vec<&str> {
    let lines: Vec<&str> = program.split("\n").filter(|x| !x.is_empty()).collect();
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = Path::new(&args[1]); //Path::new("mul.human"); //
    let lines = read_file_to_lines(filename);
    let instructions = get_instructions(lines.iter().map(|x| x.as_str()).collect());

    let enable_logging = args[2].parse::<u32>().expect("cannot parse to number") == 1;

    let inbox = args
        .iter()
        .skip(3)
        .map(|x| x.parse().expect("cannot parse to number"))
        .collect::<Vec<i32>>();

    let mut machine = Machine::new(instructions, 10, enable_logging);
    let outbox = machine.run(&inbox);

    println!(
        "instructions: {}",
        machine.get_instruction_count() as usize + outbox.len()
    );

    if enable_logging {
        println!("{:?}", outbox);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum() {
        // calculates 1 + 2 + 3 + 4 + 5 + ... + n where n comes from inbox
        let program = r"
copyfrom 9
copyto 1
copyto 2
bump+ 1
inbox
copyto 2
label 1
sub 1
copyto 3
add 2
copyto 2
copyfrom 3
jumpzero 2
jump 1
label 2
copyfrom 2
outbox";

        let lines = string_to_lines(program);
        let mut machine = Machine::new(get_instructions(lines), 10, false);
        let outbox = machine.run(&[5]);
        assert_eq!(1, outbox.len());
        assert_eq!(15, outbox[0]);
    }
}
