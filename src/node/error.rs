use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeError {
    pub message: String,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeError: {}", self.message)
    }
}

impl NodeError {
    pub fn new(message: String) -> Self {
        NodeError { message }
    }
}

