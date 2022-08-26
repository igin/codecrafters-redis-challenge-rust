use std::fmt;

pub struct Command<'a> {
    command_type: &'a str,
}

pub struct RESPError {
    pub message: String,
}

pub enum RESPValue {
    String(String),
    Error(RESPError),
    Array(Vec<RESPValue>),
}

impl fmt::Debug for RESPValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPValue::String(content) => f.debug_tuple("SimpleString").field(&content).finish(),
            RESPValue::Error(content) => f.debug_tuple("Error").field(&content.message).finish(),
            RESPValue::Array(content) => f.debug_list().entries(content).finish(),
        }
    }
}
