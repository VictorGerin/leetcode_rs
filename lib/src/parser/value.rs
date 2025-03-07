#[derive(Debug, Clone)]
pub enum Value {
    Int(i128),
    Double(f64),
    Bool(bool),
    Str(String),
    Vec(Vec<Value>),
    None
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(v) => Some(*v as i32),
            Value::Double(v) => Some(*v as i32),
            _ => None
        }
    }
    
    pub fn as_double(&self) -> Option<f64> {
        match self {
            Value::Int(v) => Some(*v as f64),
            Value::Double(v) => Some(*v as f64),
            _ => None
        }
    }

    pub fn as_string<'a>(&'a self) -> Option<&'a str> {
        match self {
            Value::Str(s) => Some(s.as_str()),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None
        }
    }

    pub fn as_long(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v as i64),
            Value::Double(v) => Some(*v as i64),
            _ => None
        }
    }
} 