pub struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_sum = nums[0];  // Keep track of the maximum sum found
        let mut current_sum = nums[0];  // Keep track of current ascending sequence sum

        // Iterate through the array starting from second element
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                // If current number is greater than previous, add to current sequence
                current_sum += nums[i];
            } else {
                // If not strictly increasing, start a new sequence
                current_sum = nums[i];
            }
            // Update maximum sum if current sequence sum is larger
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}
