use std::ops::Deref;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn iter(&self) -> ListNodeIterRef {
        ListNodeIterRef { current: Some(self) }
    }

    pub fn iter_mut(&mut self) -> ListNodeIterRefMut {
        ListNodeIterRefMut { current: Some(self) }
    }
}

pub struct ListNodeIterRefMut<'a> {
    current: Option<&'a mut ListNode>,
}

pub struct ListNodeIterRef<'a> {
    current: Option<&'a ListNode>,
}

pub struct ListNodeIter {
    current: Option<Box<ListNode>>,
}

impl<'a> Iterator for ListNodeIterRefMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(c) => {
                self.current = match c.next.as_mut() {
                    None => None,
                    Some(n) => Some(n.as_mut())
                };
                Some(&mut c.val)
            }
        }
    }
}

impl<'a> Iterator for ListNodeIterRef<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        match current {
            None => None,
            Some(c) => {
                self.current = match c.next.as_ref() {
                    None => None,
                    Some(n) => Some(n.deref())
                };
                Some(&c.val)
            }
        }
    }
}

impl Iterator for ListNodeIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match (*self).current.take() {
            None => None,
            Some(c) => {
                self.current = c.next;
                Some(c.val)
            }
        }
    }
}

impl IntoIterator for ListNode {
    type Item = i32;
    type IntoIter = ListNodeIter;
    
    fn into_iter(self) -> ListNodeIter {
        ListNodeIter { current: Some(Box::new(self)) }
    }
}

impl FromIterator<i32> for ListNode {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let list = iter
            .into_iter()
            .collect::<Vec<i32>>()
            .into_iter()
            .rev()
            .fold(None, |acc: Option<Box<ListNode>>, x| {
                let mut node = ListNode::new(x);
                node.next = acc;
                Some(Box::new(node))
            });

        *list.unwrap()
    }
}

impl ToString for ListNode {
    fn to_string(&self) -> String {
        let mut result = "[".to_string();
        for (index, val) in self.iter().enumerate() {
            if index != 0 {
                result.push_str(",");
            }
            result.push_str(&val.to_string());
        }
        result.push_str("]");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node() {
        let list = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.to_string(), "[1,2,3,4,5]");
        
        let list = *list.next.unwrap();
        assert_eq!(list.to_string(), "[2,3,4,5]");
        
        let list = *list.next.unwrap();
        assert_eq!(list.to_string(), "[3,4,5]");
        
        let list = *list.next.unwrap();
        assert_eq!(list.to_string(), "[4,5]");
        
        let list = *list.next.unwrap();
        assert_eq!(list.to_string(), "[5]");
    }
}