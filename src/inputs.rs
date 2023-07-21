use std::io;

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

pub fn get_program_inputs(lines: i32) -> Vec<i32> {
    let mut program_inputs = Vec::new();
    for _ in 0..lines as usize {
        let c = parse_input!(input(), i32);
        program_inputs.push(c);
    }
    program_inputs
}
