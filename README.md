# Brainfuck-rs

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter, written in Rust.

This is my solution for the Clash Of Code ["WHAT THE BRAINFUCK !"](https://www.codingame.com/training/medium/what-the-brainfuck) puzzle.


> **Note**
> Work in progress

## Build

CodinGame accepts code solutions in a single file only, so to bundle the code you must run.

```bash
cargo build
```

And the project will bundle into a single file located at `./target/codingame.rs`


## Puzzle statement

Taken from the puzzle page

> Brainfuck is a minimalist programming language consisting of 8 commands. That's all.
> However it is Turing complete and allows you to make whatever you want, if you are very patient and motivated.
> 
> Your goal is to create a fully functional Brainfuck interpreter.
> Let see how it works.
> 
> The Brainfuck model is composed of three elements:
> - An array of `S` one byte cells initialized to 0, and indexed from 0.
> - A pointer initialized to point to the first cell (index 0) of the array.
> - A program made up of the 8 valid instructions.
> 
> The following are the instructions:
> - `>` increment the pointer position.
> - `<` decrement the pointer position.
> - `+` increment the value of the cell the pointer is pointing to.
> - `-` decrement the value of the cell the pointer is pointing to.
> - `.` output the value of the pointed cell, interpreting it as an ASCII value.
> - `,` accept a positive one byte integer as input and store it in the pointed cell.
> - `[` jump to the instruction after the corresponding `]` if the pointed cell's value is `0`.
> - `]` go back to the instruction after the corresponding `[` if the pointed cell's value is different from `0`.
> 
> Note: The `[` and `]` commands always come in pairs, and in case of nested `[]` the first `[` always correspond to the last `]`.
>
> **Be careful:** A Brainfuck program can contain any characters, that allow the developers to comment their code and to make it more readable. Of course your interpreter must ignore all of these "inactive" characters.
>
> In some cases, errors might be encountered. When this happens you have to stop the execution of the program and print the correct error message from the following list:
> - `"SYNTAX ERROR"` if a `[` appears to have no `]` to jump to, or vice versa. Note that this error must be raised before the execution of the program, no matter its position in the Brainfuck code.
> - `"POINTER OUT OF BOUNDS"` if the pointer position goes below `0` or above `S - 1`.
> - `"INCORRECT VALUE"` if after an operation the value of a cell becomes negative or higher than `255`.
