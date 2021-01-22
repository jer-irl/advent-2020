use std::collections::HashSet;

use super::errors::AdventError;
use super::vm::{
    errors::VirtualMachineError, Instruction, Program, RunnableMachine, VirtualMachine,
};

pub fn solve(input: &str) -> Result<(), AdventError> {
    let input_program = Program::from(input);
    if let Err(VirtualMachineError::ParseError(_)) = input_program {
        return Err(AdventError::ParseError);
    }
    let input_program = input_program.unwrap();

    for i in 0..input_program.len() {
        let instruction_to_substitute = match input_program.get_instruction(i) {
            Some(Instruction::Nop(n)) => Instruction::Jmp(*n),
            Some(Instruction::Jmp(n)) => Instruction::Nop(*n),
            Some(_i) => continue,
            None => unreachable!(),
        };
        let mut program_to_run = input_program.clone();
        program_to_run
            .replace_instruction(i, instruction_to_substitute)
            .unwrap();
        if let Some(acc) = get_termination_acc(program_to_run)? {
            println!("{}", acc);
            return Ok(());
        }
    }

    Err(AdventError::NoSolution)
}

fn get_termination_acc(program: Program) -> Result<Option<isize>, AdventError> {
    let target_iptr = program.len();
    let mut hit_instructions = HashSet::new();
    let mut vm = VirtualMachine::from(program);
    loop {
        hit_instructions.insert(vm.iptr());
        match vm.run(1) {
            Err(VirtualMachineError::InstructionPtrOutOfRange(_)) => {
                return Err(AdventError::NoSolution)
            }
            Err(_) => return Err(AdventError::ParseError),
            Ok(()) => (),
        }
        if hit_instructions.contains(&vm.iptr()) {
            return Ok(None);
        } else if vm.iptr() == target_iptr {
            return Ok(Some(vm.acc()));
        }
    }
}
