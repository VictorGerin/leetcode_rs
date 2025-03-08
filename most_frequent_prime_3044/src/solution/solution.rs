use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        
        // Directions: [east, south-east, south, south-west, west, north-west, north, north-east]
        let directions = [
            (0, 1), (1, 1), (1, 0), (1, -1),
            (0, -1), (-1, -1), (-1, 0), (-1, 1)
        ];
        
        let mut prime_freq = HashMap::new();
        let mut prime_cache = HashMap::new();
        
        // Helper function to check if a number is prime using Sieve of Eratosthenes
        let mut is_prime = |n: i32| -> bool {
            // Check cache first
            if let Some(&result) = prime_cache.get(&n) {
                return result;
            }
            
            if n <= 1 {
                return false;
            }
            if n <= 3 {
                prime_cache.insert(n, true);
                return true;
            }
            if n % 2 == 0 {
                prime_cache.insert(n, false);
                return false;
            }

            // Use Sieve of Eratosthenes to find primes up to sqrt(n)
            let sqrt_n = (n as f64).sqrt() as i32;
            let mut sieve = vec![true; (sqrt_n + 1) as usize];

            // Mark non-prime numbers
            for i in 2..=sqrt_n {
                if sieve[i as usize] {
                    let mut j = i * i;
                    while j <= sqrt_n {
                        sieve[j as usize] = false;
                        j += i;
                    }
                }
            }
            
            // Cache all prime numbers found
            for i in 2..=sqrt_n {
                prime_cache.insert(i, sieve[i as usize]);
            }
            
            // Check if n is divisible by any prime up to sqrt(n)
            let result = (2..=sqrt_n)
                .filter(|&x| sieve[x as usize])
                .all(|x| n % x != 0);
                
            prime_cache.insert(n, result);
            result
        };
        
        // For each cell in the matrix
        for i in 0..m {
            for j in 0..n {
                // Try each direction
                for &(di, dj) in &directions {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    let mut num = mat[i][j];
                    
                    // Keep moving in the same direction until we go out of bounds
                    while x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        // Only consider numbers greater than 10
                        if num > 10 && is_prime(num) {
                            *prime_freq.entry(num).or_insert(0) += 1;
                        }
                        
                        x += di;
                        y += dj;
                        
                        // Add next digit if still in bounds
                        if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                            num = num * 10 + mat[x as usize][y as usize];
                        }
                    }
                }
            }
        }
                
        // Find the most frequent prime number
        prime_freq.into_iter()
            .max_by_key(|&(num, freq)| (freq, num))
            .map(|(num, _)| num)
            .unwrap_or(-1)
    }
}
