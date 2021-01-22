use std::{fmt, fmt::{Display, Formatter}};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(r"(?P<opcode>\w{3}) (?P<arg>[+-]\d+)").unwrap();
}

pub trait RunnableMachine {
    fn run(&mut self, steps: usize) -> Result<(), VirtualMachineError>;
}

#[derive(Debug)]
pub struct VirtualMachine {
    program: Program,
    iptr: usize,
    acc: isize,
}

impl VirtualMachine {
    pub fn from(input: &str) -> Result<Self, VirtualMachineError> {
        Ok(Self {
            program: Program::from(input)?,
            iptr: 0,
            acc: 0,
        })
    }

    pub fn acc(&self) -> isize {
        self.acc
    }

    pub fn iptr(&self) -> usize {
        self.iptr
    }
}

impl RunnableMachine for VirtualMachine {
    fn run(&mut self, steps: usize) -> Result<(), VirtualMachineError> {
        for _ in 0..steps {
            match self.program.get_instruction(self.iptr)? {
                Instruction::Acc(n) => {
                    self.acc += n;
                    self.iptr += 1;
                }
                Instruction::Jmp(n) => {
                    let new_iptr = self.iptr as isize + n;
                    if new_iptr < 0 {
                        return Err(VirtualMachineError::InstructionPtrOutOfRange(new_iptr))
                    } else {
                        self.iptr = new_iptr as usize
                    }
                }
                Instruction::Nop(_) => self.iptr += 1,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn from(input: &str) -> Result<Self, VirtualMachineError> {
        let instructions = input
            .split_terminator("\n")
            .map(Instruction::from)
            .collect::<Result<Vec<Instruction>, ParseError>>()?;
        Ok(Program { instructions })
    }

    pub fn get_instruction(&self, index: usize) -> Result<&Instruction, VirtualMachineError> {
        match self.instructions.get(index) {
            Some(i) => Ok(i),
            None => Err(VirtualMachineError::InstructionPtrOutOfRange(index as isize)),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Nop(isize),
    Jmp(isize),
    Acc(isize),
}

impl Instruction {
    pub fn from(line: &str) -> Result<Self, ParseError> {
        match INSTRUCTION_RE.captures(line) {
            Some(captures) => {
                let opcode_match: Option<&str> =
                    captures.name("opcode").and_then(|c| Some(c.as_str()));
                let arg_match: Option<isize> = captures
                    .name("arg")
                    .and_then(|c| Some(c.as_str()))
                    .and_then(|s| s.parse().ok());
                match (opcode_match, arg_match) {
                    (None, _) => Err(ParseError::NoCaptures),
                    (_, None) => Err(ParseError::InvalidArg),
                    (Some("nop"), Some(arg)) => Ok(Self::Nop(arg)),
                    (Some("acc"), Some(arg)) => Ok(Self::Acc(arg)),
                    (Some("jmp"), Some(arg)) => Ok(Self::Jmp(arg)),
                    (Some(_), _) => Err(ParseError::InvalidOpcode),
                }
            }
            None => Err(ParseError::NoCaptures),
        }
    }
}

#[derive(Debug)]
pub enum VirtualMachineError {
    ParseError(ParseError),
    InstructionPtrOutOfRange(isize),
}

impl Display for VirtualMachineError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{:?}", self);
        Ok(())
    }
}

impl From<ParseError> for VirtualMachineError {
    fn from(e: ParseError) -> VirtualMachineError {
        VirtualMachineError::ParseError(e)
    }
}

#[derive(Debug)]
pub enum ParseError {
    NoCaptures,
    InvalidArg,
    InvalidOpcode,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "{:?}", self);
        Ok(())
    }
}
