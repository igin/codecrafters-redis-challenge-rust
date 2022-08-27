use std::fmt;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct RESPError {
    pub message: String,
}

pub enum RESPValue {
    String(String),
    Error(RESPError),
    Array(Vec<RESPValue>),
    NullString(),
}

impl fmt::Debug for RESPValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPValue::String(content) => f.debug_tuple("String").field(&content).finish(),
            RESPValue::Error(content) => f.debug_tuple("Error").field(&content.message).finish(),
            RESPValue::Array(content) => f.debug_list().entries(content).finish(),
            _ => todo!(),
        }
    }
}

pub struct ExpiringValue {
    pub value: String,
    pub expiry: Option<SystemTime>,
}

pub struct State {
    pub map: HashMap<String, ExpiringValue>,
}