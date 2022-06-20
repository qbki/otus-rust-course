use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DscError(String);

impl DscError {
    pub fn new(description: String) -> Self {
        DscError(description)
    }
}

impl Error for DscError {}

impl fmt::Display for DscError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
