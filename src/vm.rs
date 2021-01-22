use std::{
    fmt,
    fmt::{Display, Formatter},
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(r"(?P<opcode>\w{3}) (?P<arg>[+-]\d+)").unwrap();
}

pub trait RunnableMachine {
    fn run(&mut self, steps: usize) -> Result<(), errors::VirtualMachineError>;
}

#[derive(Debug)]
pub struct VirtualMachine {
    program: Program,
    iptr: usize,
    acc: isize,
}

impl VirtualMachine {
    pub fn from(program: Program) -> Self {
        Self {
            program,
            iptr: 0,
            acc: 0,
        }
    }

    pub fn acc(&self) -> isize {
        self.acc
    }

    pub fn iptr(&self) -> usize {
        self.iptr
    }
}

impl RunnableMachine for VirtualMachine {
    fn run(&mut self, steps: usize) -> Result<(), errors::VirtualMachineError> {
        for _ in 0..steps {
            match self.program.get_instruction(self.iptr) {
                Some(Instruction::Acc(n)) => {
                    self.acc += n;
                    self.iptr += 1;
                }
                Some(Instruction::Jmp(n)) => {
                    let new_iptr = self.iptr as isize + n;
                    if new_iptr < 0 {
                        return Err(errors::VirtualMachineError::InstructionPtrOutOfRange(
                            new_iptr,
                        ));
                    } else {
                        self.iptr = new_iptr as usize
                    }
                }
                Some(Instruction::Nop(_)) => self.iptr += 1,
                None => {
                    return Err(errors::VirtualMachineError::InstructionPtrOutOfRange(
                        self.iptr as isize,
                    ))
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn from(input: &str) -> Result<Self, errors::VirtualMachineError> {
        let instructions = input
            .split_terminator('\n')
            .map(Instruction::from)
            .collect::<Result<Vec<Instruction>, errors::InstructionParseError>>()?;
        Ok(Program { instructions })
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn replace_instruction(
        &mut self,
        index: usize,
        instruction: Instruction,
    ) -> Result<(), ()> {
        match self.instructions.get_mut(index) {
            Some(i) => {
                *i = instruction;
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(isize),
}

impl Instruction {
    pub fn from(line: &str) -> Result<Self, errors::InstructionParseError> {
        match INSTRUCTION_RE.captures(line) {
            Some(captures) => {
                let opcode_match: Option<&str> = captures.name("opcode").map(|c| c.as_str());
                let arg_match: Option<isize> = captures
                    .name("arg")
                    .map(|c| c.as_str())
                    .and_then(|s| s.parse().ok());
                match (opcode_match, arg_match) {
                    (None, _) => Err(errors::InstructionParseError::NoCaptures),
                    (_, None) => Err(errors::InstructionParseError::InvalidArg),
                    (Some("nop"), Some(arg)) => Ok(Self::Nop(arg)),
                    (Some("acc"), Some(arg)) => Ok(Self::Acc(arg)),
                    (Some("jmp"), Some(arg)) => Ok(Self::Jmp(arg)),
                    (Some(_), _) => Err(errors::InstructionParseError::InvalidOpcode),
                }
            }
            None => Err(errors::InstructionParseError::NoCaptures),
        }
    }
}

pub mod errors {
    use super::*;

    #[derive(Debug)]
    pub enum VirtualMachineError {
        ParseError(InstructionParseError),
        InstructionPtrOutOfRange(isize),
    }

    impl Display for VirtualMachineError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            write!(formatter, "{:?}", self)
        }
    }

    impl From<InstructionParseError> for VirtualMachineError {
        fn from(e: InstructionParseError) -> VirtualMachineError {
            VirtualMachineError::ParseError(e)
        }
    }

    #[derive(Debug)]
    pub enum InstructionParseError {
        NoCaptures,
        InvalidArg,
        InvalidOpcode,
    }

    impl Display for InstructionParseError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            write!(formatter, "{:?}", self)
        }
    }
}
