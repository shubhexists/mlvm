use colored::*;
use std::fmt;

#[derive(Debug)]
pub struct LTS {
    pub version: String,
    pub alias: String,
}

impl fmt::Display for LTS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ({})", self.alias.yellow(), self.version.red())
    }
}
