use std::{collections::VecDeque, io};

#[macro_export]
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
pub use parse_input;

pub fn input() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

pub fn get_program_lines(lines: i32) -> Vec<String> {
    let mut program_lines = Vec::new();
    for _ in 0..lines as usize {
        let r = input().trim_matches('\n').to_string();
        program_lines.push(r);
    }
    program_lines
}

pub fn get_program_inputs(lines: i32) -> VecDeque<u8> {
    let mut program_inputs = VecDeque::new();
    for _ in 0..lines as usize {
        let c = parse_input!(input(), u8);
        program_inputs.push_back(c);
    }
    program_inputs
}
