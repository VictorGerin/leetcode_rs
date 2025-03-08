pub struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        let mid = nums.len() / 2;
        let left = Self::sort_array(nums[..mid].to_vec());
        let right = Self::sort_array(nums[mid..].to_vec());

        Self::merge(left, right)
    }

    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut left_iter = left.into_iter();
        let mut right_iter = right.into_iter();
        let mut left_peek = left_iter.next();
        let mut right_peek = right_iter.next();

        while left_peek.is_some() && right_peek.is_some() {
            let left_val = left_peek.unwrap();
            let right_val = right_peek.unwrap();

            if left_val <= right_val {
                result.push(left_val);
                left_peek = left_iter.next();
            } else {
                result.push(right_val);
                right_peek = right_iter.next();
            }
        }

        while let Some(val) = left_peek {
            result.push(val);
            left_peek = left_iter.next();
        }

        while let Some(val) = right_peek {
            result.push(val);
            right_peek = right_iter.next();
        }

        result.extend(left_iter);
        result.extend(right_iter);
        result
    }
}
