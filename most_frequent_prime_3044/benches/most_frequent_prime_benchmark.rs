use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import the Solution struct directly from the crate's root
mod solution {
    include!("../src/solution/solution.rs");
}
use solution::Solution;

fn create_large_matrix(size: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            // Create larger numbers that could form interesting primes
            // Using modulo 90 + 10 to get numbers between 10 and 99
            matrix[i][j] = ((i as i32 * 3 + j as i32 * 7) % 90 + 10) as i32;
        }
    }
    
    // Add some specific patterns that could form larger prime numbers
    for i in 0..size {
        if i < size - 2 {
            // Add some prime numbers in diagonal pattern
            matrix[i][i] = if i % 2 == 0 { 73 } else { 97 };
            // Add some prime numbers in reverse diagonal
            matrix[i][size - 1 - i] = if i % 2 == 0 { 89 } else { 83 };
        }
    }

    // Add some specific larger numbers that could form interesting primes when combined
    for i in 0..size.min(10) {
        for j in 0..size.min(10) {
            if (i + j) % 3 == 0 {
                matrix[i][j] = 71 + ((i as i32 * j as i32) % 20);  // Numbers between 71 and 90
            }
        }
    }

    matrix
}

fn bench_most_frequent_prime(c: &mut Criterion) {
    // Small matrix test case (2x2)
    let small_matrix = vec![
        vec![1, 2],
        vec![3, 4],
    ];
    
    // Medium matrix test case (3x4)
    let medium_matrix = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 2, 3],
    ];
    
    // Large matrix test case (4x4)
    let large_matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 2, 3, 4],
        vec![5, 6, 7, 8],
    ];
    
    // Complex matrix with many primes (5x5)
    let complex_matrix = vec![
        vec![7, 3, 2, 1, 9],
        vec![1, 7, 3, 2, 1],
        vec![2, 1, 7, 3, 2],
        vec![3, 2, 1, 7, 3],
        vec![9, 3, 2, 1, 7],
    ];

    // Create a large 15x15 matrix
    let huge_matrix = create_large_matrix(15);

    let mut group = c.benchmark_group("most_frequent_prime");
    
    group.bench_function("small_matrix_2x2", |b| {
        b.iter(|| Solution::most_frequent_prime(black_box(small_matrix.clone())))
    });
    
    group.bench_function("medium_matrix_3x4", |b| {
        b.iter(|| Solution::most_frequent_prime(black_box(medium_matrix.clone())))
    });
    
    group.bench_function("large_matrix_4x4", |b| {
        b.iter(|| Solution::most_frequent_prime(black_box(large_matrix.clone())))
    });
    
    group.bench_function("complex_matrix_5x5", |b| {
        b.iter(|| Solution::most_frequent_prime(black_box(complex_matrix.clone())))
    });
    
    group.bench_function("huge_matrix_15x15", |b| {
        b.iter(|| Solution::most_frequent_prime(black_box(huge_matrix.clone())))
    });
    
    group.finish();
}

criterion_group!(benches, bench_most_frequent_prime);
criterion_main!(benches); 