# Greppy
My first mini-project to learn Rust language

## How to compile the project
In the project root, you can run the command `cargo install greppy` to compile the project and install it.

You can also run the command `cargo build --release`, then you'll find the binary (compiled) version in `./target/release/greppy`

## How to use

## How to use the project

As mentioned earlier, the current project is a simplified version of `grep` command.

You can use it as the following:
`greppy --source-path ~/example_path --text-to-find "example_text"`

Or a simplified version: `greppy -s ~/example_path -t example_text`.

N.B. You can also use `--help` (or `-h`) option to get the full details and list of
options.
