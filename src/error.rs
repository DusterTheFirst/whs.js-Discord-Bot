use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum EnumParseError {
    VarientDoesNotExist(String),
}

impl Display for EnumParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::VarientDoesNotExist(s) => write!(f, "Varient does not exist: {}", s),
        }
    }
}
