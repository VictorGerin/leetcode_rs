pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits;
        let n = result.len();
        
        // Iterate from right to left
        for i in (0..n).rev() {
            if result[i] < 9 {
                result[i] += 1;
                return result;
            }
            result[i] = 0;
        }
        
        // If we get here, all digits were 9
        // Need to add a new digit 1 at the beginning
        let mut new_result = vec![1];
        new_result.extend(result);
        new_result
    }
}

