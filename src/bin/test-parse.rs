use belgium::{Input, Parser};

fn main() {
    let test = "asect  0x00

ldi r0, a
ld r0, r1
ldi r0, b
ld r0, r2
add r1, r2
rsect lolx
ldi r0, res
st r0, r2

ldi r0, res  # Loads the address of your result into r0 for the robot
halt         # Brings execution to a halt

INPUTS>
a:  dc  19    # replace 19 by your choice of integer for testing
b:  dc  -128    # replace -2 by your choice of integer for testing
ENDINPUTS>

res:	ds	1		# one byte reserved for the result
rsect idk
end";

    let input = Input::from(test.to_string());
    let mut parser = Parser::new(input);

    if let Err(err) = parser.node() {
        err.print(Some(&*parser));
    } else {
        for sect in parser.sections() {
            println!("{}", sect.borrow());
        }
    }
}
