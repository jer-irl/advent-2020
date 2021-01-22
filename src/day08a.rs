use std::collections::HashSet;

use super::errors::AdventError;
use super::vm::{RunnableMachine, VirtualMachine, VirtualMachineError};

pub fn solve(input: &str) -> Result<(), AdventError> {
    let vm = VirtualMachine::from(input);
    if let Err(VirtualMachineError::ParseError(_)) = vm {
        return Err(AdventError::ParseError);
    }
    let mut vm = vm.unwrap();
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
