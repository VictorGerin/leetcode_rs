use std::ops::Deref;
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T: Eq + PartialEq + Clone + Display> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>
}

impl<T: Eq + PartialEq + Clone + Display> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn iter(&self) -> ListNodeIterRef<'_, T> {
        ListNodeIterRef { current: Some(self) }
    }

    pub fn iter_mut(&mut self) -> ListNodeIterRefMut<'_, T> {
        ListNodeIterRefMut { current: Some(self) }
    }
}

pub struct ListNodeIterRefMut<'a, T: Eq + PartialEq + Clone + Display> {
    current: Option<&'a mut ListNode<T>>,
}

pub struct ListNodeIterRef<'a, T: Eq + PartialEq + Clone + Display> {
    current: Option<&'a ListNode<T>>,
}

pub struct ListNodeIter<T: Eq + PartialEq + Clone + Display> {
    current: Option<Box<ListNode<T>>>,
}

impl<'a, T: Eq + PartialEq + Clone + Display> Iterator for ListNodeIterRefMut<'a, T> {
    type Item = &'a mut T;

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

impl<'a, T: Eq + PartialEq + Clone + Display> Iterator for ListNodeIterRef<'a, T> {
    type Item = &'a T;

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

impl<T: Eq + PartialEq + Clone + Display> Iterator for ListNodeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match (*self).current.take() {
            None => None,
            Some(c) => {
                self.current = c.next;
                Some(c.val)
            }
        }
    }
}

impl<T: Eq + PartialEq + Clone + Display> IntoIterator for ListNode<T> {
    type Item = T;
    type IntoIter = ListNodeIter<T>;
    
    fn into_iter(self) -> ListNodeIter<T> {
        ListNodeIter { current: Some(Box::new(self)) }
    }
}

impl<T: Eq + PartialEq + Clone + Display> FromIterator<T> for ListNode<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let list = iter
            .into_iter()
            .collect::<Vec<T>>()
            .into_iter()
            .rev()
            .fold(None, |acc: Option<Box<ListNode<T>>>, x| {
                let mut node = ListNode::new(x);
                node.next = acc;
                Some(Box::new(node))
            });

        *list.unwrap()
    }
}

impl<T: Eq + PartialEq + Clone + Display> Display for ListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (index, val) in self.iter().enumerate() {
            if index != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", val)?;
        }
        write!(f, "]")
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