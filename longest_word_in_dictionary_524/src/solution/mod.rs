pub struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let s = s.as_bytes();
        dictionary
            .into_iter()
            .filter(|word| {
                let word = word.as_bytes();
                let (mut i, mut j) = (0, 0);

                while i < word.len() && j < s.len() {
                    if word.get(i) == s.get(j) {
                        i += 1;
                    }
                    j += 1;
                    if s.len() - j < word.len() - i {
                        return false;
                    }
                }

                i == word.len()
            })
            .fold(String::new(), |acc, x| {
                if acc.len() < x.len() {
                    x
                } else if acc.len() == x.len() {
                    match acc.cmp(&x) {
                        std::cmp::Ordering::Less => acc,
                        std::cmp::Ordering::Equal => x,
                        std::cmp::Ordering::Greater => x,
                    }
                } else {
                    acc
                }
            })
    }
} 