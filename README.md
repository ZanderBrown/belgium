# Belgium

![belgium logo](logo.png)

Belgium, an implementation of CdM-8 v4.

Currently only implements a VM supporting a subset of opcodes

The implementation is in [Rust](https://www.rust-lang.org/) as it provides
native performance but with a smarter compiler and built-in WASM support. Also
because I like it.

Partially based on reversing Cocas/CocoIDE (c) Prof. Alex Shaferenko

## Usage

Like most projects written in rust we use
[cargo](https://doc.rust-lang.org/cargo/) as our build system. To build belgium
first use [rustup](https://rustup.rs/) to install the rust
[toolchain](https://en.wikipedia.org/wiki/Toolchain) & cargo. Then
[clone](https://help.github.com/articles/cloning-a-repository/) belgium to you
machine and navigate to it's directory in a terminal & run:

```
cargo build --release
```
To make a release build of belgium or to compile & run
```
cargo run --release --bin belgium <file.asm>
```
Where `<file.asm>` is a path to a file containing CdM-8 assembly

## Why the name?

Inside joke

> If today is Tuesday, then this is Belgium. Today is Tuesday. This is Belgium.
