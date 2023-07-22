use crate::{errors::BrainFuckError, memory::Memory, program::Program, syntax::Instruction};

pub struct Executor<'a> {
    program: &'a mut Program,
    memory: &'a mut Memory,
    call_stack: Vec<Instruction>,
}

impl<'a> Executor<'a> {
    pub fn new(program: &'a mut Program, memory: &'a mut Memory) -> Self {
        let call_stack = Vec::new();
        Self {
            program,
            call_stack,
            memory,
        }
    }

    pub fn run(&mut self) -> Result<(), BrainFuckError> {
        while !self.program.completed() {
            self.execute_instruction()?;
            self.move_instruction_pointer()?;
        }
        Ok(())
    }

    fn execute_instruction(&mut self) -> Result<(), BrainFuckError> {
        let instruction = self.program.get_current_instruction();
        match instruction {
            Instruction::IncrementPointer => self.memory.increment_pointer()?,
            Instruction::DecrementPointer => self.memory.decrement_pointer()?,
            Instruction::IncrementValue => self.memory.increment_value()?,
            Instruction::DecrementValue => self.memory.decrement_value()?,
            Instruction::Output => {
                print!("{}", self.memory.get() as char);
            }
            Instruction::Input => todo!(),
            Instruction::JumpForward => todo!(),
            Instruction::JumpBackward => todo!(),
        }
        Ok(())
    }

    fn move_instruction_pointer(&mut self) -> Result<(), BrainFuckError> {
        let instruction = self.program.get_current_instruction();
        match instruction {
            Instruction::JumpForward | Instruction::JumpBackward => {
                todo!("implement Jumps' pointer movements")
            }
            _ => self.program.increment_pointer(),
        }
    }
}
