use clap::Parser;

/// Brainfuck
#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Args {
    /// Path to the program file
    pub path: String,

    /// Size of the memory (in bytes)
    #[arg(short, long, verbatim_doc_comment)]
    pub size: usize,

    /// Inputs for the program (1 byte each)
    /// If ignored, the program will read from stdin whenever an input is required.
    #[arg(short, long, num_args=0.., verbatim_doc_comment)]
    pub inputs: Option<Vec<u8>>,
}
