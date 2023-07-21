use crate::errors::BrainFuckError;

#[derive(Debug)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    JumpForward,
    JumpBackward,
}

impl Instruction {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Self::IncrementPointer),
            '<' => Some(Self::DecrementPointer),
            '+' => Some(Self::IncrementValue),
            '-' => Some(Self::DecrementValue),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            '[' => Some(Self::JumpForward),
            ']' => Some(Self::JumpBackward),
            _ => None,
        }
    }
}

pub fn tokenize(program: &Vec<String>) -> Vec<Instruction> {
    let mut tokens = Vec::new();
    for line in program {
        for c in line.chars() {
            if let Some(token) = Instruction::from_char(c) {
                tokens.push(token);
            }
        }
    }
    tokens
}

// Analyzes the syntax of the program, if the brackets don't have a correct match
// the syntax is invalid
pub fn analyze(program: &Vec<Instruction>) -> Result<(), BrainFuckError> {
    let mut open_brackets = 0;
    for instruction in program {
        match instruction {
            Instruction::JumpForward => open_brackets += 1,
            Instruction::JumpBackward => open_brackets -= 1,
            _ => {}
        }
        if open_brackets < 0 {
            return Err(BrainFuckError::SyntaxError);
        }
    }
    if open_brackets != 0 {
        return Err(BrainFuckError::SyntaxError);
    }
    Ok(())
}