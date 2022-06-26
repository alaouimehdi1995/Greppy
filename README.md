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

You can also install it by moving the binary to `/usr/local/bin` folder:
```sh
sudo cp ./target/release/m_grep /usr/local/bin
```
and then, use `m_grep` command as the following: `m_grep -s some_path -t some_text`

N.B. You can also use `--help` (or `-h`) option to get the full details and list of
options of the binary.
