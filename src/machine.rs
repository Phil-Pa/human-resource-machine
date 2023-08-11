use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Inbox,
    Outbox,
    CopyFrom(u32),
    CopyTo(u32),
    Add(u32),
    Sub(u32),
    Mul(u32),
    BumpPlus(u32),
    BumpMinus(u32),
    Label(u32),
    Jump(u32),
    JumpIfZero(u32),
    JumpIfNegative(u32),
}

#[derive(Debug)]
pub enum MachineRuntimeError {
    // RegisterOutOfBounds,
    EmptyInbox,
    EmptyBuffer,
    EmptyRegister,
    InvalidJumpAddress,
}

pub struct Machine {
    instructions: Vec<Instruction>,
    register: Vec<Option<i32>>,
    buffer: Option<i32>,
    pub enable_logging: bool,
    program_counter: usize,
    instruction_count: i32,
    num_to_label_map: HashMap<u32, usize>,
}

impl Machine {
    fn create_label_num_to_address_map(instructions: &[Instruction]) -> HashMap<u32, usize> {
        let mut map = HashMap::new();

        for ins in instructions.iter() {
            match ins {
                Instruction::Label(n) => {
                    map.insert(*n, *ins);
                }
                _ => {}
            }
        }

        let mut num_to_label_map = HashMap::new();

        for (label_num, label) in map {
            let address = instructions.iter().position(|x| *x == label).unwrap();
            num_to_label_map.insert(label_num, address);
        }
        num_to_label_map
    }
    pub fn get_instruction_count(&self) -> i32 {
        self.instruction_count
    }
    pub fn new(instructions: Vec<Instruction>, num_registers: usize, enable_logging: bool) -> Self {
        let mut register = vec![None; num_registers];
        register[num_registers - 1] = Some(0);

        let num_to_label_map = Machine::create_label_num_to_address_map(&instructions);

        Self {
            instructions,
            register,
            buffer: Some(0),
            enable_logging,
            program_counter: 0,
            instruction_count: 0,
            num_to_label_map,
        }
    }
    fn jump(&mut self, n: u32) -> std::result::Result<usize, MachineRuntimeError> {
        Ok(*self
            .num_to_label_map
            .get(&n)
            .ok_or(MachineRuntimeError::InvalidJumpAddress)?)
    }
    fn reset(&mut self) {
        self.buffer = Some(0);
        self.program_counter = 0;
        self.instruction_count = 0;
    }
    pub fn run(&mut self, inbox: &[i32]) -> std::result::Result<Vec<i32>, MachineRuntimeError> {
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
                    self.buffer = Some(*inbox.pop_front().ok_or(MachineRuntimeError::EmptyInbox)?)
                }
                Instruction::Outbox => {
                    outbox.push(self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?);
                    self.buffer = None;
                }
                Instruction::CopyFrom(n) => {
                    self.buffer =
                        Some(self.register[n as usize].ok_or(MachineRuntimeError::EmptyRegister)?)
                }
                Instruction::CopyTo(n) => {
                    self.register[n as usize] =
                        Some(self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?)
                }
                Instruction::Add(n) => {
                    self.buffer = Some(
                        self.register[n as usize].ok_or(MachineRuntimeError::EmptyRegister)?
                            + self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?,
                    )
                }
                Instruction::Sub(n) => {
                    self.buffer = Some(
                        self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?
                            - self.register[n as usize]
                                .ok_or(MachineRuntimeError::EmptyRegister)?,
                    )
                }
                Instruction::Mul(n) => {
                    self.buffer = Some(
                        self.register[n as usize].ok_or(MachineRuntimeError::EmptyRegister)?
                            * self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?,
                    )
                }
                Instruction::BumpPlus(n) => {
                    self.register[n as usize] = Some(
                        self.register[n as usize].ok_or(MachineRuntimeError::EmptyRegister)? + 1,
                    );
                    self.buffer = self.register[n as usize];
                }
                Instruction::BumpMinus(n) => {
                    self.register[n as usize] = Some(
                        self.register[n as usize].ok_or(MachineRuntimeError::EmptyRegister)? - 1,
                    );
                    self.buffer = self.register[n as usize];
                }
                Instruction::Label(_) => {}
                Instruction::Jump(n) => {
                    let new_instruction_address = self.jump(n)?;
                    self.program_counter = new_instruction_address;
                    continue;
                }
                Instruction::JumpIfZero(instruction_number) => {
                    let buffer = self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?;
                    if buffer == 0 {
                        let new_instruction_address = self.jump(instruction_number)?;
                        self.program_counter = new_instruction_address;
                        continue;
                    }
                }
                Instruction::JumpIfNegative(instruction_number) => {
                    let buffer = self.buffer.ok_or(MachineRuntimeError::EmptyBuffer)?;
                    if buffer < 0 {
                        let new_instruction_address = self.jump(instruction_number)?;
                        self.program_counter = new_instruction_address;
                        continue;
                    }
                }
            }

            self.program_counter += 1;
        }

        Ok(outbox)
    }
}
