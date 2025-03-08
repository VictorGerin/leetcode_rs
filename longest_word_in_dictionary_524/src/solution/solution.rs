pub struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut result = String::new();
        
        for word in dictionary {
            if Self::is_subsequence(&word, &s) {
                if word.len() > result.len() || (word.len() == result.len() && word < result) {
                    result = word;
                }
            }
        }
        
        result
    }
    
    fn is_subsequence(word: &str, s: &str) -> bool {
        let mut it = s.chars();
        word.chars().all(|c| it.by_ref().any(|x| x == c))
    }
}
