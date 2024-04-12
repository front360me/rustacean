# Cargo

## Cargo commands

- To create a new project use `cargo new`, e.g. `cargo new my_project`.
- To build project use `cargo build`.
- To build and run a project use `cargo run`.
- You can build a project without producing a binary to check for errors `cargo check`.
- Build documentation for the project and open it: `cargo doc --open`


The `cargo check` is much faster than `cargo build` because it skips the step of producing an executable. 
Many rustaceans run `cargo check` periodically as they write their program to make sure it compiles. 

## Cargo version

To see the cargo version

```
cargo --version
```

## Install a new project

To install new broject

```
cargo new hello_cargo
```

It creates a folder with a project files. 

The folder contains *Cargo.toml* file that is Cargo's configuration format. TOML means *Tom's Obvious, Minimal Language*.

## Cargo project files

- *Cargo.toml* - Cargo's configuration format. TOML means *Tom's Obvious, Minimal Language*.

- *Cargo.lock* - This file keeps track of the exact versions of dependencies in the project. 

- *target* folder. It contains built application.
