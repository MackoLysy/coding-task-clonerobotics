use std::error::Error;

use super::IParser;
use log::*;
use regex::Regex;

pub struct Parser {
    buff: Vec<u8>,
    parretn: Regex,
}

const MAX_SIZE: usize = 512;

impl Parser {
    pub fn new() -> Self {
        Self {
            buff: vec![],
            parretn: Regex::new(r"\$(d+).*\n|\$\d+(?:,\w+)*\n").unwrap(),
        }
    }
}
impl IParser for Parser {
    fn parse(&mut self, data: u8) -> Option<String> {
        self.buff.push(data);
        let result = self.parse_vec(self.buff.to_vec());
        if result.is_some() {
            return result;
        }
        if self.buff.len() > MAX_SIZE {
            self.buff.clear();
        }
        None
    }
    fn parse_vec(&mut self, data: Vec<u8>) -> Option<String> {
        let validate = String::from_utf8(data);
        if validate.is_ok() {
            for mat in self.parretn.captures_iter(validate.unwrap().as_str()) {
                trace!("matching: {:#?}", mat);
                return Some(mat[0].to_string());
            }
        } else {
            error!("error validate buffer");
            self.buff.clear();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        let mut parser = Parser::new();
        let result = parser.parse('2' as u8);
        assert_eq!(result.is_none(), true);
    }
    #[test]
    fn simple_vec_test_proper_msg() {
        let text = "$1\n";
        let mut parser = Parser::new();
        let result = parser.parse_vec(text.as_bytes().to_vec());
        assert_eq!(result.is_some(), true);
    }
    #[test]
    fn simple_vec_test_invalid_msg() {
        let text = "$1asdsad\n";
        let mut parser = Parser::new();
        let result = parser.parse_vec(text.as_bytes().to_vec());
        assert_eq!(result.is_some(), false);
    }
    #[test]
    fn simple_vec_valid_multiple_message() {
        let text = "$1,v1,v2\n";
        let mut parser = Parser::new();
        let result = parser.parse_vec(text.as_bytes().to_vec());
        assert_eq!(result.is_some(), true);
    }
}

// \$([0-9]).*\n
