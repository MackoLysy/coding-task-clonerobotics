use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Errors {
    msg: String,
}
impl Errors {
    pub fn new(msg: &str) -> Errors {
        Errors {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for Errors {}
