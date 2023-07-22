use std::vec;

use crate::errors::BrainFuckError;

struct Memory {
    data: Vec<u8>,
    pointer: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let data = vec![0; size];
        let pointer = 0;

        Self { data, pointer }
    }

    pub fn get(&self) -> u8 {
        self.data[self.pointer]
    }

    pub fn set(&mut self, value: u8) {
        self.data[self.pointer] = value;
    }

    pub fn increment(&mut self) -> Result<(), BrainFuckError> {
        if self.get() == 255 {
            return Err(BrainFuckError::IncorrectValue);
        }
        self.data[self.pointer] += 1;
        Ok(())
    }

    pub fn decrement(&mut self) -> Result<(), BrainFuckError> {
        if self.get() == 0 {
            return Err(BrainFuckError::IncorrectValue);
        }
        self.data[self.pointer] -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_success() {
        let mut mem = Memory::new(1);
        let result = mem.increment();

        assert_eq!(result, Ok(()));
        assert_eq!(mem.get(), 1);
    }

    #[test]
    fn test_increment_error() {
        let mut mem = Memory::new(1);
        // Set to max allowed value
        mem.set(255);
        let result = mem.increment();

        assert_eq!(result, Err(BrainFuckError::IncorrectValue));
    }

    #[test]
    fn test_decrement_success() {
        let mut mem = Memory::new(1);
        mem.set(1); // Set to 1 so it can be decremented
        let result = mem.decrement();

        assert_eq!(result, Ok(()));
        assert_eq!(mem.get(), 0); // The value at the current pointer should now be 0
    }

    #[test]
    fn test_decrement_error() {
        let mut mem = Memory::new(1);
        // The memory is initialized to 0, so decrementing should cause an error
        let result = mem.decrement();

        assert_eq!(result, Err(BrainFuckError::IncorrectValue));
    }
}
