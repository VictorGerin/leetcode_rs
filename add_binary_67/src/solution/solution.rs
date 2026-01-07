use std::iter;

pub struct Solution;

impl Solution {
    pub fn add_binary(_a: String, _b: String) -> String {

        let a_iter = _a.chars().rev().map(|c| c.to_digit(10).unwrap()).map(Some).chain(iter::repeat(None));
        let b_iter = _b.chars().rev().map(|c| c.to_digit(10).unwrap()).map(Some).chain(iter::repeat(None));

        let max_len = std::cmp::max(_a.len(), _b.len());
        
        let mut carry = 0;
        let mut result = String::new();
        for (opt_a, opt_b) in a_iter.zip(b_iter).take(max_len) {
            let digit_a = opt_a.unwrap_or(0);
            let digit_b = opt_b.unwrap_or(0);
            let sum = digit_a + digit_b + carry;
            carry = sum / 2;
            result.push((sum % 2).to_string().chars().next().unwrap());
        }

        if carry > 0 {
            result.push('1');
        }
        
        return result.chars().rev().collect();
    }
}

