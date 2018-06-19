extern crate aqabler;

use aqabler::Eval;
use aqabler::Input;
use aqabler::MainMemory;
use aqabler::Parser;
use aqabler::Registers;
use aqabler::Storage;

use std::env;
use std::fs::read_to_string;
use std::path::Path;

// The entry point
fn main() {
    // Fetch the arguments into an array
    let arguments: Vec<String> = env::args().collect();
    // We require a filename
    if arguments.len() < 2 {
        println!("Expected filename");
        return;
    }
    // Check the file exists
    let path = Path::new(&arguments[1]);
    if path.exists() {
        // Read the file into a string
        match read_to_string(path) {
            Ok(program) => {
                // We have 12 registers
                let mut regs = Registers::create(12);
                // On 64bit system this gives us 64k "RAM"
                let mut main = MainMemory::create(1000);
                // Parse the program
                match Input::from(program).parse() {
                    Ok((p, l)) => {
                        // Run the program
                        if let Err(e) = p.eval(&l, &mut regs, &mut main) {
                            // Showing any error
                            println!("{}", e);
                        }
                        // Show the end state of the registers
                        for (i, v) in regs.iter().enumerate() {
                            println!("{}: {}", i, v);
                        }
                    }
                    // Opps syntax error
                    Err(e) => println!("{}", e),
                }
            }
            // Or not...
            Err(e) => println!("Can't read {}: {}", path.display(), e),
        }
    } else {
        // It didn't
        println!("{} doesn't exist", path.display());
    }
}
