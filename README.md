# AQAbler

![aqabler logo](logo.png)

"aquabler", an implementation of [AQA ALI](http://filestore.aqa.org.uk/resources/computing/AQA-75162-75172-ALI.PDF) (which is subset of [ARM](https://en.wikipedia.org/wiki/ARM_architecture) [Assembly](https://en.wikipedia.org/wiki/Assembly_language)). While not a true assembler (and lacking debugging tools) aqabler gives students a chance to play with the ALI that will be used in their exams (AQA AS/A-Level Computer Science [7516 or 7517](http://www.aqa.org.uk/subjects/computer-science-and-it/as-and-a-level/computer-science-7516-7517))

The implementation is in [Rust](https://www.rust-lang.org/) to gain the performance of the likes of C but with pointer safety. We target rust stable and the aqabler library only requires `std::collections::HashMap` & `std::fmt` for the standard library with no other dependancies, whilst the cli requires parts of `std::fs` & `std::path` to read source files

Aqabler provides a very simple language with very few verbs, it doesn't even have [comments](https://en.wikipedia.org/wiki/Comment_(computer_programming)) but this is because we only intend to support the language as AQA defines it. Feel free to fork and add extra commands just be aware we are unlikly to accept such pull requests

## Usage

Like most projects written in rust we use [cargo](https://doc.rust-lang.org/cargo/) as our build system. To build aqabler first use [rustup](https://rustup.rs/) to install the rust [toolchain](https://en.wikipedia.org/wiki/Toolchain) & cargo. Then [clone](https://help.github.com/articles/cloning-a-repository/) aqabler to you machine and navigate to it's directory in a terminal & run:

```
cargo build --release
```
To make a release build of aqabler or to compile & run
```
cargo run --release --bin aqabler <file.aqb>
```
Where `<file.aqb>` is a path to a file containing AQA ALI code