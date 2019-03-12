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

struct RChange {
    verbose: bool,
}

impl Observer<ChangeEvent> for RChange {
    fn notify(&self, evt: ChangeEvent) {
        match evt.idx {
            COUNTER => {
                if self.verbose {
                    println!("Counter     to 0x{:08X}", evt.val)
                }
            }
            ADDRESS => {
                if self.verbose {
                    println!("Address     to 0x{:08X}", evt.val)
                }
            }
            MBUFF => {
                if self.verbose {
                    println!("Buffer      to 0x{:08X}", evt.val)
                }
            }
            CIR => {
                if self.verbose {
                    println!("Instruction to 0x{:08X}", evt.val)
                }
            }
            STATUS => {
                if self.verbose {
                    println!("Status      to 0x{:08X}", evt.val)
                }
            }
            _ => println!("R{:02}         to 0x{:08X} ({})", evt.idx, evt.val, evt.val),
        }
    }
}

struct MChange;

impl Observer<ChangeEvent> for MChange {
    fn notify(&self, evt: ChangeEvent) {
        println!("Memory {:04} to 0x{:08X} ({})", evt.idx, evt.val, evt.val);
    }
}

// The entry point
fn main() {
    // Fetch the arguments into an array
    let arguments: Vec<String> = env::args().collect();
    let program = arguments[0].clone();

    // Setup the argument parser
    let mut opts = Options::new();
    opts.optflag(
        "v",
        "verbose",
        "show system register changes (overrides -c)",
    );
    opts.optflag("c", "reg-changed", "show changes to registers");
    opts.optflag("m", "mem-changed", "show changes to memory");
    opts.optflag("i", "dump-inital", "show inital state of memory");
    opts.optflag("f", "dump-final", "show final state of memory");
    opts.optflag("r", "registers", "show final state of registers");
    opts.optopt("s", "mem-size", "set the size of memory (default=500)", "");
    opts.optflag("h", "help", "print this help menu");

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

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options] FILE", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    // If a file wasn't passed
    let input = if matches.free.is_empty() {
        println!("Expected a file");
        // Exit
        return;
    } else {
        // Get the filename
        matches.free[0].clone()
    };

    let size = if let Some(s) = matches.opt_str("s") {
        if let Ok(size) = s.parse() {
            size
        } else {
            println!("Unrecognised number '{}'", s);
            return;
        }
    } else {
        500
    };

    // Check the file exists
    let path = Path::new(&input);
    if path.exists() {
        // Read the file into a string
        match read_to_string(path) {
            Ok(program) => {
                // We have 12 registers
                let mut regs = Memory::create(String::from("register"), 18);

                // Declared outside the if to keep a local reference
                let rc: Rc<Observer<ChangeEvent>> = Rc::new(RChange {
                    verbose: matches.opt_present("v"),
                });
                if matches.opt_present("v") || matches.opt_present("c") {
                    regs.add_observer(Rc::downgrade(&rc));
                }

                let mut main = Memory::create(String::from("memory"), size);
                let rc: Rc<Observer<ChangeEvent>> = Rc::new(MChange {});
                if matches.opt_present("m") {
                    main.add_observer(Rc::downgrade(&rc));
                }

                let mut inp = Input::from(program);
                // Parse the program
                match inp.assemble(&mut main) {
                    Ok(()) => {
                        if matches.opt_present("i") {
                            for (i, v) in Storage::iter(&main) {
                                println!("0x{:04X}: 0x{:08X} {:10}", i, v, v);
                            }
                        }

                        if let Err(err) = regs.set(COUNTER, 0) {
                            println!("{}", err);
                        }

                        loop {
                            match execute(&mut main, &mut regs) {
                                Ok(res) => {
                                    if !res {
                                        break;
                                    }
                                }
                                Err(err) => println!("{}", err),
                            }
                        }

                        if matches.opt_present("f") {
                            for (i, v) in Storage::iter(&main) {
                                println!("0x{:04X}: 0x{:08X} {:10}", i, v, v);
                            }
                        }
                    }
                    // Opps error
                    Err(e) => println!("{}", e),
                }
                // Show the end state of the registers
                if matches.opt_present("r") {
                    for (i, v) in Storage::iter(&regs) {
                        println!("R{:02}: {}", i, v);
                    }
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
