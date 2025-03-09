use crate::data_structures::{ListNode, TreeNode};

use super::{error::ValErr, input, parser_str};

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Int(i128),
    Double(f64),
    Bool(bool),
    Str(String),
    Vec(Vec<Val>),
    None
}

impl Val {

    pub fn as_int(self) -> Result<i32, Self> {
        match self {
            Val::Int(v) => Ok(v as i32),
            Val::Double(v) => Ok(v as i32),
            _ => Err(self)
        }
    }
    
    pub fn as_double(self) -> Result<f64, Self> {
        match self {
            Val::Int(v) => Ok(v as f64),
            Val::Double(v) => Ok(v as f64),
            _ => Err(self)
        }
    }

    pub fn as_string(self) -> Result<String, Self> {
        match self {
            Val::Str(s) => Ok(s),
            _ => Err(self)
        }
    }

    pub fn as_bool(self) -> Result<bool, Self> {
        match self {
            Val::Bool(b) => Ok(b),
            _ => Err(self)
        }
    }

    pub fn as_long(self) -> Result<i64, Self> {
        match self {
            Val::Int(v) => Ok(v as i64),
            Val::Double(v) => Ok(v as i64),
            _ => Err(self)
        }
    }

    pub fn as_vec(self) -> Result<Vec<Val>, Self>
    {
        match self {
            Val::Vec(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn as_vec_int(self) -> Result<Vec<i32>, ValErr>
    {
        self.as_vec()
            .map_err(|x| ValErr::IsNotVec(x))?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, Self>>()
            .map_err(|x| ValErr::NotAllElementsIsIntOnVec(x))
    }

    
    pub fn as_list_node(self) -> Result<ListNode, ValErr>
    {
        let vec = self.as_vec()
            .map_err(|x| ValErr::IsNotVec(x))?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, Self>>()
            .map_err(|x| ValErr::NotAllElementsIsIntOnVec(x))?;

        Ok(vec.into_iter().collect::<ListNode>())
    }

    pub fn as_tree_node(self) -> Result<TreeNode, Self>
    {
        todo!("Implement this method")
    }

}


impl std::str::FromStr for Val
{
    type Err = super::ProcessInputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_str(s)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_int() {
        let v = Val::Int(10);
        assert_eq!(v.as_int(), Ok(10));
        let v = Val::Double(10.0);
        assert_eq!(v.as_int(), Ok(10));
        let v = Val::Str("10".to_string());
        assert_eq!(v.clone().as_int(), Err(v));
    }

    #[test]
    fn test_as_double() {
        let v = Val::Int(10);
        assert_eq!(v.as_double(), Ok(10.0));
        let v = Val::Double(10.0);
        assert_eq!(v.as_double(), Ok(10.0));
        let v = Val::Str("10".to_string());
        assert_eq!(v.clone().as_double(), Err(v));
    }

    #[test]
    fn test_as_string() {
        let v = Val::Str("10".to_string());
        assert_eq!(v.as_string(), Ok("10".to_string()));
        let v = Val::Int(10);
        assert_eq!(v.clone().as_string(), Err(v));
    }

    #[test]
    fn test_as_bool() {
        let v = Val::Bool(true);
        assert_eq!(v.as_bool(), Ok(true));
        let v = Val::Int(10);
        assert_eq!(v.clone().as_bool(), Err(v));
    }

    #[test]
    fn test_as_long() {
        let v = Val::Int(10);
        assert_eq!(v.as_long(), Ok(10));
        let v = Val::Double(10.0);
        assert_eq!(v.as_long(), Ok(10));
        let v = Val::Str("10".to_string());
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
        assert_eq!(v, Err(ValErr::NotAllElementsIsIntOnVec(Val::Str("teste".to_string()))));

        //Not a vector
        let v = super::super::parser_str(r#""20""#)
        .map_err(|_| "opps".to_string())?;
        let v = v.as_vec_int();
        assert_eq!(v, Err(ValErr::IsNotVec(Val::Str("20".to_string()))));

        Ok(())
    }

    #[test]
    fn test_as_list_node() -> Result<(), String> {
        let v: Val = "[10, 20]".parse()
        .map_err(|_| "opps".to_string())?;

        let v = v.as_list_node();
        assert_eq!(v, Ok(ListNode::from_iter(vec![10, 20])));
        
        let v = r#""not a vector""#.parse::<Val>()
        .map_err(|_| "opps".to_string())?;
    
        let v = v.as_list_node();
        assert_eq!(v, Err(ValErr::IsNotVec(Val::Str("not a vector".to_string()))));

        Ok(())
    }
}