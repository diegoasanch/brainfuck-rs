use std::{fs, process::ExitCode};

use brainfuck_rs::{cli::Args, executor::Executor, memory::Memory, program::Program, syntax};
use clap::Parser;

fn main() -> ExitCode {
    let args = Args::parse();

    // Read the program from the specified path
    let program_content = match fs::read_to_string(&args.path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to open file \"{}\": {}", args.path, error);
            return ExitCode::FAILURE;
        }
    };

    // Tokenize and analyze the program
    let instructions = syntax::tokenize(&vec![&program_content]);
    if let Err(error) = syntax::analyze(&instructions) {
        println!("{}", error);
        return ExitCode::FAILURE;
    };

    let mut program = Program::new(instructions);
    let mut memory = Memory::new(args.size);
    let mut executor = Executor::new(&mut program, &mut memory, None);

    let run_result = executor.run();

    match run_result {
        Ok(_) => {
            // Print new line to flush stdout
            println!("");
            ExitCode::SUCCESS
        }
        Err(error) => {
            println!("{}", error);
            ExitCode::FAILURE
        }
    }
}
