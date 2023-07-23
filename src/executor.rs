use crate::{errors::BrainFuckError, memory::Memory, program::Program, syntax::Instruction};

pub struct Executor<'a> {
    program: &'a mut Program,
    memory: &'a mut Memory,
    // Used to store the instruction pointers of the JumpForwards
    call_stack: Vec<usize>,
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
            self.program.increment_pointer()?;
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
            Instruction::Output => self.execute_output(),
            Instruction::Input => self.execute_input()?,
            Instruction::JumpForward => self.execute_jump_forward()?,
            Instruction::JumpBackward => self.execute_jump_backward()?,
        }
        Ok(())
    }

    fn execute_output(&self) {
        print!("{}", self.memory.get() as char);
    }

    fn execute_input(&mut self) -> Result<(), BrainFuckError> {
        let input = self.program.consume_next_input()?;
        self.memory.set(input);
        Ok(())
    }

    fn execute_jump_forward(&mut self) -> Result<(), BrainFuckError> {
        if self.memory.get() == 0 {
            let new_instruction_ptr = self.program.get_closing_bracket_pointer()?;
            self.program.set_pointer(new_instruction_ptr)?;
        } else {
            self.call_stack.push(self.program.get_pointer());
        }
        Ok(())
    }

    fn execute_jump_backward(&mut self) -> Result<(), BrainFuckError> {
        if self.memory.get() == 0 {
            self.call_stack.pop();
        } else {
            let open_bracket_ptr = self
                .call_stack
                .last()
                .ok_or(BrainFuckError::PointerOutOfBounds)?;
            self.program.set_pointer(*open_bracket_ptr)?;
        }
        Ok(())
    }
}
