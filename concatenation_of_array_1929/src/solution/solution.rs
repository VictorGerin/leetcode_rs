// AI_GENERATED_CODE_START
// Descrição: Estrutura Solution com método get_concatenation para concatenar array com ele mesmo
// Gerado por: Cursor AI
// Versão: Rust edition 2021
// AI_GENERATED_CODE_END
pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().chain(nums.iter()).copied().collect()
    }
}

