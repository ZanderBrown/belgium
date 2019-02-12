use aqabler::ChangeEvent;
use aqabler::Eval;
use aqabler::Input;
use aqabler::Memory;
use aqabler::Observer;
use aqabler::Parser;
use aqabler::Storage;

use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::rc::Rc;

struct RChange;

impl Observer<ChangeEvent> for RChange {
    fn notify(&self, evt: ChangeEvent) {
        println!("R{} <- {}", evt.idx, evt.val);
    }
}

struct MChange;

impl Observer<ChangeEvent> for MChange {
    fn notify(&self, evt: ChangeEvent) {
        println!("{} <- {}", evt.idx, evt.val);
    }
}

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
                let mut regs = Memory::create(12);
                let rc: Rc<Observer<ChangeEvent>> = Rc::new(RChange {});
                regs.add_observer(Rc::downgrade(&rc));
                // On 64bit system this gives us 64k "RAM"
                let mut main = Memory::create(1000);
                let rc: Rc<Observer<ChangeEvent>> = Rc::new(MChange {});
                main.add_observer(Rc::downgrade(&rc));
                // Parse the program
                match Input::from(program).parse() {
                    Ok((p, l)) => {
                        // Run the program
                        if let Err(e) = p.eval(&l, &mut regs, &mut main) {
                            // Showing any error
                            println!("{}", e);
                        }
                        // Show the end state of the registers
                        for (i, v) in Storage::iter(&regs) {
                            println!("R{}: {}", i, v);
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
