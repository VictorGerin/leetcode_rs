pub struct Solution;

impl Solution {
    /// Retorna o número mínimo de elementos a remover para que o array restante seja balanceado
    /// (max ≤ k * min).
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        // sort_unstable é geralmente mais rápido que sort para tipos primitivos
        // pois não preserva a ordem de elementos iguais (que não importa aqui)
        nums.sort_unstable(); 

        let n = nums.len();
        let mut max_window_size = 0;

        for (i, &min_val) in nums.iter().enumerate() {
            let limit = min_val as i64 * k as i64;
            
            // OTIMIZAÇÃO AQUI:
            // Em vez de um loop 'for j', usamos busca binária.
            // partition_point retorna o índice do primeiro elemento ONDE A CONDIÇÃO É FALSA.
            // Ou seja, o primeiro elemento que é MAIOR que o limite.
            // Isso custa O(log n) em vez de O(n).
            let j = nums.partition_point(|&x| x as i64 <= limit);
            
            // O range válido é de i até j (exclusivo).
            // O tamanho dessa janela é j - i.
            // Queremos maximizar quantos números FICAM, para minimizar quantos SAEM.
            if j > i {
                let current_window = j - i;
                max_window_size = max_window_size.max(current_window);
            }
        }

        (n - max_window_size) as i32
    }
}
