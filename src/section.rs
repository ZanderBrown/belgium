use crate::node::Node;
use crate::Error;

use std::fmt;

#[derive(Debug)]
pub enum Section {
    Absolute { pos: u8, content: Vec<Node> },
    RSect { name: String, content: Vec<Node> },
    Template { name: String, content: Vec<Node> },
    None,
}

impl Section {
    pub fn absolute(pos: u8) -> Self {
        Self::Absolute {
            pos,
            content: Vec::with_capacity(10),
        }
    }

    pub fn rsect(name: String) -> Self {
        Self::RSect {
            name,
            content: Vec::with_capacity(10),
        }
    }

    pub fn add(&mut self, node: Node) -> Result<(), Error> {
        match self {
            Self::Absolute { content, .. }
            | Self::RSect { content, .. }
            | Self::Template { content, .. } => {
                content.push(node);
                Ok(())
            }
            Self::None => Err(Error::new("Not in a section".to_string(), node.range())),
        }
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absolute{pos, content} => {
                writeln!(f, "asect {}", pos)?;
                for node in content {
                    writeln!(f, "  {}", **node)?;
                }
                Ok(())
            }
            Self::RSect{name, content} => {
                writeln!(f, "rsect {}", name)?;
                for node in content {
                    writeln!(f, "  {}", **node)?;
                }
                Ok(())
            }
            Self::Template{name, content} => {
                writeln!(f, "tplate {}", name)?;
                for node in content {
                    writeln!(f, "  {}", **node)?;
                }
                Ok(())
            }
            Self::None => {
                writeln!(f, "[NONE]")
            }
        }
    }
}