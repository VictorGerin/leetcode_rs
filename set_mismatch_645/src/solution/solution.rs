pub struct Solution;

impl Solution {

    /// Calcula o XOR de todos os números naturais de 1 até n.
    ///
    /// Utiliza uma otimização baseada em um padrão cíclico observado no resultado
    /// do XOR de números consecutivos. O resultado depende apenas do resto da
    /// divisão de n por 4, permitindo calcular em O(1) ao invés de O(n).
    ///
    /// Padrão observado:
    /// - Se n % 4 == 0: resultado = n
    /// - Se n % 4 == 1: resultado = 1
    /// - Se n % 4 == 2: resultado = n + 1
    /// - Se n % 4 == 3: resultado = 0
    fn xor_sum_of_n(n: i32) -> i32 {
        match n % 4 {
            0 => n,
            1 => 1,
            2 => n + 1,
            3 => 0,
            _ => unreachable!(), // n % 4 sempre retorna 0, 1, 2 ou 3
        }
    }
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {

        // Seja "a" o número repetido e "b" o número que está faltando
        let n = nums.len() as i32;

        // Calcula XOR de todos os elementos do array e de todos os inteiros de 1 a n
        // No array: temos 1, 2, 3, ..., a, a, ..., n (onde "a" aparece duas vezes e "b" está faltando)
        // No conjunto natural: temos 1, 2, 3, ..., a, b, ..., n (onde cada número aparece uma vez)
        // Ao fazer XOR de ambos, os números que aparecem uma vez em cada conjunto se cancelam
        // Resta apenas: a ^ b (pois "a" aparece duas vezes no array e "b" não aparece)
        let a_xor_b = nums.iter().fold(Self::xor_sum_of_n(nums.len() as i32), |acc, x| acc ^ *x);

        // Qualquer bit 1 em a_xor_b indica um bit que "a" tem e "b" não tem, ou vice-versa

        // Para isolar o bit menos significativo que está setado em a_xor_b, usamos o truque:
        // x & -x (ou x & (!x + 1)), que funciona porque -x em complemento de dois é !x + 1
        // Isso nos dá uma máscara com apenas um bit setado, que diferencia "a" de "b"
        let diff_bit = a_xor_b & -a_xor_b;
        
        // Separa os números em dois grupos baseado no diff_bit
        // Um grupo contém números com o bit setado, outro contém números sem o bit setado
        // Como "a" e "b" diferem nesse bit, eles vão para grupos diferentes
        // O XOR de cada grupo nos dá "a" ou "b" (mas ainda não sabemos qual é qual)
        let (a, b) = nums.iter().copied().chain(1..n+1)
            .fold((0, 0), |(balde_a, balde_b), x| {

                //Checa se x possui o bit ou não, aqui é o pulo do gato
                if x & diff_bit != 0 {
                    (balde_a ^ x, balde_b)
                } else {
                    (balde_a, balde_b ^ x)
                }
            });

        // Determina qual é o repetido e qual é o faltando
        if nums.contains(&a) {
            vec![a, b]  // [repetido, faltando]
        } else {
            vec![b, a]  // [repetido, faltando]
        }
    }
}
