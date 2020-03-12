use belgium::Input;
use belgium::{Token, Type};

fn main() {
    let test = "
    asect  0x00

    # =================================
    # INSERT YOUR CODE BELOW
    # Executable instructions only
    # No dc or ds pseudo-instructions
    # Do not include a halt instruction: that has been done already (below)
    # ---------------------------------------------------------------------
    
    ldi r0, x
    ld r0, r0
    ldi r1, y
    ld r1, r1
    ldi r2, 0
    ldi r3, 0
    
    if
      tst r1
    is mi
      neg r1
      neg r0
    fi
    
    while
      tst r1
    stays nz
      if
        add r0, r2
      is vs
        ldi r2, -1
        ldi r3, 1
        break
      fi
      dec r1
    wend
    
    ldi r1, ans
    st r1, r2
    ldi r1, oflow
    st r1, r3
    
    # =================================
    # LEAVE THIS PART OF THE FILE ALONE
    # Do not change the next two instructions: they must be the last two
    # instructions executed by your program.
        ldi r0, ans  # Loads the address of your result into r0 for the robot
        halt         # Brings execution to a halt
    
    # =================================
    # DATA GOES BELOW
    # We have set this up for you, but you will need to test your program by
    # compiling and running it several times with different input data values
    # (different unsigned numbers placed at addresses x and y)
    # ---------------------------------------------------------------------
    
    INPUTS>
    x:     dc -15        # replace these with your choice
    y:     dc -12    # of integers for testing
    ENDINPUTS>
    
    ans:   ds 1    # one byte reserved for the (unsigned number) product
    oflow: ds 1
    
    end
    ";

    let mut input = Input::from(test.to_string());

    loop {
        match input.consume() {
            Ok(node) => match *node {
                Type::Eof => break,
                _ => println!("{:?}", *node),
            },
            Err(err) => {
                err.print(Some(&input));
                break;
            }
        }
    }
}
