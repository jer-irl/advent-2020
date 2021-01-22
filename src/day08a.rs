use std::collections::HashSet;

use super::errors::AdventError;
use super::vm::{errors::VirtualMachineError, Program, RunnableMachine, VirtualMachine};

pub fn solve(input: &str) -> Result<(), AdventError> {
    let program = match Program::from(input) {
        Ok(p) => p,
        Err(VirtualMachineError::ParseError(_)) => return Err(AdventError::ParseError),
        Err(_) => unreachable!(),
    };
    let mut vm = VirtualMachine::from(program);

    let mut hit_instructions = HashSet::new();

    loop {
        hit_instructions.insert(vm.iptr());
        match vm.run(1) {
            Err(VirtualMachineError::InstructionPtrOutOfRange(_)) => {
                return Err(AdventError::NoSolution)
            }
            Err(_) => return Err(AdventError::ParseError),
            Ok(_) => (),
        }
        if hit_instructions.contains(&vm.iptr()) {
            println!("{}", vm.acc());
            return Ok(());
        }
    }
}
