pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize; // Cast necessário pois n vem como i32
        
        // 1. Cria os slices (primeira e segunda metade)
        // 2. Zipa eles para criar pares (&x, &y)
        nums[..n].iter().zip(&nums[n..]) 
            // 3. FlatMap (SelectMany): Transforma o par em um array [x, y] e achata
            .flat_map(|(&x, &y)| [x, y]) 
            // 4. Coleta o resultado em um novo Vec
            .collect()
    }
}

