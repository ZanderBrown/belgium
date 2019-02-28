use aqabler::execute;
use aqabler::Assemble;
use aqabler::ChangeEvent;
use aqabler::Input;
use aqabler::Memory;
use aqabler::Observer;
use aqabler::Storage;
use aqabler::{ADDRESS, CIR, COUNTER, MBUFF, STATUS};

use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::rc::Rc;

use getopts::Options;

struct RChange;

impl Observer<ChangeEvent> for RChange {
    fn notify(&self, evt: ChangeEvent) {
        match evt.idx {
            COUNTER => println!("Counter     to 0x{:08X}", evt.val),
            ADDRESS => println!("Address     to 0x{:08X}", evt.val),
            MBUFF => println!("Buffer      to 0x{:08X}", evt.val),
            CIR => println!("Instruction to 0x{:08X}", evt.val),
            STATUS => println!("Status      to 0x{:08X}", evt.val),
            _ => println!("R{:02}         to 0x{:08X} ({})", evt.idx, evt.val, evt.val),
        }
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

    // Setup the argument parser
    let mut opts = Options::new();
    opts.optflag("c", "classic", "use the classic evaluator");
    // Try and parse the arguments
    let matches = match opts.parse(&arguments[1..]) {
        // Store the result
        Ok(m) => m,
        // Something went wrong
        Err(f) => {
            // Display the message
            println!("{}", f);
            // Quit early
            return;
        }
    };
    // If a file wasn't passed
    let input = if matches.free.is_empty() {
        println!("Expected a file");
        // Exit
        return;
    } else {
        // Get the filename
        matches.free[0].clone()
    };
    // Check the file exists
    let path = Path::new(&input);
    if path.exists() {
        // Read the file into a string
        match read_to_string(path) {
            Ok(program) => {
                // We have 12 registers
                let mut regs = Memory::create(String::from("register"), 18);
                let rc: Rc<Observer<ChangeEvent>> = Rc::new(RChange {});
                regs.add_observer(Rc::downgrade(&rc));
                let mut main = Memory::create(String::from("memory"), 500);
                let rc: Rc<Observer<ChangeEvent>> = Rc::new(MChange {});
                main.add_observer(Rc::downgrade(&rc));
                let mut inp = Input::from(program);
                // Parse the program
                match inp.assemble(&mut main) {
                    Ok(()) => {
                        for (i, v) in Storage::iter(&main) {
                            println!("0x{:04X}: 0x{:08X} {}", i, v, v);
                        }
                        if let Err(err) = execute(&mut main, &mut regs) {
                            println!("{}", err);
                        }
                    }
                    // Opps error
                    Err(e) => println!("{}", e),
                }
                // Show the end state of the registers
                for (i, v) in Storage::iter(&regs) {
                    println!("R{:02}: {}", i, v);
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
