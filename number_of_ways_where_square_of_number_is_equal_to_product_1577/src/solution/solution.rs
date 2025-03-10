pub struct Solution;


///
/// Given two arrays of integers nums1 and nums2, return the number of triplets formed (type 1 and type 2) under the following rules:
/// Type 1: Triplet (i, j, k) if nums1[i]2 == nums2[j] * nums2[k] where 0 <= i < nums1.length and 0 <= j < k < nums2.length.
/// 
/// Type 2: Triplet (i, j, k) if nums2[i]2 == nums1[j] * nums1[k] where 0 <= i < nums2.length and 0 <= j < k < nums1.length.
/// 
/// 
 




impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

        let nums1_squered: Vec<i64> = nums1.iter().map(|x| *x as i64 * *x as i64 ).collect();
        let nums2_squered: Vec<i64> = nums2.iter().map(|x| *x as i64 * *x as i64 ).collect();

        let mut count = 0;
        
        for j in 0..nums2.len() {
            for k in j+1..nums2.len() {
                let product = nums2[j] as i64 * nums2[k]  as i64;
                
                for i in 0..nums1_squered.len() {
                    if nums1_squered[i] == product {
                        count += 1;
                    }
                }

            }
        }
        
        for j in 0..nums1.len() {
            for k in j+1..nums1.len() {
                let product = nums1[j] as i64 * nums1[k] as i64;
                
                for i in 0..nums2_squered.len() {
                    if nums2_squered[i] == product {
                        count += 1;
                    }
                }

            }
        }

        count
    }
}