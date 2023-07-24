extern crate brainfuck_rs;

use brainfuck_rs::{executor::Executor, inputs, memory::Memory, program::Program, syntax};

// Version submitted to CodinGame
fn main() {
    let input_line = inputs::input();
    let inputs = input_line.split(" ").collect::<Vec<_>>();

    let l = inputs::parse_input!(inputs[0], i32);
    let s = inputs::parse_input!(inputs[1], usize);
    let n = inputs::parse_input!(inputs[2], i32);

    let input_lines = inputs::get_program_lines(l);

    let program_lines = input_lines.iter().map(|s| s.as_str()).collect();
    let program_inputs = inputs::get_program_inputs(n);

    let tokens = syntax::tokenize(&program_lines);

    if let Err(error) = syntax::analyze(&tokens) {
        println!("{}", error);
        return;
    }

    let mut program = Program::new(tokens);
    let mut memory = Memory::new(s);
    let mut executor = Executor::new(&mut program, &mut memory, Some(program_inputs));

    if let Err(error) = executor.run() {
        print!("{}", error);
    }
    println!("");
}
