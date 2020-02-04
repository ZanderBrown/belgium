use std::error;
use std::fmt;

/// Line, Column
#[derive(Debug)]
pub struct Point(usize, usize);

#[derive(Debug)]
pub struct Error {
    message: String,
    point: Option<Point>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.message)?;
        if let Some(point) = &self.point {
            write!(f, " [{}:{}]", point.0, point.1)?;
        }
        Ok(())
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl Error {
    #[must_use]
    pub fn new(message: String, point: Option<Point>) -> Self {
        Self { message, point }
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
    #[must_use]
    pub fn error(&self, msg: String) -> Error {
        Error::new(msg, Some(Point(self.line, self.col)))
    }

    fn read(&mut self, matcher: &dyn Fn(char) -> bool) -> String {
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
    Number(u32),
    Memory(u32),
    Register(u32),
    Label(String),
    Comma,
    Colon,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Command(cmd) => write!(f, "{}", cmd),
            Token::Number(num) => write!(f, "#{}", num),
            Token::Memory(mem) => write!(f, "{}", mem),
            Token::Register(reg) => write!(f, "R{}", reg),
            Token::Label(lbl) => write!(f, "{}", lbl),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
        }
    }
}

impl Iterator for Input {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
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
                let text = self.read(&|c| c.is_alphanumeric());
                if COMMANDS.contains(&text.as_str()) {
                    Some(Ok(Token::Command(text)))
                } else {
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
            } else if ch == ':' {
                self.forward();
                Some(Ok(Token::Colon))
            } else if ch == '#' {
                self.forward();
                let num = self.read(&|c| c.is_numeric());
                match num.parse::<u32>() {
                    Ok(num) => Some(Ok(Token::Number(num))),
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
