pub struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        
        // Generate all possible powers that can be used
        let mut powers = Vec::new();
        let mut i = 1;
        loop {
            let power = (i as i64).pow(x as u32);
            if power > n as i64 {
                break;
            }
            powers.push(power);
            i += 1;
        }
        
        // dp[i] = number of ways to express i as sum of powers
        let mut dp = vec![0i64; (n + 1) as usize];
        dp[0] = 1;
        
        // For each power, update dp
        for &power in &powers {
            let power = power as usize;
            for i in (power..=n as usize).rev() {
                dp[i] = (dp[i] + dp[i - power]) % MOD;
            }
        }
        
        dp[n as usize] as i32
    }
}
