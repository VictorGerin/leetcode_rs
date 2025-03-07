pub struct Solution {}

impl Solution {
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(left.len() + right.len());
        
        let (mut i, mut j) = (0, 0);

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);
        result
    }

    fn sort_array_slice(nums: &[i32]) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums.to_vec();
        }

        let mid = nums.len() / 2;
        let left = Solution::sort_array_slice(&nums[..mid]);
        let right = Solution::sort_array_slice(&nums[mid..]);

        Solution::merge(&left, &right)
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Solution::sort_array_slice(&nums)
    }
} 