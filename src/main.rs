use std::io::*;
use std::iter::FromIterator;
use std::{collections::VecDeque, env};
use std::{fs::File, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inbox,
    Outbox,
    CopyFrom(u32),
    CopyTo(u32),
    Add(u32),
    Sub(u32),
    BumpPlus(u32),
    BumpMinus(u32),
    Label(u32),
    Jump(u32),
    JumpIfZero(u32),
    JumpIfNegative(u32),
}

fn jump(instructions: &[Instruction], n: u32) -> usize {
    let labels: Vec<&Instruction> = instructions
        .iter()
        .filter(|x| {
            if **x == Instruction::Label(n) {
                return true;
            }
            false
        })
        .collect();
    let label = **labels.first().expect("could not find the label");
    let label_instruction_address = instructions
        .iter()
        .position(|x| *x == label)
        .expect("could not find label");
    label_instruction_address
}

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

struct Machine {
    instructions: Vec<Instruction>,
    register: Vec<Option<i32>>,
    buffer: Option<i32>,
    enable_logging: bool,
    program_counter: usize,
    instruction_count: i32,
}

impl Machine {
    pub fn new(instructions: Vec<Instruction>, num_registers: usize, enable_logging: bool) -> Self {
        let mut register: Vec<Option<i32>> = std::iter::repeat(None).take(num_registers).collect();
        register[num_registers - 1] = Some(0i32);
        Self {
            instructions,
            register,
            buffer: Some(0i32),
            enable_logging,
            program_counter: 0,
            instruction_count: 0,
        }
    }
    fn reset(&mut self) {
        self.buffer = Some(0i32);
        self.program_counter = 0;
        self.instruction_count = 0;
    }
    pub fn run(&mut self, inbox: &[i32]) -> Vec<i32> {
        self.reset();
        let mut outbox = Vec::new();

        if self.enable_logging {
            println!("program: {:#?}", self.instructions);
            println!("inbox: {:?}", inbox);
        }

        let mut inbox = VecDeque::from_iter(inbox);

        while self.program_counter < self.instructions.len() {
            let ins = self.instructions[self.program_counter];
            self.instruction_count += 1;
            if self.enable_logging {
                println!(
                    "count: {}, instruction: {:?}, counter: {}, register: {:?}, buffer: {:?}",
                    self.instruction_count, ins, self.program_counter, self.register, self.buffer
                );
            }
            match ins {
                Instruction::Inbox => {
                    let next = inbox.pop_front();
                    if next.is_some() {
                        self.buffer = Some(*next.unwrap() as i32);
                    } else {
                        // TODO: panic
                        break;
                    }
                }
                Instruction::Outbox => {
                    outbox.push(self.buffer.expect("can't write None to outbox"));
                    self.buffer = None;
                }
                Instruction::CopyFrom(n) => {
                    let copy = self.register[n as usize];
                    // cant copy from register where nothing is
                    assert!(copy.is_some());
                    self.buffer = copy;
                }
                Instruction::CopyTo(n) => {
                    // cant copy when we have nothing
                    assert!(self.buffer.is_some());
                    self.register[n as usize] = self.buffer;
                }
                Instruction::Add(n) => {
                    self.buffer = Some(
                        self.register[n as usize].expect("cannot add None")
                            + self.buffer.expect("cannot add None"),
                    );
                }
                Instruction::Sub(n) => {
                    self.buffer = Some(
                        self.buffer.expect("cannot add None")
                            - self.register[n as usize].expect("cannot add None"),
                    );
                }
                Instruction::BumpPlus(n) => {
                    self.register[n as usize] =
                        Some(self.register[n as usize].expect("cannot bump+ None") + 1);
                    self.buffer = self.register[n as usize];
                }
                Instruction::BumpMinus(n) => {
                    self.register[n as usize] =
                        Some(self.register[n as usize].expect("cannot bump+ None") - 1);
                    self.buffer = self.register[n as usize];
                }
                Instruction::Label(_) => {}
                Instruction::Jump(n) => {
                    let new_instruction_address = jump(&self.instructions, n);

                    self.program_counter = new_instruction_address;
                    continue;
                }
                Instruction::JumpIfZero(instruction_number) => match self.buffer {
                    Some(n) => {
                        if n == 0 {
                            let new_instruction_address =
                                jump(&self.instructions, instruction_number);
                            self.program_counter = new_instruction_address;

                            continue;
                        }
                    }
                    None => panic!("cannot compare with None"),
                },
                Instruction::JumpIfNegative(instruction_number) => match self.buffer {
                    Some(n) => {
                        if n < 0 {
                            let new_instruction_address =
                                jump(&self.instructions, instruction_number);
                            self.program_counter = new_instruction_address;

                            continue;
                        }
                    }
                    None => panic!("cannot compare with None"),
                },
            }

            self.program_counter += 1;
        }

        outbox
    }
}

pub fn string_to_lines(program: &str) -> Vec<&str> {
    let lines: Vec<&str> = program.split("\n").filter(|x| !x.is_empty()).collect();
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
    let args: Vec<String> = vec!["progpath", "prog.human", "1", "5"]
        .iter()
        .map(|x| String::from_str(*x).unwrap())
        .collect();
        */

    let filename = Path::new(&args[1]); //Path::new("mul.human"); //
    let lines = read_file_to_lines(filename);
    let instructions = get_instructions(lines.iter().map(|x| x.as_str()).collect());

    let enable_logging = args[2].parse::<u32>().expect("cannot parse to number") == 1;

    let inbox = args
        .iter()
        .skip(3)
        .map(|x| x.parse().expect("cannot parse to number"))
        .collect::<Vec<i32>>();
    //let mut inbox = VecDeque::from(vec![5, 1, 2, 3]);

    let mut machine = Machine::new(instructions, 10, enable_logging);
    let outbox = machine.run(&inbox);

    println!(
        "instructions: {}, out: {:?}",
        machine.instruction_count, outbox
    );
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
