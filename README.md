# Brainfuck-rs

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter, written in Rust.

This is my solution for the CodinGame ["WHAT THE BRAINFUCK !"](https://www.codingame.com/training/medium/what-the-brainfuck) puzzle.

## Building the project

CodinGame accepts code solutions in a single file only. For that a bundle of the `codingame` executable is created when you build the project.

```bash
cargo build
```

This will:

1. Build binary executables for both

  - `brainfuck`: the default executable with a CLI interface. Run this executable with the `-h` flag for info on the expected input. Binary location: `target/debug/brainfuck`.
  - `codingame`: the executable with the CodinGame puzzle format. This one requires the inputs in a specific format [specified below](#codingame_exec). Binary location: `target/debug/codingame`.

2. Create a bundled version of the `codingame` executable at `target/codingame.rs`. This bundled file can be copy-pasted into the CodinGame IDE.

## Running Brainfuck programs

There are some example Brainfuck files that you can use with the interpreter, these are located in the `examples` directory.

### Running with cargo

To run the a program with the cargo CLI, you must pass the program's CLI arguments after a double dash (`--`), to differentiate them from the cargo CLI's args.

In order to successfully run a program, check the required memory size stated at the top of the file and pass it as the `size` argument with `-s` or `--size`.

#### Command structure

```bash
cargo run -- <file_path> -s <memory_size>
```

#### Example

```bash
cargo run -- examples/hello_world.bf -s 4
```

### Running the compiled binary

First you have to build the project. This will create a binary executable located at `target/debug/brainfuck`.

You now have two options:

1. Execute the binary from it's path.
2. Add the executable to you `$PATH` in order to make it available everywhere in your machine and run it from any location (not recommended, but I won't interfere with your decisions).

For this example let's do option 1:

```bash
# Copy the binary to the root of the project
cp target/debug/brainfuck .

# Execute the binary, passing the "hello_world.bf" example file
# as input with 4 bytes of memory
./brainfuck examples/hello_world.bf -s 4

# Output: Hello World!
```

#### CLI Arguments

The `brainfuck` interpreter accepts the following arguments:

- `[Path]`: path of the brainfuck file to execute. Must be the first argument.
- `--size` or `-s`: the memory size in bytes that will be made available to the running program.
- `--inputs` or `-i`: **Optional**. A space-separated array of inputs that the program will read whenever an input instruction is executed.

  If the `--inputs` argument is omitted, the program will read a line from `stdin` whenever an input is required.

  If the program requires more inputs that received, the program will exit with the error code `NO INPUT FOUND`.

<h2 id="codingame_exec">Running the <code>codingame</code> executable</h2>

The `codingame` executable works the same as the regular executable, but requires the inputs to be specified in a different way:

> **Line 1:** Three integers L, S and N for the program line count, the needed array size and the inputs count.
> **Next L lines:** A line of the Brainfuck program.
> **Next N lines:** An integer input to the Brainfuck program.

### Hello World example

This example requires `1` program line, `4` bytes of memory and no user input, so `N` will be `0`

```bash
# Run the codigame binary
./target/debug/codingame

# Inputs
1 4 0
++++++++++[>+++++++>++++++++++>+++<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.

# Output: Hello World!
```

## CodinGame Puzzle statement

Taken from [the puzzle page](https://www.codingame.com/training/medium/what-the-brainfuck)

> Brainfuck is a minimalist programming language consisting of 8 commands. That's all.
> However it is Turing complete and allows you to make whatever you want, if you are very patient and motivated.
>
> Your goal is to create a fully functional Brainfuck interpreter.
> Let see how it works.
>
> The Brainfuck model is composed of three elements:
>
> - An array of `S` one byte cells initialized to 0, and indexed from 0.
> - A pointer initialized to point to the first cell (index 0) of the array.
> - A program made up of the 8 valid instructions.
>
> The following are the instructions:
>
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
>
> - `"SYNTAX ERROR"` if a `[` appears to have no `]` to jump to, or vice versa. Note that this error must be raised before the execution of the program, no matter its position in the Brainfuck code.
> - `"POINTER OUT OF BOUNDS"` if the pointer position goes below `0` or above `S - 1`.
> - `"INCORRECT VALUE"` if after an operation the value of a cell becomes negative or higher than `255`.
