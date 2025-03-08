use crate::data_structures::{ListNode, TreeNode};

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
    pub fn as_int(self) -> Result<i32, Self> {
        match self {
            Value::Int(v) => Ok(v as i32),
            Value::Double(v) => Ok(v as i32),
            _ => Err(self)
        }
    }
    
    pub fn as_double(self) -> Result<f64, Self> {
        match self {
            Value::Int(v) => Ok(v as f64),
            Value::Double(v) => Ok(v as f64),
            _ => Err(self)
        }
    }

    pub fn as_string(self) -> Result<String, Self> {
        match self {
            Value::Str(s) => Ok(s),
            _ => Err(self)
        }
    }

    pub fn as_bool(self) -> Result<bool, Self> {
        match self {
            Value::Bool(b) => Ok(b),
            _ => Err(self)
        }
    }

    pub fn as_long(self) -> Result<i64, Self> {
        match self {
            Value::Int(v) => Ok(v as i64),
            Value::Double(v) => Ok(v as i64),
            _ => Err(self)
        }
    }

    pub fn as_vec<F, T, U>(self, f: F) -> Result<U, Self>
    where
        F: Fn(Value) -> T,
        U: std::iter::FromIterator<T>
    {
        match self {
            Value::Vec(v) => Ok(v.into_iter().map(f).collect::<U>()),
            _ => Err(self),
        }
    }

    
    pub fn as_list_node(self) -> Result<ListNode, Self>
    {
        todo!("Implement this method")
    }

    pub fn as_tree_node(self) -> Result<TreeNode, Self>
    {
        todo!("Implement this method")
    }
} 