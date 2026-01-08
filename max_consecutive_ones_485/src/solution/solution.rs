pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        
        let mut max_count = 0;
        let mut current_count = 0;
        for x in nums {
            if x == 1 {
                current_count += 1;
            } else {
                max_count = max_count.max(current_count);
                current_count = 0;
            }
        }
        max_count.max(current_count)
    }
}
