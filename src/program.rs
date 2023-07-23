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

    pub fn get_pointer(&self) -> usize {
        self.pointer
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

    pub fn get_closing_bracket_pointer(&self) -> Result<usize, BrainFuckError> {
        let mut open_brackets = 1;
        let mut search_ptr = self.pointer;
        let instruction_len = self.instructions.len();

        // Loop until the open brackets count goes to zero
        while open_brackets >= 1 && search_ptr < (instruction_len - 1) {
            search_ptr += 1;
            match self.instructions.get(search_ptr) {
                Some(instruction) => match instruction {
                    Instruction::JumpForward => open_brackets += 1,
                    Instruction::JumpBackward => open_brackets -= 1,
                    _ => {}
                },
                None => return Err(BrainFuckError::PointerOutOfBounds),
            }
        }
        let final_instruction = self.instructions.get(search_ptr);
        let pointer_is_valid = search_ptr < instruction_len;
        let is_expected_instruction = final_instruction == Some(&Instruction::JumpBackward);

        if !pointer_is_valid || !is_expected_instruction {
            return Err(BrainFuckError::SyntaxError);
        }
        Ok(search_ptr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_the_closing_bracket_position() {
        let instructions = vec![
            Instruction::JumpForward,
            Instruction::IncrementPointer,
            Instruction::JumpBackward,
        ];

        let program = Program::new(instructions, VecDeque::new());
        let closing_pos = program.get_closing_bracket_pointer();

        assert_eq!(closing_pos, Ok(2));
    }

    #[test]
    fn returns_syntax_error_when_no_closing_exist() {
        let instructions = vec![Instruction::JumpForward, Instruction::IncrementPointer];

        let program = Program::new(instructions, VecDeque::new());
        let closing_pos = program.get_closing_bracket_pointer();

        assert_eq!(closing_pos, Err(BrainFuckError::SyntaxError));
    }
}
