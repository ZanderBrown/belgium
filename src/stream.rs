use std::fmt;

pub struct Syntax(String, usize, usize);

impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error: {} [{}:{}]", self.0, self.1, self.2)
    }
}

pub struct Input {
    input: String,
    line: usize,
    col: usize,
    pos: usize,
}

impl Input {
    fn forward(&mut self) {
        // If further chars are avalible
        if let Some(ch) = self.input.chars().nth(self.pos) {
            // Advance the position
            self.pos += 1;
            // If the charecter is a newline
            if ch == '\n' {
                // Record the new line position
                self.line += 1;
                // Reset the column position
                self.col = 0;
            } else {
                // Still on the same line, advance the column
                self.col += 1;
            }
        }
    }

    /// Fetch the next character without consuming it
    fn peek(&self) -> Option<char> {
        // Return the character
        self.input.chars().nth(self.pos)
    }

    /// Generate a syntax error with msg at the current position
    pub fn error(&self, msg: String) -> Syntax {
        Syntax(msg, self.line, self.col)
    }

    fn read(&mut self, matcher: &Fn(char) -> bool) -> String {
        // The string we will return
        let mut s = String::new();
        // Read whilst charecters are still available
        while let Some(ch) = self.peek() {
            // If ch satisfies predicate
            if matcher(ch) {
                // Consume ch appending it to the result
                s.push(ch);
                self.forward();
            } else {
                // Break out of the while loop
                break;
            }
        }
        s
    }
}

/// Allow casting String to Input
impl From<String> for Input {
    fn from(input: String) -> Self {
        Self {
            input,
            line: 1,
            col: 0,
            pos: 0,
        }
    }
}

const COMMANDS: [&str; 18] = [
    "LDR", "STR", "ADD", "SUB", "MOV", "CMP", "B", "BEQ", "BNE", "BGT", "BLT", "AND", "ORR", "EOR",
    "MVN", "LSL", "LSR", "HALT",
];

pub enum Token {
    Command(String),
    Number(usize),
    Memory(usize),
    Register(usize),
    Label(String),
    Comma,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Command(cmd) => write!(f, "{}", cmd),
            Token::Number(num) => write!(f, "#{}", num),
            Token::Memory(mem) => write!(f, "{}", mem),
            Token::Register(reg) => write!(f, "R{}", reg),
            Token::Label(lbl) => write!(f, "{}", lbl),
            Token::Comma => write!(f, ","),
        }
    }
}

impl Iterator for Input {
    type Item = Result<Token, Syntax>;

    fn next(&mut self) -> Option<Result<Token, Syntax>> {
        self.read(&|ch| ch.is_whitespace());
        if let Some(ch) = self.peek() {
            if ch == 'R' {
                self.forward();
                let num = self.read(&|c| c.is_numeric());
                match num.parse() {
                    Ok(num) => Some(Ok(Token::Register(num))),
                    Err(_) => Some(Err(self.error(format!("Bad number {}", num)))),
                }
            } else if ch.is_alphabetic() {
                let text = self.read(&|c| c.is_alphabetic());
                if COMMANDS.contains(&text.as_str()) {
                    Some(Ok(Token::Command(text)))
                } else {
                    if self.peek() == Some(':') {
                        self.next();
                    }
                    Some(Ok(Token::Label(text)))
                }
            } else if ch.is_numeric() {
                let num = self.read(&|c| c.is_numeric());
                match num.parse() {
                    Ok(num) => Some(Ok(Token::Memory(num))),
                    Err(_) => Some(Err(self.error(format!("Bad number {}", num)))),
                }
            } else if ch == ',' {
                self.forward();
                Some(Ok(Token::Comma))
            } else if ch == '#' {
                self.forward();
                let neg = if self.peek() == Some('-') {
                    self.forward();
                    true
                } else {
                    false
                };
                let num = self.read(&|c| c.is_numeric());
                match num.parse::<usize>() {
                    Ok(num) => Some(Ok(Token::Number(if neg {
                        (!num) + 1
                    } else {
                        num
                    }))),
                    Err(_) => Some(Err(self.error(format!("Bad number {}", num)))),
                }
            } else {
                self.forward();
                Some(Err(self.error(format!("Unknown {}", ch))))
            }
        } else {
            None
        }
    }
}
