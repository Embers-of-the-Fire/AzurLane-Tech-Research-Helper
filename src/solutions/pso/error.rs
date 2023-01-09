use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PsoError {
    pub message: String,
}

impl PsoError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for PsoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PsoError: {}", self.message)
    }
}