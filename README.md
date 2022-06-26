# M_grep
My first mini-project to learn Rust language

## How to compile the project
In the project's root, you can run the command `cargo build --release`, then
you'll find the binary (compiled) version in `./target/release/m_grep`

## How to use the project
As mentioned earlier, the current project is a simplified version of `grep` command.

You can use it as the following:
`./m_grep --source-path ~/example_path --text-to-find "example_text"`

Or a simplified version: `./m_grep -s ~/example_path -t example_text`.

You can also install it by moving the binary to `/usr/bin` folder, and therefore
call `m_grep -s some_path -t some_text`
