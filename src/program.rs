use std::collections::VecDeque;

use crate::{errors::BrainFuckError, syntax::Instruction};

pub struct Program {
    instructions: Vec<Instruction>,
    pointer: usize,
    inputs: VecDeque<u8>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>, inputs: VecDeque<u8>) -> Self {
        let pointer = 0;
        Self {
            instructions,
            pointer,
            inputs,
        }
    }

    pub fn get_current_instruction(&self) -> &Instruction {
        &self.instructions[self.pointer]
    }

    pub fn completed(&self) -> bool {
        self.pointer == self.instructions.len()
    }

    pub fn increment_pointer(&mut self) -> Result<(), BrainFuckError> {
        if self.pointer == self.instructions.len() {
            return Err(BrainFuckError::PointerOutOfBounds);
        }
        self.pointer += 1;
        Ok(())
    }

    pub fn set_pointer(&mut self, location: usize) -> Result<(), BrainFuckError> {
        if location >= self.instructions.len() {
            return Err(BrainFuckError::PointerOutOfBounds);
        }
        self.pointer = location;
        Ok(())
    }

    pub fn consume_next_input(&mut self) -> Result<u8, BrainFuckError> {
        self.inputs.pop_front().ok_or(BrainFuckError::NoInputFound)
    }
}
