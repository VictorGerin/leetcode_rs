use std::{collections::HashMap, sync::OnceLock};

pub struct Number {
    pub(crate) value: i32,
}

pub struct NumberIter {
    current: Option<i32>,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

impl Iterator for NumberIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        }
        let current = self.current.unwrap();
        let div = current / 10;
        let n = current % 10;
        self.current = if div == 0 {
            None
        } else {
            Some(div)
        };
        Some(n)
    }
}

impl IntoIterator for Number {
    type Item = i32;
    type IntoIter = NumberIter;
    fn into_iter(self) -> NumberIter {
        NumberIter {
            current: if self.value <= 0 {
                None
            } else {
                Some(self.value)
            }
        }
    }
}

static MAP: OnceLock<HashMap<usize, (&str, &str)>> = OnceLock::new();

pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        Number::from(num)
        .into_iter()
        .enumerate() //create a tuple with the index digit from the least significant to the most significant
        .collect::<Vec<(usize, i32)>>()
        .into_iter()
        .rev() //reverse the order to start from the most significant digit
        .map(|(i, x)| {
            let map = MAP.get_or_init(|| {
                let mut map = HashMap::new();
                map.insert(0, ("I", "V")); //units
                map.insert(1, ("X", "L")); //tens
                map.insert(2, ("C", "D")); //hundreds
                map.insert(3, ("M", ""));  //thousands
                map.insert(4, ("", ""));   //thousands
                map
            });
            let r_num = map.get(&i).unwrap();
            let r_num_next = map.get(&(i + 1));
            match x {
                1..=3 => {
                    format!("{}", r_num.0.repeat(x as usize))
                },
                4 => {
                    format!("{}{}", r_num.0, r_num.1)
                },
                5..=8 => {
                    format!("{}{}", r_num.1, r_num.0.repeat((x - 5) as usize))
                },
                9 => {
                    format!("{}{}", r_num.0, r_num_next.as_ref().unwrap().0)
                },
                0 => {"".to_string()},
                _ => {panic!("Invalid number {}", x);},
            }
        }).fold(String::with_capacity(15), |mut acc, x| {
            acc.push_str(&x);
            acc
        })
    }
} 