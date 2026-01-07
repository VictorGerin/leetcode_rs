pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // Ordenar é crucial!
        let mut result = Vec::new();
        let n = nums.len();
        
        for i in 0..n {
            // Pular duplicatas do primeiro elemento
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            let mut left = i + 1;
            let mut right = n - 1;
            
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                
                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    
                    // Pular duplicatas do segundo elemento
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    // Pular duplicatas do terceiro elemento
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1; // Precisamos de um número maior
                } else {
                    right -= 1; // Precisamos de um número menor
                }
            }
        }
        
        result
    }
}

