use crate::data_structures::{ListNode, TreeNode};

use super::error::ValueError;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn as_vec(self) -> Result<Vec<Value>, Self>
    {
        match self {
            Value::Vec(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn as_vec_int(self) -> Result<Vec<i32>, ValueError>
    {
        self.as_vec()
            .map_err(|x| ValueError::IsNotVec(x))?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, Self>>()
            .map_err(|x| ValueError::NotAllElementsIsIntOnVec(x))
    }

    
    pub fn as_list_node(self) -> Result<ListNode, ValueError>
    {
        let vec = self.as_vec()
            .map_err(|x| ValueError::IsNotVec(x))?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, Self>>()
            .map_err(|x| ValueError::NotAllElementsIsIntOnVec(x))?;

        Ok(vec.into_iter().collect::<ListNode>())
    }

    pub fn as_tree_node(self) -> Result<TreeNode, Self>
    {
        todo!("Implement this method")
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_int() {
        let v = Value::Int(10);
        assert_eq!(v.as_int(), Ok(10));
        let v = Value::Double(10.0);
        assert_eq!(v.as_int(), Ok(10));
        let v = Value::Str("10".to_string());
        assert_eq!(v.clone().as_int(), Err(v));
    }

    #[test]
    fn test_as_double() {
        let v = Value::Int(10);
        assert_eq!(v.as_double(), Ok(10.0));
        let v = Value::Double(10.0);
        assert_eq!(v.as_double(), Ok(10.0));
        let v = Value::Str("10".to_string());
        assert_eq!(v.clone().as_double(), Err(v));
    }

    #[test]
    fn test_as_string() {
        let v = Value::Str("10".to_string());
        assert_eq!(v.as_string(), Ok("10".to_string()));
        let v = Value::Int(10);
        assert_eq!(v.clone().as_string(), Err(v));
    }

    #[test]
    fn test_as_bool() {
        let v = Value::Bool(true);
        assert_eq!(v.as_bool(), Ok(true));
        let v = Value::Int(10);
        assert_eq!(v.clone().as_bool(), Err(v));
    }

    #[test]
    fn test_as_long() {
        let v = Value::Int(10);
        assert_eq!(v.as_long(), Ok(10));
        let v = Value::Double(10.0);
        assert_eq!(v.as_long(), Ok(10));
        let v = Value::Str("10".to_string());
        assert_eq!(v.clone().as_long(), Err(v));
    }

    #[test]
    fn test_as_vec() -> Result<(), String> {

        //ok test
        let v = super::super::parser_str("[10, 20]")
        .map_err(|_| "opps".to_string())?;

        let v = v.as_vec_int();
        assert_eq!(v, Ok(vec![10, 20]));

        //is a vector but can't convert to int all elements
        let v = super::super::parser_str("[10, \"teste\"]")
        .map_err(|x| x.to_string())?;

        let v = v.as_vec_int();
        assert_eq!(v, Err(ValueError::NotAllElementsIsIntOnVec(Value::Str("teste".to_string()))));

        //Not a vector
        let v = super::super::parser_str(r#""20""#)
        .map_err(|_| "opps".to_string())?;
        let v = v.as_vec_int();
        assert_eq!(v, Err(ValueError::IsNotVec(Value::Str("20".to_string()))));

        Ok(())
    }

    #[test]
    fn test_as_list_node() -> Result<(), String> {
        let v = super::super::parser_str("[10, 20]")
        .map_err(|_| "opps".to_string())?;

        let v = v.as_list_node();
        assert_eq!(v, Ok(ListNode::from_iter(vec![10, 20])));
        
        let v = super::super::parser_str(r#""not a vector""#)
        .map_err(|_| "opps".to_string())?;
    
        let v = v.as_list_node();
        assert_eq!(v, Err(ValueError::IsNotVec(Value::Str("not a vector".to_string()))));

        Ok(())
    }
}