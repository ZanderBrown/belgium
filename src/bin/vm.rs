use belgium::ChangeEvent;
use belgium::Machine;
use belgium::Observer;
use belgium::{Response, COUNTER, SP, STATUS};

use std::env;
use std::fs::read;
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
                    println!("Counter     to 0x{:02X}", evt.val)
                }
            }
            STATUS => {
                if self.verbose {
                    println!("Status      to 0b{:08b}", evt.val)
                }
            }
            SP => {
                if self.verbose {
                    println!("Stack       to 0x{:02X}", evt.val)
                }
            }
            _ => println!(
                "R{:02}         to 0x{:02X} ({}, {})",
                evt.idx, evt.val, evt.val, evt.val as i8
            ),
        }
    }
}

struct MChange;

impl Observer<ChangeEvent> for MChange {
    fn notify(&self, evt: ChangeEvent) {
        println!(
            "Memory 0x{:02X} to 0x{:02X} ({})",
            evt.idx, evt.val, evt.val
        );
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

    // Check the file exists
    let path = Path::new(&input);
    if path.exists() {
        // Read the file into a string
        match read(path) {
            Ok(program) => {
                let mut machine = Machine::new();

                // Declared outside the if to keep a local reference
                let rc: Rc<dyn Observer<ChangeEvent>> = Rc::new(RChange {
                    verbose: matches.opt_present("v"),
                });
                if matches.opt_present("v") || matches.opt_present("c") {
                    machine.add_reg_observer(Rc::downgrade(&rc));
                }

                let rc: Rc<dyn Observer<ChangeEvent>> = Rc::new(MChange {});
                if matches.opt_present("m") {
                    machine.add_mem_observer(Rc::downgrade(&rc));
                }

                for (i, b) in program.iter().enumerate() {
                    machine.set_mem(i as u8, *b);
                }

                if matches.opt_present("i") {
                    for (i, v) in machine.iter_mem() {
                        println!("0x{:04X}: 0x{:08X} {:10}", i, v, v);
                    }
                }

                loop {
                    match machine.step(None) {
                        Ok(res) => match res {
                            Response::Halt => {
                                println!("stop on halt");
                                break;
                            }
                            Response::Wait => {
                                println!("stop on wait");
                                break;
                            }
                            _ => continue,
                        },
                        Err(res) => match res {
                            Response::UnknownInstruction => {
                                println!("Bad Instruction");
                                break;
                            }
                            Response::BadRegister => {
                                println!("Bad Register");
                                break;
                            }
                            _ => continue,
                        },
                    }
                }

                if matches.opt_present("f") {
                    for (i, v) in machine.iter_mem() {
                        println!("0x{:02X}: 0x{:02X} ({:4}, {:3})", i, v, v as i8, v);
                    }
                }

                // Show the end state of the registers
                if matches.opt_present("r") {
                    for i in 0..4 {
                        if let Ok(v) = machine.reg(i) {
                            println!("R{}: 0x{:02X} ({:4}, {:3})", i, v, v as i8, v);
                        }
                    }
                }
            }
            // Or not...
            Err(e) => println!("Can't read {}: {}", path.display(), e),
        }
    }
}
