# Rust Shell

This is a simple unix shell built using Rust. It is meant to provide minimal
command-line interface to execute basic shell commands, navigate the file 
system, and interact with system processe.

## Features

* Execute system commands
* Navigate directories(cd, ls, pwd)
* Run scripts and custom commands
* Handle simple input/output operations

## Installation

Ensure you have Rust installed. If not run the following commands to have it up
and running.

```sh
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone this repository and run build for the project:

```sh
 git clone https://github.com/ckchesire/simple_rust_shell.git
 cd simple_rust_shell
 cargo build --release
```
## Usage

Run the shell using:

```sh
 cargo run
```
Or, if built :

```
 target/release/simple_rust_shell.git
```
