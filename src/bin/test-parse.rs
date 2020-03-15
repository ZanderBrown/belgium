use belgium::{Input, NodeType};

fn main() {
    let test = "asect  0x00

ldi r0, a
ld r0, r1
ldi r0, b
ld r0, r2
add r1, r2
ldi r0, res
st r0, r2

ldi r0, res  # Loads the address of your result into r0 for the robot
halt         # Brings execution to a halt

INPUTS>
a:  dc  19    # replace 19 by your choice of integer for testing
b:  dc  -128    # replace -2 by your choice of integer for testing
ENDINPUTS>

res:	ds	1		# one byte reserved for the result
end";

    let mut input = Input::from(test.to_string());

    loop {
        match input.node() {
            Ok(node) => match *node {
                NodeType::End => {
                    println!("{:?}", *node);
                    break;
                }
                _ => println!("{:?}", *node),
            },
            Err(err) => {
                err.print(Some(&input));
                break;
            }
        }
    }
}
