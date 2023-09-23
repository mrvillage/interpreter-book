use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Object::*;

        match self {
            Integer(i) => write!(f, "{}", i),
            Boolean(b) => write!(f, "{}", b),
            Null => write!(f, "null"),
        }
    }
}

impl PartialEq<i64> for Object {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Integer(i) if i == other => true,
            _ => false,
        }
    }
}

impl PartialEq<bool> for Object {
    fn eq(&self, other: &bool) -> bool {
        match self {
            Self::Boolean(i) if i == other => true,
            _ => false,
        }
    }
}
