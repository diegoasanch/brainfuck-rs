extern crate brainfuck_rs;

use brainfuck_rs::{executor::Executor, inputs, memory::Memory, program::Program, syntax};

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let input_line = inputs::input();
    let inputs = input_line.split(" ").collect::<Vec<_>>();

    let l = inputs::parse_input!(inputs[0], i32);
    let s = inputs::parse_input!(inputs[1], usize);
    let n = inputs::parse_input!(inputs[2], i32);

    let program_lines = inputs::get_program_lines(l);
    let program_inputs = inputs::get_program_inputs(n);

    let tokens = syntax::tokenize(&program_lines);

    if let Err(error) = syntax::analyze(&tokens) {
        println!("{}", error);
        return;
    }

    let mut program = Program::new(tokens, program_inputs);
    let mut memory = Memory::new(s);

    let mut executor = Executor::new(&mut program, &mut memory);

    if let Err(error) = executor.run() {
        print!("{}", error);
    }
    println!("");
}
