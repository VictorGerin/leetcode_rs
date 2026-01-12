pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        
        let mut has_change = false;
        num.to_string().chars().map(|ch| if !has_change && ch == '6' {
            has_change = true;
            '9'
        } else {ch}).collect::<String>().parse::<i32>().unwrap()
    }
}
